use anyhow::Result;
use crates_io_api_wasm_patch::{AsyncClient, CrateDownloads, CrateResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CrateData {
    pub crate_name: String,
    pub crate_response: CrateResponse,
    pub downloads: CrateDownloads,
    pub deps: usize,
}

pub async fn get_data(client: &AsyncClient, crate_name: &str) -> Result<CrateData> {
    // First check if data was stored to disk
    let mut crate_path = std::path::PathBuf::new();
    crate_path.push("./out/");
    crate_path.push(crate_name);

    if crate_path.exists() {
        let content = std::fs::read_to_string(crate_path)?;
        let res: CrateData = ron::from_str(&content)?;
        Ok(res)
    } else {
        let dow = client.crate_downloads(crate_name);
        let crate_response = client.get_crate(crate_name).await?;
        let num = &crate_response.versions.first().unwrap().num.clone();
        let deps = client.crate_dependencies(crate_name, num);

        let res = CrateData {
            crate_name: crate_name.to_string(),
            crate_response,
            downloads: dow.await?,
            deps: deps.await?.len(),
        };
        std::fs::create_dir_all(crate_path.parent().unwrap())?;
        let contents = ron::to_string(&res)?;
        std::fs::write(crate_path, contents)?;
        Ok(res)
    }
}

/// Query crates.io and generate a summary table
///
/// The crate names are sorted by name before any output is produced.
#[derive(clap::Parser, Debug)]
#[command(version, about, max_term_width = 60)]
pub struct Args {
    /// Comma-separated list of crate names
    #[arg(short, long, value_delimiter = ',')]
    pub crates: Vec<String>,
    /// Read crate names from lines in a text file
    #[arg(short, long)]
    pub file: Option<String>,
}
