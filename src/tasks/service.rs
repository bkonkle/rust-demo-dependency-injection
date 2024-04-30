use std::sync::Arc;

use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, Set};

use crate::utils::Update::{Empty, Unchanged, Value};

use super::{
    inputs,
    model::{self, Task},
};

/// Get an individual `Task` by id
pub async fn get(db: Arc<DatabaseConnection>, id: &str) -> anyhow::Result<Option<Task>> {
    let query = model::Entity::find_by_id(id.to_string());

    let task = query.one(&*db).await?;

    Ok(task)
}

/// Create a `Task` with the given input
pub async fn create(db: Arc<DatabaseConnection>, input: &inputs::Create) -> anyhow::Result<Task> {
    let task = model::ActiveModel {
        title: Set(input.title.clone()),
        description: Set(input.description.clone()),
        ..Default::default()
    }
    .insert(&*db)
    .await?;

    Ok(task)
}

/// Update an existing `Task` by id
pub async fn update(
    db: Arc<DatabaseConnection>,
    id: &str,
    input: &inputs::Update,
) -> anyhow::Result<Task> {
    let query = model::Entity::find_by_id(id.to_owned());

    // Retrieve the existing Show
    let show = query
        .one(&*db)
        .await?
        .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

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

    let updated: Task = show.update(&*db).await?;

    Ok(updated)
}

/// Delete an existing `Task`
pub async fn delete(db: Arc<DatabaseConnection>, id: &str) -> anyhow::Result<()> {
    let show = model::Entity::find_by_id(id.to_owned())
        .one(&*db)
        .await?
        .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

    let _result = show.delete(&*db).await?;

    Ok(())
}
