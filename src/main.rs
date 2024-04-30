//! # A demo project showing dependency injection approaches
#![forbid(unsafe_code)]

use crate::{
    args::{Args, DataStore},
    config::default_http_config,
};

mod args;
mod config;
mod init;
mod tasks;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let output = Args::parse()?;

    if output.is_none() {
        print!("{}", args::HELP);

        return Ok(());
    }

    let args = output.unwrap();

    let mut http = default_http_config();

    if let Some(address) = args.address {
        http.address = address;
    }

    if let Some(port) = args.port {
        http.port = port;
    }

    let data_store: DataStore = args
        .data_store
        .map_or(Ok(DataStore::Postgres), |v| v.try_into())?;

    let app = match data_store {
        DataStore::Postgres => init::database_app().await?,
        DataStore::DynamoDB => init::dynamo_app().await?,
    };
    let (server, addr) = app.run(None).await?;

    println!("listening on {}", addr);

    server.await?;

    Ok(())
}

// TODO: Move to a structure where both arms of the config are populated regardless of the data
// store chosen at runtime. The config for the data store not in use can just be ignored. I'll
// probably need a flag of some kind in the config itself to indicate what datastore mode is active
// at runtime.
