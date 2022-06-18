extern crate lazy_static;

mod command_line;
mod comserver;
mod settings;

use anyhow::Result;
use tokio::time::Duration;
use tokio_graceful_shutdown::Toplevel;

#[tokio::main]
async fn main() -> Result<()> {
    // Query command line options and initialize logging
    let _opts = command_line::parse();

    // Initialize and run subsystems
    Toplevel::new()
        .start("comserver", comserver::comserver)
        //.start("database_manager", database_manager::database_manager)
        .catch_signals()
        .handle_shutdown_requests(Duration::from_millis(1000))
        .await
}
