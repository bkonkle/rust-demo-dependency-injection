#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ConfigForDB {
    /// HTTP config
    pub http: Http,

    /// Optional Database config
    pub db: Database,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ConfigForDynamo {
    /// HTTP config
    pub http: Http,

    /// Optional Dynamo config
    pub dynamo: Dynamo,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Http {
    /// The port to bind to
    pub port: u16,

    /// The IP address to bind to, such as 0.0.0.0 or 127.0.0.1
    pub address: String,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Database {
    /// The database URL to use with Postgres
    pub url: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            url: "postgres://localhost:5432/rust_demo".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
