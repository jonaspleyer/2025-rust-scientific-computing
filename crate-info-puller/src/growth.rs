use anyhow::Result;
use chrono::Datelike;
use crates_io_api_wasm_patch::AsyncClient;

use crate_info_puller::*;

pub async fn format_crates<T: AsRef<str>>(client: &AsyncClient, crates: &[T]) -> Result<()> {
    let mut all_downloads = std::collections::BTreeMap::<chrono::DateTime<chrono::Utc>, u64>::new();
    for name in crates.iter() {
        let cd = async_compat::Compat::new(get_data(client, name.as_ref())).await?;

        for version in cd.crate_response.versions.into_iter() {
            let date = version.updated_at;
            // let date = updated_at.format("%d/%m/%Y").to_string();
            let entry = all_downloads.entry(date).or_insert(0);
            *entry += 1;
        }
    }

    let tmin = all_downloads.keys().min().unwrap();
    let tmax = all_downloads.keys().max().unwrap();

    let interval = chrono::Duration::weeks(4);

    let mut skip_size = 0;
    let mut t = *tmin;
    let mut output = std::collections::BTreeMap::new();
    while &t <= tmax {
        t += interval;
        let n = all_downloads
            .iter()
            .skip(skip_size)
            .take_while(|(d, _)| d < &&t)
            .count();

        skip_size += n;

        use anyhow::Context;
        if let Some(x) = output.insert(t, n) {
            None.context(format!("Should not contain any value at this point: {x}"))?;
        }
    }

    let mut file = std::fs::File::create("./crate-lists/releases.csv")?;
    for (month, counts) in output.iter() {
        use std::io::Write;
        writeln!(&mut file, "{}/{},{counts}", month.month(), month.year())?;
    }

    Ok(())
}

#[macro_rules_attribute::apply(smol_macros::main)]
async fn main() -> Result<()> {
    use clap::Parser;
    use reqwest::header::*;

    let args = Args::parse();
    let mut crates = args.crates;
    if let Some(filename) = args.file {
        let content = std::fs::read_to_string(filename)?;
        crates.extend(content.lines().map(String::from));
    }
    crates.sort();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("gobbler")?);

    let client1 = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let client =
        AsyncClient::with_http_client(client1.clone(), web_time::Duration::from_millis(1000));

    format_crates(&client, &crates).await?;

    Ok(())
}
