use std::path::{PathBuf, Path};

use tracing_subscriber::fmt::writer::{MakeWriterExt, BoxMakeWriter};
use tracing_subscriber::EnvFilter;

fn writer(logs_dir: Option<&Path>) -> BoxMakeWriter {
    match logs_dir {
        Some(logs_dir) => {
            let logfile = tracing_appender::rolling::hourly(logs_dir, env!("CARGO_PKG_NAME"));
            BoxMakeWriter::new(std::io::stdout.and(logfile))
        },
        None => BoxMakeWriter::new(std::io::stdout),
    }
}

pub fn init(logs_dir: Option<PathBuf>) {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_from_env("GMBOX_LOG"))
        .or_else(|_| EnvFilter::try_new("gmbox=info"))
        .unwrap();
    
    tracing_subscriber::fmt()
        .with_writer(writer(logs_dir.as_ref().map(PathBuf::as_path)))
        .with_env_filter(filter_layer)
        .with_target(true)
        .init();
    
    if let Some(logs_dir) = logs_dir {
        tracing::info!("logs path: {}", logs_dir.display());
    }
}
