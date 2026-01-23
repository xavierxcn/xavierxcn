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
        /// Override base path (use "" for local development)
        #[arg(long)]
        base_path: Option<String>,
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
        Commands::Build { config, base_path } => run_build(&config, base_path),
    };

    if let Err(e) = result {
        error!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run_build(config_path: &str, base_path_override: Option<String>) -> Result<()> {
    let mut config = Config::load(config_path)?;

    // Override base_path if provided
    if let Some(bp) = base_path_override {
        config.site.base_path = bp;
    }

    let builder = Builder::new(config)?;
    builder.build()?;
    Ok(())
}
