use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use derive_new::new;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, Set};

use crate::{
    tasks::model::{self, Task},
    utils::Update::{Empty, Unchanged, Value},
};

use super::inputs;

#[derive(new)]
/// A version of the TasksService backed by a Postgres database
pub struct Service {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl super::Service for Service {
    /// Get an individual `Task` by id
    async fn get(&self, id: &str) -> anyhow::Result<Option<Task>> {
        let query = model::Entity::find_by_id(id.to_string());

        let task = query.one(&*self.db).await?;

        Ok(task)
    }

    /// Create a `Task` with the given input
    async fn create(&self, input: &inputs::Create) -> anyhow::Result<Task> {
        let task = model::ActiveModel {
            title: Set(input.title.clone()),
            description: Set(input.description.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(task)
    }

    /// Update an existing `Task` by id
    async fn update(&self, id: &str, input: &inputs::Update) -> anyhow::Result<Task> {
        let query = model::Entity::find_by_id(id.to_owned());

        // Retrieve the existing Task
        let show = query
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Task with id: {}", id))?;

        let mut show: model::ActiveModel = show.into();

        match &input.title {
            Unchanged | Empty => (),
            Value(value) => show.title = Set(value.clone()),
        };

        match &input.description {
            Unchanged => (),
            Empty => show.description = Set(None),
            Value(value) => show.description = Set(Some(value.clone())),
        }

        let updated: Task = show.update(&*self.db).await?;

        Ok(updated)
    }

    /// Delete an existing `Task`
    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let show = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Task with id: {}", id))?;

        let _result = show.delete(&*self.db).await?;

        Ok(())
    }
}
