use anyhow::Result;
use crates_io_api_wasm_patch::AsyncClient;

use crate_info_puller::*;

pub async fn format_crates<T: AsRef<str>>(client: &AsyncClient, crates: &[T]) -> Result<()> {
    println!("\\begin{{tabular}}{{l r r r r r}}");
    println!("    \\toprule");
    println!(
        "    Crate Name &Last Update &Version &$N_d$ &$D_\\text{{week}}$ &$D_\\text{{total}}$\\\\"
    );
    println!("    \\midrule");
    for name in crates.iter() {
        let cd = async_compat::Compat::new(get_data(client, name.as_ref())).await?;

        let last_update = cd.crate_response.crate_data.updated_at;

        let (_, &latest_version_id, latest_version) = cd
            .crate_response
            .crate_data
            .versions
            .iter()
            .flatten()
            .zip(cd.crate_response.versions.iter())
            .filter_map(|(id, version)| {
                if !version.yanked
                    && !version.num.contains("dev")
                    && !version.num.contains("alpha")
                    && !version.num.contains("rc")
                {
                    semver::Version::parse(&version.num)
                        .ok()
                        .map(|x| (x, id, version))
                } else {
                    None
                }
            })
            .max_by_key(|(smvr, _, _)| smvr.clone())
            .unwrap_or((
                semver::Version::parse(&cd.crate_response.versions[0].num)?,
                &cd.crate_response.crate_data.versions.as_ref().unwrap()[0],
                &cd.crate_response.versions[0],
            ));

        let weekly_downloads = {
            let mut n = 0;
            let w = cd
                .downloads
                .version_downloads
                .into_iter()
                .filter(|x| x.version == latest_version_id)
                .map(|di| {
                    n += 1;
                    di.downloads
                })
                .sum::<u64>();
            w as f64 / n as f64 * 7.
        };
        let total_downloads = cd.crate_response.crate_data.downloads;
        let n_deps = cd.deps;

        let tot = format!("{:>7.1}k", total_downloads as f64 / 1_000.);
        println!(
            "    {:16} &{:10} &{:10} &{:<4.0} &{:<8} &{:>10}\\\\",
            cd.crate_name.replace("_", "\\_"),
            last_update.format("%Y-%m-%d"),
            latest_version.num,
            n_deps,
            if weekly_downloads.is_nan() {
                String::new()
            } else {
                format!("{weekly_downloads:<8.0}")
            },
            tot,
        );
    }
    println!("    \\bottomrule");
    println!("\\end{{tabular}}");
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
