use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use sqlx::{FromRow, SqlitePool};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod command;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct EntityTemplate {
    id: String,
    slug: String,
    name: String,
    schema: serde_json::Value,
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("templates")
        .invoke_handler(tauri::generate_handler![
            command::list,
            command::find,
            command::create,
            command::update,
        ])
        .build()
}
