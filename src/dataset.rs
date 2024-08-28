use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::{Context, Error, Result};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;
use std::path::Path;
use std::io::BufReader;
use std::fs::File as StdFile;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialEntry {
    pub manufacturer: String,
    pub model: String,
    pub version: String,
    pub role: String,
    pub login: String,
    pub password: String,
    pub method: String,
    pub source: String,
    pub comment: String,
    pub port: String,
    pub address: String,
}

pub async fn fetch_dataset(url: &str, cache_file: &Path) -> Result<Vec<CredentialEntry>> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send request to dataset URL")?
        .error_for_status()
        .context("Received unsuccessful status code from dataset URL")?
        .text()
        .await
        .context("Failed to read response text")?;

    let dataset: Vec<CredentialEntry> = serde_json::from_str(&response)
        .context("Failed to parse JSON dataset")?;

    let mut file = File::create(cache_file)
        .await
        .context("Failed to create cache file")?;
    file.write_all(response.as_bytes())
        .await
        .context("Failed to write dataset to cache file")?;

    Ok(dataset)
}

pub async fn load_cached_dataset(cache_file: &Path) -> Result<Vec<CredentialEntry>> {
    let cache_file = cache_file.to_path_buf();
    let dataset = task::spawn_blocking(move || -> Result<Vec<CredentialEntry>, Error> {
        let file = StdFile::open(&cache_file)
            .context("Failed to open cache file")?;
        let reader = BufReader::new(file);

        let dataset: Vec<CredentialEntry> = serde_json::from_reader(reader)
            .context("Failed to parse JSON from cache file")?;

        Ok(dataset)
    })
    .await?
    .context("Failed to load dataset in a blocking manner")?;

    Ok(dataset)
}

pub fn export_to_csv(results: &[CredentialEntry], file_path: &Path) -> Result<()> {
    let file = StdFile::create(file_path)
        .context("Failed to create CSV file")?;
    let writer = std::io::BufWriter::new(file);
    let mut csv_writer = csv::Writer::from_writer(writer);

    for entry in results {
        csv_writer.serialize(entry)
            .context("Failed to serialize entry to CSV")?;
    }

    csv_writer.flush()
        .context("Failed to flush CSV writer")?;

    Ok(())
}
