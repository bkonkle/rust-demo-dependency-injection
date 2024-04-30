use axum::{
    async_trait,
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use nakago::{hooks, inject, Hook, Inject};
use nakago_axum::{routes, AxumApplication};

use crate::{
    config::{ConfigForDatabase, ConfigForDynamo},
    tasks,
};

pub async fn database_app() -> inject::Result<AxumApplication<ConfigForDatabase>> {
    let mut app = AxumApplication::default();

    app.provide_type::<nakago_sea_orm::DatabaseConnection>(nakago_sea_orm::connection::Provide::<
        ConfigForDatabase,
    >::new(None))
        .await?;

    Ok(app)
}

pub async fn dynamo_app() -> inject::Result<AxumApplication<ConfigForDynamo>> {
    let mut app = AxumApplication::default();

    Ok(app)
}

/// Load dependencies for all HTTP handlers
#[derive(Default)]
pub struct LoadHttp {}

#[async_trait]
impl Hook for LoadHttp {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide_type::<tasks::http::Controller>(tasks::http::Provide::default())
            .await?;

        Ok(())
    }
}

/// Init all HTTP handlers
#[derive(Default)]
pub struct InitHttp {}

#[async_trait]
impl Hook for InitHttp {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let tasks = i.get_type::<tasks::http::Controller>().await?;

        i.handle(routes::Init::new(
            Router::new()
                .route(
                    "/tasks",
                    post(|input| async move { tasks.create(input).await }),
                )
                .route(
                    "/tasks/:id",
                    get(|id| async move { tasks.get(id).await })
                        .patch(|id, input| async move { tasks.update(id, input).await })
                        .delete(|id| async move { tasks.delete(id).await }),
                ),
        ))
        .await?;

        Ok(())
    }
}
