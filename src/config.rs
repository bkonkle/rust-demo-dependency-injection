use axum::extract::FromRef;
use nakago::Tag;
use nakago_sea_orm::DatabasePool;
use serde::{Deserialize, Serialize};

/// Tag(Config)
pub const CONFIG: Tag<Config> = Tag::new("app::Config");

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, Default, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// Data store config
    pub data_store: DataStore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataStore {
    Postgres(nakago_sea_orm::Config),
    Dynamo(Dynamo),
}

impl Default for DataStore {
    fn default() -> Self {
        Self::Postgres(default_db_config())
    }
}

pub fn default_http_config() -> nakago_axum::Config {
    nakago_axum::Config {
        address: "127.0.0.1".to_string(),
        port: 3000,
    }
}

pub fn default_db_config() -> nakago_sea_orm::Config {
    nakago_sea_orm::Config {
        url: "postgres://localhost:5432/rust_demo".to_string(),
        debug: false,
        pool: DatabasePool::default(),
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dynamo {
    /// The table name to use for Tasks with DynamoDB
    pub tasks_table_name: String,
}

impl Default for Dynamo {
    fn default() -> Self {
        Self {
            tasks_table_name: "tasks".to_string(),
        }
    }
}

impl nakago::Config for Config {}
