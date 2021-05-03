mod bunny;
pub mod command;
pub mod config;
pub mod error;
pub mod functions;
pub mod path;

use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const PACKAGE: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = match config::Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let path = path::Path::load();

    let bunny = bunny::Bunny::new(config, path).await;

    bunny.run().await?;

    Ok(())
}
