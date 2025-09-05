use std::os::linux::fs::MetadataExt;

use anyhow::Result;
use crates_io_api_wasm_patch::{AsyncClient, CrateDownloads, CrateResponse};
use kdam::BarExt;
use serde::{Deserialize, Serialize};
use smol::io::AsyncWriteExt;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CrateData {
    pub crate_name: String,
    pub crate_response: CrateResponse,
    // pub full_crate: FullCrate,
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
        // let full_crate = client.full_crate(crate_name, true).await?;
        let num = &crate_response.versions.first().unwrap().num.clone();
        let deps = client.crate_dependencies(crate_name, num);

        let res = CrateData {
            crate_name: crate_response.crate_data.name.clone(),
            crate_response,
            // full_crate,
            downloads: dow.await?,
            deps: deps.await?.len(),
        };
        std::fs::create_dir_all(crate_path.parent().unwrap())?;
        let contents = ron::to_string(&res)?;
        std::fs::write(crate_path, contents)?;
        Ok(res)
    }
}

fn build_progress_bar(total_bytes: Option<usize>) -> Result<kdam::Bar> {
    let builder = kdam::BarBuilder::default().unit_scale(true).unit("Mb");
    let builder = if let Some(total_bytes) = total_bytes {
        builder.bar_format(
        "{desc}{percentage:3.0}%|{animation}| {count}/{total} [{elapsed} {rate:.2}{unit}/s{postfix}]")
        .total(total_bytes)
    } else {
        builder.bar_format("{count} [{elapsed} {rate:.2}{unit}/s{postfix}]")
    };
    builder.build().map_err(|e| anyhow::anyhow!(e))
}

pub async fn get_db_dump() -> Result<std::path::PathBuf> {
    // Check if there is a db-dump file from today
    // If not, download a new one
    let out_dir = std::path::PathBuf::from("./out/db-dump");
    let ofile = format!("{}.tar.gz", chrono::Utc::now().format("%Y-%m-%d"));
    let out_path = out_dir.join(ofile);

    let mut response =
        async_compat::Compat::new(reqwest::get("https://static.crates.io/db-dump.tar.gz")).await?;

    let total_bytes = response.content_length().unwrap_or_default();
    let mut pb = build_progress_bar(Some(total_bytes as usize))?;

    // Check if the file is already present
    if out_path.exists() && out_path.is_file() {
        let file_size = std::fs::File::open(&out_path)?.metadata()?.st_size();

        if file_size == total_bytes {
            return Ok(out_path);
        }
    }

    let mut file = smol::fs::File::create(&out_path).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        pb.update(chunk.len())?;
    }

    Ok(out_path)
}

pub fn unpack_db_dump(
    output_path: &std::path::Path,
    skip_unpacking: bool,
) -> Result<std::path::PathBuf> {
    let tar_gz = std::fs::File::open(output_path)?;
    // let total_bytes = tar_gz.metadata()?.st_size();
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    let output_dir = output_path.with_extension("").with_extension("");

    if output_dir.exists() && skip_unpacking {
        return Ok(output_dir);
    }

    let mut pb = build_progress_bar(None)?;
    for entry in archive.entries()? {
        let mut e = entry?;
        let size = e.size();
        e.unpack_in(&output_dir)?;
        pb.update(size as usize)?;
    }
    Ok(output_dir)
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
