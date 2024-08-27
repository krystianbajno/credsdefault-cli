use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "credsdefault-cli")]
#[command(about = "CLI client for factory-default credentials search", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Update,
    Search {
        keywords: Vec<String>,
        #[arg(short, long)]
        csv: Option<PathBuf>,
    },
}
