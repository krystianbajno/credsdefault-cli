mod cli;
mod dataset;
mod search;
mod output;
mod loading;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use dataset::{fetch_dataset, load_cached_dataset, export_to_csv};
use output::print_results;
use search::search_dataset;
use std::path::PathBuf;
use loading::perform_with_loading;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let dataset_url = "https://github.com/krystianbajno/credsdefault-dataset/releases/download/release/output.json";
    let cache_file = PathBuf::from("credsdefault-dataset.json");

    match &args.command {
        Commands::Update => {
            perform_with_loading("+ Updating dataset...", || async {
                fetch_dataset(dataset_url, &cache_file).await
            }).await?;
        }
        Commands::Search { keywords, csv } => {
            let dataset = perform_with_loading("+ Loading dataset", || async {
                load_cached_dataset(&cache_file).await
            }).await.expect("Failed to load dataset. Please run `credsdefault-cli update` first.");
            
            let results = perform_with_loading("+ Searching dataset", || async {
                Ok(search_dataset(&dataset, keywords))
            }).await?;

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
