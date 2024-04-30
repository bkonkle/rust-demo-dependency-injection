#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Config {
    /// HTTP config
    pub http: Http,

    /// Optional Database config
    pub db: Option<Database>,

    /// Optional Dynamo config
    pub dynamo: Option<Dynamo>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Http {
    /// The port to bind to
    pub port: u16,

    /// The IP address to bind to, such as 0.0.0.0 or 127.0.0.1
    pub address: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Database {
    /// The database URL to use with Postgres
    pub url: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Dynamo {
    /// The table name to use with DynamoDB
    pub table_name: String,
}
