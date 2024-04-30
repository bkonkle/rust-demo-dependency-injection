use pico_args::Arguments;

pub const HELP: &str = "\
Usage: rust-demo-dependency-injection [OPTIONS]

Options:
  -h, --help           Print help (this message)
  -d, --data-store     The data store to use: 'postgres' or 'dynamodb', defaults to 'postgres'
  -a, --address        The address to bind to, defaults to '127.0.0.1'
  -p, --port           The port to bind to, defaults to '3000'
";

#[derive(Debug)]
pub struct Args {
    pub data_store: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
}

impl Args {
    pub fn parse() -> anyhow::Result<Option<Self>> {
        let mut pargs = Arguments::from_env();

        // Help has a higher priority and should be handled separately.
        if pargs.contains(["-h", "--help"]) {
            return Ok(None);
        }

        let args = Args {
            data_store: pargs.opt_value_from_str(["-d", "--data-store"])?,
            address: pargs.opt_value_from_str(["-a", "--address"])?,
            port: pargs.opt_value_from_str(["-p", "--port"])?,
        };

        Ok(Some(args))
    }
}

#[derive(Debug)]
pub enum DataStore {
    Postgres,
    DynamoDB,
}

impl TryFrom<String> for DataStore {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "postgres" => Ok(DataStore::Postgres),
            "dynamodb" => Ok(DataStore::DynamoDB),
            _ => Err(anyhow::anyhow!("Invalid data store: {}", value)),
        }
    }
}

impl From<DataStore> for String {
    fn from(data_store: DataStore) -> Self {
        match data_store {
            DataStore::Postgres => "postgres".to_string(),
            DataStore::DynamoDB => "dynamodb".to_string(),
        }
    }
}
