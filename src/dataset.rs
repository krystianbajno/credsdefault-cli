use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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

pub async fn fetch_dataset(url: &str, cache_file: &PathBuf) -> Result<Vec<CredentialEntry>> {
    let response = reqwest::get(url)
        .await
        .context("Failed to make a request to the dataset URL")?
        .text()
        .await
        .context("Failed to read the response body")?;

    let dataset: Vec<CredentialEntry> = serde_json::from_str(&response)
        .context("Failed to parse the dataset as a JSON array of credentials")?;

    let mut file = File::create(cache_file)
        .context("Failed to create the cache file for the dataset")?;
    file.write_all(response.as_bytes())
        .context("Failed to write the dataset to the cache file")?;

    Ok(dataset)
}

pub fn load_cached_dataset(cache_file: &PathBuf) -> Result<Vec<CredentialEntry>> {
    let file = File::open(cache_file).context("Failed to open the cache file")?;
    let dataset: Vec<CredentialEntry> =
        serde_json::from_reader(file).context("Failed to read the dataset from the cache file")?;
    Ok(dataset)
}

pub fn export_to_csv(results: &[CredentialEntry], file_path: &PathBuf) -> Result<()> {
    let mut writer = csv::Writer::from_path(file_path)?;
    for entry in results {
        writer.serialize(entry)?;
    }
    writer.flush()?;
    Ok(())
}
