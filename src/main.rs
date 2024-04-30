//! # A demo project showing dependency injection approaches
#![forbid(unsafe_code)]

use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sea_orm::DatabaseConnection;

use crate::{
    args::{Args, DataStore},
    config::Config,
};

mod args;
mod config;
mod tasks;
mod utils;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct DatabaseAppState {
    config: Config,
    db: Arc<DatabaseConnection>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct DynamoAppState {
    config: Config,
    client: Arc<Client>,
    task_table_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let output = Args::parse()?;

    if output.is_none() {
        print!("{}", args::HELP);

        return Ok(());
    }

    let args = output.unwrap();

    let address = args.address.unwrap_or("127.0.0.1".to_string());
    let port = args.port.unwrap_or(3000);

    let data_store: DataStore = args
        .data_store
        .map_or(Ok(DataStore::Postgres), |v| v.try_into())?;

    let app = match data_store {
        DataStore::Postgres => {
            let config = Config {
                http: config::Http { port, address },
                db: Some(config::Database {
                    url: "postgres://localhost:5432/rust_demo".to_string(),
                }),
                dynamo: None,
            };

            let db = Arc::new(sea_orm::Database::connect(config.db.clone().unwrap().url).await?);

            let state = DatabaseAppState { db, config };

            Router::new()
                .route("/tasks", post(tasks_create_in_db))
                .route(
                    "/tasks/:id",
                    get(tasks_get_from_db)
                        .patch(tasks_update_in_db)
                        .delete(tasks_delete_in_db),
                )
                .with_state(state)
        }
        DataStore::DynamoDB => {
            let config = Config {
                http: config::Http { port, address },
                db: None,
                dynamo: Some(config::Dynamo {
                    table_name: "tasks".to_string(),
                }),
            };

            let client = Arc::new(Client::from_conf(
                aws_sdk_dynamodb::Config::builder().build(),
            ));

            let state = DynamoAppState {
                client,
                task_table_name: config.dynamo.clone().unwrap().table_name.clone(),
                config,
            };

            Router::new()
                .route("/tasks", post(tasks_create_in_dynamo))
                .route(
                    "/tasks/:id",
                    get(tasks_get_from_dynamo)
                        .patch(tasks_update_in_dynamo)
                        .delete(tasks_delete_in_dynamo),
                )
                .with_state(state)
        }
    };

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn tasks_get_from_db(
    Path(id): Path<String>,
    State(state): State<DatabaseAppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let maybe_task = match tasks::service::get(state.db.clone(), &id).await {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    if let Some(task) = maybe_task {
        return Ok(Json(task));
    }

    Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
}

async fn tasks_get_from_dynamo(
    Path(id): Path<String>,
    State(state): State<DynamoAppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let maybe_task =
        match tasks::dynamo_service::get(state.client.clone(), &state.task_table_name, &id).await {
            Ok(result) => result,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        };

    if let Some(task) = maybe_task {
        return Ok(Json(task));
    }

    Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
}

async fn tasks_create_in_db(
    State(state): State<DatabaseAppState>,
    Json(input): Json<tasks::inputs::Create>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let task = match tasks::service::create(state.db.clone(), &input).await {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(task))
}

async fn tasks_create_in_dynamo(
    State(state): State<DynamoAppState>,
    Json(input): Json<tasks::inputs::Create>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let task =
        match tasks::dynamo_service::create(state.client.clone(), &state.task_table_name, &input)
            .await
        {
            Ok(result) => result,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        };

    Ok(Json(task))
}

async fn tasks_update_in_db(
    Path(id): Path<String>,
    State(state): State<DatabaseAppState>,
    Json(input): Json<tasks::inputs::Update>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let task = match tasks::service::update(state.db.clone(), &id, &input).await {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(task))
}

async fn tasks_update_in_dynamo(
    Path(id): Path<String>,
    State(state): State<DynamoAppState>,
    Json(input): Json<tasks::inputs::Update>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let task = match tasks::dynamo_service::update(
        state.client.clone(),
        &state.task_table_name,
        &id,
        &input,
    )
    .await
    {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(task))
}

async fn tasks_delete_in_db(
    Path(id): Path<String>,
    State(state): State<DatabaseAppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = tasks::service::delete(state.db.clone(), &id).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }

    Ok(())
}

async fn tasks_delete_in_dynamo(
    Path(id): Path<String>,
    State(state): State<DynamoAppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) =
        tasks::dynamo_service::delete(state.client.clone(), &state.task_table_name, &id).await
    {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }

    Ok(())
}
