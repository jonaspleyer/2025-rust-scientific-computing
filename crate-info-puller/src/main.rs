use anyhow::{Context, Result};
use crates_io_api_wasm_patch::{AsyncClient, CrateDownloads, CrateResponse};

struct CrateData {
    crate_name: String,
    crate_response: CrateResponse,
    downloads: CrateDownloads,
}

async fn get_data(client: &AsyncClient, crate_name: &str) -> Result<CrateData> {
    let cre = client.get_crate(crate_name);
    let dow = client.crate_downloads(crate_name);

    Ok(CrateData {
        crate_name: crate_name.to_string(),
        crate_response: cre.await?,
        downloads: dow.await?,
    })
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Comma-separated list of crate names
    #[arg(short, long, value_delimiter = ',')]
    crates: Vec<String>,
    // #[arg(short, long)]
    // filename: Option<String>,
}

#[macro_rules_attribute::apply(smol_macros::main)]
async fn main() -> Result<()> {
    use clap::Parser;
    use reqwest::header::*;

    let args = Args::parse();
    // let crates = ["nalgebra", "ndarray", "cellular_raza", "approx-derive"];
    let crates = args.crates;

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("gobbler")?);

    let client1 = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let client =
        AsyncClient::with_http_client(client1.clone(), web_time::Duration::from_millis(1000));

    println!("\\begin{{tabular}}{{l r r}}");
    println!("    Crate Name       &Weekly Downloads   &Last Update    &Latest Version\\\\");
    for (n, name) in crates.iter().enumerate() {
        let cd = async_compat::Compat::new(get_data(&client, name)).await?;

        let mut d = cd.downloads.version_downloads;
        d.sort_by_key(|x| x.date);

        let last_update = cd.crate_response.crate_data.updated_at;
        let latest_version = cd
            .crate_response
            .crate_data
            .versions
            .and_then(|v| v.into_iter().max())
            .context("no version provided by crates.io")?;

        let weekly_downloads = {
            let mut n = 0;
            let w = d
                .into_iter()
                .filter(|x| x.version == latest_version)
                .map(|di| {
                    n += 1;
                    di.downloads
                })
                .sum::<u64>();
            w as f64 / n as f64 * 7.
        };
        let latest_version = cd.crate_response.versions[0].num.clone();

        print!(
            "    {:16} &{weekly_downloads:<8.0} &{:10} &{latest_version:10}",
            cd.crate_name,
            last_update.format("%d/%m/%Y")
        );
        if n + 1 < crates.len() {
            println!("\\\\");
        } else {
            println!();
        }
    }
    println!("\\end{{tabular}}");

    Ok(())
}
