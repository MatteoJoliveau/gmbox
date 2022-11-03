use anyhow::Context as _;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};

const DB_FILENAME: &str = concat!(env!("CARGO_PKG_NAME"), ".db");

pub async fn init(dir_path: Option<PathBuf>) -> anyhow::Result<SqlitePool> {
    let path = dir_path
        .map(|path| path.join(DB_FILENAME))
        .unwrap_or_else(|| PathBuf::from_str(":memory:").unwrap());
    let url = format!("sqlite:{}", path.display());
    
    tracing::info!("Connecting to {url}");
    let options = SqliteConnectOptions::from_str(url.as_str())
        .context("failed to parse SQLite connection string")?
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true);
    SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .context("failed to connect to SQLite database")
}

pub async fn migrate(pool: &SqlitePool, migrations_dir: &Path) -> anyhow::Result<()> {
    let migrator = Migrator::new(migrations_dir)
    .await
    .context("failed to resolve migration source")?;
    
    tracing::debug!("running migrations");
    migrator
        .run(pool)
        .await
        .context("failed to run database migrations")
}
