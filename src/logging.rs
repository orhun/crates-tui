use color_eyre::eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::config;

pub fn initialize_logging() -> Result<()> {
  let config = config::get();
  let directory = config.data_dir.clone();
  std::fs::create_dir_all(directory.clone())?;
  let project_name = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
  let log_file = format!("{}.log", env!("CARGO_PKG_NAME"));
  let log_path = directory.join(log_file);
  let log_file = std::fs::File::create(log_path)?;
  let log_env = format!("{}_LOGLEVEL", project_name);
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG")
      .or_else(|_| std::env::var(log_env))
      .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
  );
  let file_subscriber = tracing_subscriber::fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_writer(log_file)
    .with_target(false)
    .with_ansi(false);
  tracing_subscriber::registry()
    .with(file_subscriber)
    .with(ErrorLayer::default())
    .with(tracing_subscriber::filter::EnvFilter::from_default_env().add_directive(config.log_level.into()))
    .init();
  Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr, $ex:expr) => {
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}