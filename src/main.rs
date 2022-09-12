use anyhow::{Context, Result};
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};
use vc_processors::{
    builtin::{processors::BuiltinProcessor, tasks::AddPieces},
    core::ext::run_consumer,
};

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env()
                .context("env filter")?,
        )
        .init();

    info!("start add_pieces consumer");
    run_consumer::<AddPieces, BuiltinProcessor>()
}
