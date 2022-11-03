#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use anyhow::Context as _;
use tauri::{Config, Manager};

mod db;
mod logger;
mod menu;
mod slug;
mod template;

pub struct Dirs {
    data_dir: Option<PathBuf>,
}
async fn init_folders(config: impl AsRef<Config>) -> anyhow::Result<Dirs> {
    let config = config.as_ref();
    let identifier = config.tauri.bundle.identifier.as_str();
    if let Some(app_dir) = tauri::api::path::app_dir(config) {
        tokio::fs::create_dir_all(app_dir)
            .await
            .context("failed to create app directory")?;
    }

    let data_dir = tauri::api::path::data_dir().map(|path| path.join(identifier));
    if let Some(ref data_dir) = data_dir {
        tokio::fs::create_dir_all(data_dir)
            .await
            .context("failed to create data dir")?;
    }

    Ok(Dirs { data_dir })
}

fn main() {
    if let Err(ref err) = tauri::Builder::default()
        .menu(menu::new())
        .on_menu_event(menu::on_menu_event)
        .plugin(template::init())
        .setup(|app| {
            logger::init(app.path_resolver().log_dir());
            let handle = app.handle();
            tauri::async_runtime::block_on(async move {
                let dirs = init_folders(handle.config()).await?;
                let pool = db::init(dirs.data_dir)
                    .await
                    .context("failed to initialize database")?;

                let migrations_dir = handle
                    .path_resolver()
                    .resolve_resource("migrations")
                    .context("migrations directory not found amongst application resources")?;
                db::migrate(&pool, migrations_dir.as_path()).await?;
                handle.manage(pool);
                anyhow::Ok(())
            })?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .context("error while running application")
    {
        tracing::error!("{err:?}");
        std::process::exit(1);
    }
}
