use axum::extract::FromRef;
use nakago_sea_orm::DatabasePool;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, FromRef)]
pub struct ConfigForDatabase {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// Database config
    pub db: nakago_sea_orm::Config,
}

impl Default for ConfigForDatabase {
    fn default() -> Self {
        Self {
            http: default_http_config(),
            db: nakago_sea_orm::Config {
                url: "postgres://localhost:5432/rust_demo".to_string(),
                debug: false,
                pool: DatabasePool::default(),
            },
        }
    }
}

impl nakago::Config for ConfigForDatabase {}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, FromRef)]
pub struct ConfigForDynamo {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// DynamoDB config
    pub dynamo: Dynamo,
}

impl Default for ConfigForDynamo {
    fn default() -> Self {
        Self {
            http: default_http_config(),
            dynamo: Dynamo {
                tasks_table_name: "tasks".to_string(),
            },
        }
    }
}

impl nakago::Config for ConfigForDynamo {}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dynamo {
    /// The table name to use for Tasks with DynamoDB
    pub tasks_table_name: String,
}

pub fn default_http_config() -> nakago_axum::Config {
    nakago_axum::Config {
        address: "127.0.0.1".to_string(),
        port: 3000,
    }
}
