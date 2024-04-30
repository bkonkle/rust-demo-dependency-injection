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

use crate::{
    args::{Args, DataStore},
    config::Config,
};

mod args;
mod config;
mod init;
mod tasks;
mod utils;

#[allow(dead_code)]
#[derive(Clone)]
struct AppState {
    config: Config,
    tasks_service: Arc<dyn tasks::Service>,
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

    let state = match data_store {
        DataStore::Postgres => {
            let config = Config {
                http: config::Http { port, address },
                db: Some(config::Database {
                    url: "postgres://localhost:5432/rust_demo".to_string(),
                }),
                dynamo: None,
            };

            let db = Arc::new(sea_orm::Database::connect(config.db.clone().unwrap().url).await?);

            let tasks_service = Arc::new(tasks::service::database::Service::new(db));

            AppState {
                config,
                tasks_service,
            }
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

            let tasks_service = Arc::new(tasks::service::dynamo::Service::new(
                client,
                "tasks".to_string(),
            ));

            AppState {
                config,
                tasks_service,
            }
        }
    };

    let app = Router::new()
        .route("/tasks", post(tasks_create))
        .route(
            "/tasks/:id",
            get(tasks_get).patch(tasks_update).delete(tasks_delete),
        )
        .with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn tasks_get(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let maybe_task = match state.tasks_service.get(&id).await {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    if let Some(task) = maybe_task {
        return Ok(Json(task));
    }

    Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
}

async fn tasks_create(
    State(state): State<AppState>,
    Json(input): Json<tasks::inputs::Create>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let task = match state.tasks_service.create(&input).await {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(task))
}

async fn tasks_update(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<tasks::inputs::Update>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let task = match state.tasks_service.update(&id, &input).await {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(task))
}

async fn tasks_delete(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = state.tasks_service.delete(&id).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }

    Ok(())
}
