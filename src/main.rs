mod cli;
mod dataset;
mod search;
mod output;

use cli::{Cli, Commands};
use dataset::{fetch_dataset, load_cached_dataset, export_to_csv};
use output::print_results;
use search::search_dataset;
use anyhow::Result;
use indicatif::ProgressBar;
use std::path::PathBuf;
use std::time::Duration;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let dataset_url = "https://github.com/krystianbajno/credsdefault-dataset/releases/download/release/output.json";
    let cache_file = PathBuf::from("credsdefault-dataset.json");

    match &args.command {
        Commands::Update => {
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_message("Updating dataset...");
            fetch_dataset(dataset_url, &cache_file).await?;
            pb.finish_with_message("Dataset updated successfully.");
        }
        Commands::Search { keywords, csv } => {
            let dataset = load_cached_dataset(&cache_file)
                .expect("Failed to load dataset. Please run `--update` first.");
            let results = search_dataset(&dataset, keywords);

            if let Some(csv_file) = csv {
                export_to_csv(&results, csv_file)
                    .expect("Failed to export results to CSV file.");
                println!("Results exported to {}", csv_file.display());
            } else {
                print_results(&results);
            }
        }
    }
    Ok(())
}
