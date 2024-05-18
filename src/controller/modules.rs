use std::{io::Cursor, sync::Arc};

use log::info;
use rocket::{get, http::Status, response::Responder, serde::json::Json, Request, Response, State};
use thiserror::Error;
use uuid::Uuid;

use crate::{model::Module, service::Registry};

#[derive(Debug, Error)]
pub enum GetModuleError {
    #[error("Failed to parse UUID: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),
}

impl<'r> Responder<'r, 'static> for GetModuleError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            GetModuleError::UuidError(_) => Status::BadRequest,
            GetModuleError::ModuleNotFound(_) => Status::NotFound,
        };

        Response::build()
            .status(status)
            .sized_body(self.to_string().len(), Cursor::new(self.to_string()))
            .ok()
    }
}

#[get("/modules")]
pub async fn get_modules(registry: &State<Arc<Registry>>) -> Json<Vec<Module>> {
    let modules = registry.get_modules().await;
    Json(modules)
}

#[get("/modules/<uuid>")]
pub async fn get_module(
    registry: &State<Arc<Registry>>,
    uuid: String,
) -> Result<Json<Module>, GetModuleError> {
    info!("Getting module with UUID: {}", uuid);
    let uuid = match Uuid::parse_str(&uuid) {
        Ok(uuid) => uuid,
        Err(err) => return Err(GetModuleError::UuidError(err)),
    };

    let module = match registry.get_module_by_uuid(uuid).await {
        Some(module) => module,
        None => return Err(GetModuleError::ModuleNotFound(uuid.to_string())),
    };

    Ok(Json(module))
}
