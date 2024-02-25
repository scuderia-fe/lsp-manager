use std::path;

mod registry;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = format!(
        "{}/.config/helix-lsp",
        simple_home_dir::home_dir().unwrap().to_str().unwrap()
    );
    let file_path = format!("{}/registry.json", dir);
    let file_path = path::Path::new(&file_path);

    if !file_path.exists() {
        registry::download::download_registry(&file_path, &dir).await?;
    }

    let packages = registry::get_registry(&file_path).unwrap();

    ui::print_table(packages).unwrap();

    Ok(())
}
