use std::sync::Arc;

use anyhow::anyhow;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use ulid::Ulid;

use crate::utils::update::Update::{Empty, Unchanged, Value};

use super::{inputs, model::Task};

/// Get an individual `Task` by id
pub async fn get(client: Arc<Client>, table_name: &str, id: &str) -> anyhow::Result<Option<Task>> {
    let results = client
        .get_item()
        .table_name(table_name)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await?;

    if let Some(item) = results.item {
        Ok(Some(item.try_into()?))
    } else {
        Ok(None)
    }
}

/// Create a `Task` with the given input
pub async fn create(
    client: Arc<Client>,
    table_name: &str,
    input: &inputs::Create,
) -> anyhow::Result<Task> {
    let id_av = AttributeValue::S(Ulid::new().to_string());

    let created_at_av = AttributeValue::S(chrono::Utc::now().to_string());
    let updated_at_av = created_at_av.clone();

    let title_av = AttributeValue::S(input.title.clone());

    let description_av = match &input.description {
        Some(description) => AttributeValue::S(description.clone()),
        None => AttributeValue::Null(true),
    };

    let result = client
        .put_item()
        .table_name(table_name)
        .item("id", id_av)
        .item("created_at", created_at_av)
        .item("updated_at", updated_at_av)
        .item("title", title_av)
        .item("description", description_av)
        .send()
        .await?;

    let task = result
        .attributes
        .ok_or(anyhow!("No attributes returned"))?
        .try_into()?;

    Ok(task)
}

/// Update an existing `Task` by id
pub async fn update(
    client: Arc<Client>,
    table_name: &str,
    id: &str,
    input: &inputs::Update,
) -> anyhow::Result<Task> {
    let existing = get(client.clone(), table_name, id).await?;

    if existing.is_none() {
        return Err(anyhow!("Task not found"));
    }
    let existing = existing.unwrap();

    let id_av = AttributeValue::S(id.to_string());
    let created_at_av = AttributeValue::S(existing.created_at.to_string());
    let updated_at_av = AttributeValue::S(chrono::Utc::now().to_string());

    let title_av = match &input.title {
        Unchanged | Empty => AttributeValue::S(existing.title.clone()),
        Value(value) => AttributeValue::S(value.to_string()),
    };

    let description_av = match &input.description {
        Unchanged => {
            if let Some(description) = &existing.description {
                AttributeValue::S(description.clone())
            } else {
                AttributeValue::Null(true)
            }
        }
        Empty => AttributeValue::Null(true),
        Value(value) => AttributeValue::S(value.to_string()),
    };

    let result = client
        .put_item()
        .table_name(table_name)
        .item("id", id_av)
        .item("created_at", created_at_av)
        .item("updated_at", updated_at_av)
        .item("title", title_av)
        .item("description", description_av)
        .send()
        .await?;

    let task = result
        .attributes
        .ok_or(anyhow!("No attributes returned"))?
        .try_into()?;

    Ok(task)
}

/// Delete an existing `Task`
pub async fn delete(client: Arc<Client>, table_name: &str, id: &str) -> anyhow::Result<()> {
    let _ = client
        .delete_item()
        .table_name(table_name)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await?;

    Ok(())
}
