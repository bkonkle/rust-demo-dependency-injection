use pico_args::Arguments;

pub(crate) const HELP: &str = "\
Usage: rust-demo-dependency-injection [OPTIONS]

Options:
  -h, --help           Print help (this message)
  -d, --data-store     The data store to use: 'postgres' or 'dynamodb', defaults to 'postgres'
";

#[derive(Debug)]
pub(crate) struct Args {
    pub(crate) data_store: Option<String>,
}

impl Args {
    pub(crate) fn parse() -> anyhow::Result<Option<Self>> {
        let mut pargs = Arguments::from_env();

        // Help has a higher priority and should be handled separately.
        if pargs.contains(["-h", "--help"]) {
            return Ok(None);
        }

        let args = Args {
            data_store: pargs.opt_value_from_str(["-d", "--data-store"])?,
        };

        Ok(Some(args))
    }
}

#[derive(Debug)]
pub(crate) enum DataStore {
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
