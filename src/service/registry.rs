use std::time::{Duration, Instant};

use log::info;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::Module;

#[derive(Debug, Error)]
pub enum RefreshError {
    #[error("Failed to fetch module registry: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Failed to parse module registry JSON: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct Registry {
    url: String,
    modules: RwLock<Vec<Module>>,
    cache_ttl: Duration,
    cache_last_refresh: RwLock<Option<Instant>>,
}

impl Registry {
    pub fn new(url: String, cache_ttl: Duration) -> Self {
        Registry {
            url,
            modules: RwLock::new(Vec::new()),
            cache_ttl,
            cache_last_refresh: RwLock::new(None),
        }
    }

    async fn needs_refresh(&self) -> bool {
        let cache_last_refresh = self.cache_last_refresh.read().await;
        match *cache_last_refresh {
            Some(last_refresh) => {
                let elapsed = last_refresh.elapsed();
                elapsed > self.cache_ttl
            }
            None => true,
        }
    }

    async fn refresh(&self) -> Result<(), RefreshError> {
        let now = Instant::now();
        let mut cache_last_refresh = self.cache_last_refresh.write().await;
        *cache_last_refresh = Some(now);
        drop(cache_last_refresh);

        let response = match reqwest::get(self.url.clone()).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(err) => return Err(RefreshError::ReqwestError(err)),
            },
            Err(err) => return Err(RefreshError::ReqwestError(err)),
        };

        let parsed_modules = match serde_json::from_str::<Vec<Module>>(&response) {
            Ok(modules) => modules,
            Err(err) => return Err(RefreshError::SerdeJsonError(err)),
        };

        let mut modules = self.modules.write().await;
        *modules = parsed_modules;
        Ok(())
    }

    async fn logging_refresh_if_needed(&self) {
        if self.needs_refresh().await {
            info!("Cache TTL reached, refreshing module registry");

            let result = self.refresh().await;
            if let Err(err) = result {
                log::error!("Failed to refresh module registry: {}\nWill use existing cache until next refresh.", err);
                return;
            }

            let modules_len = self.modules.read().await.len();
            let module_correct_noun = match modules_len {
                1 => "module",
                _ => "modules",
            };

            info!("Loaded {} {} into cache", modules_len, module_correct_noun);
        }
    }

    pub async fn get_modules(&self) -> Vec<Module> {
        self.logging_refresh_if_needed().await;
        self.modules.read().await.clone()
    }

    pub async fn get_module_by_uuid(&self, uuid: Uuid) -> Option<Module> {
        self.logging_refresh_if_needed().await;
        self.modules
            .read()
            .await
            .iter()
            .find(|module| module.uuid == uuid)
            .cloned()
    }
}
