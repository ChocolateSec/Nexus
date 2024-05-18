use std::{fmt::Display, net::IpAddr, sync::Arc, time::Duration};

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
pub enum NexusError {
    #[error("Logger is not set up")]
    LoggerNotSetUp,

    #[error("Failed to start Rocket: {0}")]
    RocketError(#[from] rocket::Error),
}

#[derive(Debug)]
pub struct NexusArgs {
    pub address: IpAddr,
    pub port: u16,
    pub registry_url: String,
    pub registry_cache_ttl: Duration,
}

impl NexusArgs {
    pub fn new(
        address: IpAddr,
        port: u16,
        registry_url: String,
        registry_cache_ttl: Duration,
    ) -> Self {
        NexusArgs {
            address,
            port,
            registry_url,
            registry_cache_ttl,
        }
    }
}

impl Display for NexusArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NexusArgs {{ port: {}, registry_url: {}, registry_cache_ttl: {:?} }}",
            self.port, self.registry_url, self.registry_cache_ttl
        )
    }
}

pub struct Nexus {
    address: IpAddr,
    port: u16,
    pub registry: Arc<Registry>,
}

impl Nexus {
    pub fn new(nexus_args: NexusArgs) -> Self {
        Nexus {
            address: nexus_args.address,
            port: nexus_args.port,
            registry: Arc::new(Registry::new(
                nexus_args.registry_url,
                nexus_args.registry_cache_ttl,
            )),
        }
    }

    pub async fn start(&self) -> Result<(), NexusError> {
        if !log::is_set_up() {
            return Err(NexusError::LoggerNotSetUp);
        }

        info!("Starting Nexus");

        let config = rocket::Config {
            address: self.address,
            port: self.port,
            ..Default::default()
        };

        let rocket = rocket::build()
            .configure(&config)
            .manage(Arc::clone(&self.registry))
            .mount("/", controller::routes());

        let result = rocket.launch().await;
        if let Err(err) = result {
            return Err(NexusError::RocketError(err));
        }

        Ok(())
    }
}
