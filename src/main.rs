use std::{
    env::{self, VarError},
    num::ParseIntError,
    time::Duration,
};

use ::log::{error, LevelFilter, SetLoggerError};
use nexus::{log, Nexus, NexusArgs, NexusError::LoggerNotSetUp};
use thiserror::Error;

#[rocket::main]
async fn main() {
    if let Err(err) = init_logger() {
        eprintln!("Failed to set up logger: {}", err);
        return;
    }

    let nexus_args = match read_nexus_args() {
        Ok(nexus_args) => nexus_args,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let nexus = Nexus::new(nexus_args);
    let result = nexus.start().await;
    if let Err(err) = result {
        match err {
            LoggerNotSetUp => unreachable!("Logger should already be set up"),
            _ => error!("{}", err),
        }
    }
}

fn init_logger() -> Result<(), SetLoggerError> {
    let min_log_level = match nexus::is_debug() {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    log::set_up(min_log_level)
}

#[derive(Debug, Error)]
enum ReadNexusArgsError {
    #[error("Could not parse {0}: {1}")]
    ParseIntError(String, ParseIntError),

    #[error("Could not read {0}: {1}")]
    VarError(String, VarError),
}

fn read_nexus_args() -> Result<NexusArgs, ReadNexusArgsError> {
    let port = env::var("NEXUS_PORT")
        .map_err(|err| ReadNexusArgsError::VarError("NEXUS_PORT".to_string(), err))?
        .parse()
        .map_err(|err| ReadNexusArgsError::ParseIntError("NEXUS_PORT".to_string(), err))?;

    let registry_url = env::var("NEXUS_REGISTRY_URL")
        .map_err(|err| ReadNexusArgsError::VarError("NEXUS_REGISTRY_URL".to_string(), err))?;

    let registry_cache_ttl = env::var("NEXUS_REGISTRY_CACHE_TTL")
        .map_err(|err| ReadNexusArgsError::VarError("NEXUS_REGISTRY_CACHE_TTL".to_string(), err))?
        .parse()
        .map_err(|err| {
            ReadNexusArgsError::ParseIntError("NEXUS_REGISTRY_CACHE_TTL".to_string(), err)
        })?;

    let registry_cache_ttl = Duration::from_secs(registry_cache_ttl);

    Ok(NexusArgs::new(port, registry_url, registry_cache_ttl))
}
