use std::{fmt::Display, time::Duration};

use ::log::info;
use service::Registry;
use thiserror::Error;

pub mod controller;
pub mod log;
pub mod model;
pub mod service;

pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}

#[derive(Debug, Error)]
#[error("Failed to start Nexus")]
pub enum NexusError {
    #[error("Logger is not set up")]
    LoggerNotSetUp,

    #[error("Failed to start Rocket: {0}")]
    RocketError(#[from] rocket::Error),
}

#[derive(Debug)]
pub struct NexusArgs {
    pub registry_url: String,
}

impl NexusArgs {
    pub fn new(registry_url: String) -> Self {
        NexusArgs { registry_url }
    }
}

impl Display for NexusArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.registry_url)
    }
}

pub struct Nexus {
    pub registry: Registry,
}

impl Nexus {
    pub fn new(nexus_args: NexusArgs) -> Self {
        Nexus {
            registry: Registry::new(nexus_args.registry_url, Duration::from_secs(60)),
        }
    }

    pub async fn start(&self) -> Result<(), NexusError> {
        if !log::is_set_up() {
            eprintln!("Logger is not set up");
            return Err(NexusError::LoggerNotSetUp);
        }

        info!("Starting Nexus");

        let rocket = rocket::build().mount("/", controller::routes());

        let result = rocket.launch().await;
        if let Err(err) = result {
            return Err(NexusError::RocketError(err));
        }

        Ok(())
    }
}
