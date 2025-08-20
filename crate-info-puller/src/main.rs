use anyhow::Context;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    use crates_io_api_wasm_patch::AsyncClient;
    use reqwest::header::*;

    let crates = ["nalgebra", "ndarray", "cellular_raza", "approx-derive"];

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("gobbler")?);

    let client1 = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let client =
        AsyncClient::with_http_client(client1.clone(), web_time::Duration::from_millis(1000));

    println!("\\begin{{tabular}}{{l r r}}");
    println!("    Crate Name       &Weekly Downloads   &Last Update    &Latest Version\\\\");
    for (n, crate_name) in crates.iter().enumerate() {
        let cr = client.get_crate(crate_name).await?;
        let downloads = client.crate_downloads(crate_name).await?;
        let mut d = downloads.version_downloads;
        d.sort_by_key(|x| x.date);

        let last_update = cr.crate_data.updated_at;
        let latest_version = cr
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
        let latest_version = cr.versions[0].num.clone();

        print!(
            "    {crate_name:16} &{weekly_downloads:<8.0} &{:10} &{latest_version:10}",
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
