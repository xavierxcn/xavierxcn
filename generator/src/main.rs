mod build;
mod config;
mod content;
mod render;

use anyhow::Result;
use build::Builder;
use clap::{Parser, Subcommand};
use config::Config;
use tracing::error;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "xavierxcn-generator")]
#[command(about = "A static site generator for xavierxcn personal blog")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site
    Build {
        /// Path to config file
        #[arg(short, long, default_value = "config.yaml")]
        config: String,
    },
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Build { config } => run_build(&config),
    };

    if let Err(e) = result {
        error!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run_build(config_path: &str) -> Result<()> {
    let config = Config::load(config_path)?;
    let builder = Builder::new(config)?;
    builder.build()?;
    Ok(())
}
