use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, write, File},
    io::prelude::*,
    path::Path,
};

const MASON_RELEASE_URI: &str =
    "https://api.github.com/repos/mason-org/mason-registry/releases/latest";

#[derive(Debug, Deserialize, Serialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

pub async fn download_registry(file_path: &Path, dir: &str) -> anyhow::Result<()> {
    dotenv().expect(".env file not found");
    let github_token = env::var("GITHUB_TOKEN").expect("Cannot find GITHUB_TOKEN key");

    println!("Downloading latest mason lsp releases");
    let client = reqwest::Client::new();
    let response: Release = client
        .get(MASON_RELEASE_URI)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", github_token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "User")
        .send()
        .await?
        .json::<Release>()
        .await?;

    let asset = response
        .assets
        .iter()
        .find(|asset| asset.name.eq("registry.json.zip"))
        .unwrap();

    println!("Downloading registry tagged {}", response.tag_name);

    let mut tmpfile: File = tempfile::tempfile().unwrap();

    let response = client
        .get(asset.browser_download_url.as_str())
        .send()
        .await?
        .bytes()
        .await?;

    println!("Done");

    tmpfile.write_all(&response[..])?;

    let mut archive = zip::ZipArchive::new(tmpfile).unwrap();

    let mut file = archive.by_name("registry.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Saving to path {}", &file_path.to_str().unwrap());
    fs::create_dir_all(dir).unwrap();

    write(file_path, contents).unwrap();

    Ok(())
}
