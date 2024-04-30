use std::sync::Arc;

use axum::{async_trait, extract::Path, http::StatusCode, response::IntoResponse, Json};
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;

use super::{inputs, Service};

/// The Task entity HTTP controller
pub struct Controller {
    service: Arc<Box<dyn Service>>,
}

impl Controller {
    pub async fn get(
        &self,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, impl IntoResponse> {
        let maybe_task = match self.service.get(&id).await {
            Ok(result) => result,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        };

        if let Some(task) = maybe_task {
            return Ok(Json(task));
        }

        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }

    pub async fn create(
        &self,
        Json(input): Json<inputs::Create>,
    ) -> Result<impl IntoResponse, impl IntoResponse> {
        let task = match self.service.create(&input).await {
            Ok(result) => result,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        };

        Ok(Json(task))
    }

    pub async fn update(
        &self,
        Path(id): Path<String>,
        Json(input): Json<inputs::Update>,
    ) -> Result<impl IntoResponse, impl IntoResponse> {
        let task = match self.service.update(&id, &input).await {
            Ok(result) => result,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        };

        Ok(Json(task))
    }

    pub async fn delete(
        &self,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, impl IntoResponse> {
        if let Err(e) = self.service.delete(&id).await {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }

        Ok(())
    }
}

/// Injection provider
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Controller> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Controller>> {
        let service = i.get_type::<Box<dyn Service>>().await?;

        Ok(Arc::new(Controller { service }))
    }
}
