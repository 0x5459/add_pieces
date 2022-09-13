use anyhow::{Context, Result};
use crossterm::tty::IsTty;
use tracing::info;
use tracing_subscriber::{
    filter::{self, FilterExt},
    fmt::{layer, time::OffsetTime},
    prelude::*,
    registry,
};

use time::format_description::well_known::Rfc3339;
use vc_processors::{
    builtin::{processors::BuiltinProcessor, tasks::AddPieces},
    core::ext::run_consumer,
};

fn log_init() -> Result<()> {
    let env_filter = filter::EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::DEBUG.into())
        .from_env()
        .context("invalid env filter")?
        .add_directive("want=warn".parse()?)
        .add_directive("jsonrpc_client_transports=warn".parse()?)
        .add_directive("jsonrpc_core=warn".parse()?)
        .add_directive("hyper=warn".parse()?)
        .add_directive("mio=warn".parse()?)
        .add_directive("reqwest=warn".parse()?);

    let worker_env_filter = filter::EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::OFF.into())
        .with_env_var("VENUS_WORKER_LOG")
        .from_env()
        .context("invalid venus worker log filter")?;

    let fmt_layer = layer()
        .with_writer(std::io::stderr)
        .with_ansi(std::io::stderr().is_tty())
        .with_target(true)
        .with_thread_ids(true)
        .with_timer(
            OffsetTime::local_rfc_3339()
                .unwrap_or_else(|_| OffsetTime::new(time::UtcOffset::UTC, Rfc3339)),
        )
        .with_filter(env_filter.or(worker_env_filter));

    registry().with(fmt_layer).init();

    Ok(())
}

fn main() -> Result<()> {
    log_init()?;

    info!("start add_pieces consumer");
    run_consumer::<AddPieces, BuiltinProcessor>()
}
