use axum::extract::FromRef;
use nakago::Tag;
use serde::{Deserialize, Serialize};

/// Tag(Config)
pub const CONFIG: Tag<Config> = Tag::new("app::Config");

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// Database config
    pub database: nakago_sea_orm::Config,

    /// Optional Dynamo config
    pub dynamo: Option<Dynamo>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dynamo {
    /// The table name to use with DynamoDB
    pub table_name: String,
}

impl nakago::Config for Config {}
