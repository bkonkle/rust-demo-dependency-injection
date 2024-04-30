use std::{collections::HashMap, convert::TryFrom};

use anyhow::anyhow;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use fake::Dummy;

/// The Task  Model
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[cfg_attr(test, derive(Dummy))]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    /// The Task id
    #[sea_orm(primary_key, column_type = "Text")]
    pub id: String,

    /// The date the Task was created
    pub created_at: DateTime,

    /// The date the Task was last updated
    pub updated_at: DateTime,

    /// The Task title
    #[sea_orm(column_type = "Text")]
    pub title: String,

    /// An optional Task description
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
}

/// The name for a Sea ORM model must be "Model", so this provides a convenient alias
pub type Task = Model;

/// Show entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            title: String::default(),
            description: Option::default(),
        }
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for Model {
    type Error = anyhow::Error;

    fn try_from(item: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id = item
            .get("id")
            .ok_or(anyhow!("Unable to find id property"))?
            .as_s()
            .map_err(|_err| anyhow!("Unable to convert id to string"))?
            .to_string();

        let created_at = item
            .get("created_at")
            .ok_or(anyhow!("Unable to find created_at property"))?
            .as_s()
            .map_err(|_err| anyhow!("Unable to convert created_at to String"))?
            .to_string()
            .parse()
            .map_err(|_err| anyhow!("Unable to parse created_at to NaiveDateTime"))?;

        let updated_at = item
            .get("updated_at")
            .ok_or(anyhow!("Unable to find updated_at property"))?
            .as_s()
            .map_err(|_err| anyhow!("Unable to convert updated_at to String"))?
            .to_string()
            .parse()
            .map_err(|_err| anyhow!("Unable to parse updated_at to NaiveDateTime"))?;

        let title = item
            .get("title")
            .ok_or(anyhow!("Unable to find title property"))?
            .as_s()
            .map_err(|_err| anyhow!("Unable to convert title to string"))?
            .to_string();

        let description = if let Some(desc) = item.get("description").map(|v| v.as_s()) {
            Some(
                desc.map_err(|_err| anyhow!("Unable to parse description to String"))?
                    .to_string(),
            )
        } else {
            None
        };

        Ok(Self {
            id,
            created_at,
            updated_at,
            title,
            description,
        })
    }
}
