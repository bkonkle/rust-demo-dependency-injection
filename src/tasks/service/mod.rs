use async_trait::async_trait;

use super::{inputs, model::Task};

#[cfg(test)]
use mockall::automock;

/// A Postgres implementation
pub mod database;

/// A DynamoDB implementation
pub mod dynamo;

/// A TasksService applies business logic to a dynamic TasksRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Service: Sync + Send {
    /// Get an individual `Task` by id
    async fn get(&self, id: &str) -> anyhow::Result<Option<Task>>;

    /// Create a `Task` with the given input
    async fn create(&self, input: &inputs::Create) -> anyhow::Result<Task>;

    /// Update an existing `Task` by id
    async fn update(&self, id: &str, input: &inputs::Update) -> anyhow::Result<Task>;

    /// Delete an existing `Task`
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}
