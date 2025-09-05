use anyhow::Result;
use crates_io_api_wasm_patch::AsyncClient;

use crate_info_puller::*;

pub async fn format_crates<T: AsRef<str>>(client: &AsyncClient, crates: &[T]) -> Result<()> {
    // Obtain recent data about the crates
    let output_path = get_db_dump().await?;
    // let output_dir = unpack_db_dump(&output_path, skip_unpacking)?;

    // let data_dir = std::fs::read_dir(&output_dir)?
    //     .next()
    //     .context("invalid db-dump")??
    //     .path()
    //     .join("data");

    let mut all_crates = Vec::with_capacity(crates.len());
    for name in kdam::tqdm!(crates.iter(), total = crates.len()) {
        let cd = async_compat::Compat::new(get_data(client, name.as_ref())).await?;
        all_crates.push(cd);
    }

    let mut dependencies = Vec::new();
    let mut crates = std::collections::BTreeMap::new();
    db_dump::Loader::new()
        .dependencies(|dep| dependencies.push(dep))
        .crates(|cra| {
            crates.insert(cra.id, cra);
        })
        .load(&output_path)?;

    let all_crates = all_crates
        .into_iter()
        .map(|cr| {
            for cr2 in crates.values() {
                if cr2.name == cr.crate_response.crate_data.id {
                    return Ok((cr2.id, (cr, cr2)));
                }
            }
            Err(anyhow::anyhow!("Could not find associated crate."))
        })
        .collect::<Result<std::collections::BTreeMap<_, _>>>()?;

    let mut deps = std::collections::BTreeMap::new();

    for (_, (_, c2)) in all_crates.iter() {
        for dep in dependencies.iter() {
            if dep.crate_id.0 == c2.id.0 && all_crates.contains_key(&dep.crate_id) {
                deps.entry(c2.id).or_insert(Vec::new()).push(dep.id);
            }
        }
    }

    use std::io::Write;
    let mut file = std::fs::File::create("./crate-lists/deplist.csv")?;
    writeln!(&mut file, "name,has_deps,is_dep,total_downloads")?;
    for (cr, deps) in deps.iter() {
        let (c1, c2) = all_crates.get(cr).unwrap();
        writeln!(
            &mut file,
            "{},{},{},{}",
            c2.name,
            c1.deps,
            deps.len(),
            c1.crate_response.crate_data.downloads,
        )?;
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
