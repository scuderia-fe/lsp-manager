pub mod download;

use serde::Deserialize;
use std::{
    fs::{self},
    path::Path,
};

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
}

#[allow(dead_code)]
pub enum PackageStatus {
    Pending,
    False,
    True,
}

impl Package {
    #[allow(dead_code)]
    async fn check_installed(&self) -> anyhow::Result<PackageStatus> {
        Ok(PackageStatus::Pending)
    }
}

pub fn get_registry(file_dir: &Path) -> anyhow::Result<Vec<Package>> {
    let registry = fs::read_to_string(file_dir).unwrap();
    let registry: Vec<Package> = serde_json::from_str(&registry).unwrap();
    Ok(registry)
}
