pub mod expand;
mod filter;
mod swview;
mod types;
pub mod utils;

use std::fs;
use std::path::Path;
use std::process::ExitCode;
use std::{collections::BTreeMap, rc::Rc};

use clap::Parser;
use leptos::prelude::*;

use filter::IgnoreFilter;
use types::swagger::{OpenApi, Operation};

use crate::swview::MainPage;
use utils::{anchor, operations_of};

const CSS: &str = include_str!("assets/style.css");
const JS: &str = include_str!("assets/script.js");

#[derive(Parser, Debug)]
#[command(
    name = "swagger-docs",
    version,
    about = "Generate a documentation HTML page from swagger.json",
    long_about = None,
)]
struct Args {
    #[arg(value_name = "file")]
    input: String,

    #[arg(short, long, default_value = "api-docs.html")]
    output: String,

    #[arg(short = 't', long = "ignore-tag", value_name = "GLOB")]
    ignore_tags: Vec<String>,

    #[arg(short = 'p', long = "ignore-path", value_name = "GLOB")]
    ignore_paths: Vec<String>,

    #[arg(short = 's', long = "ignore-schema", value_name = "GLOB")]
    ignore_schemas: Vec<String>,

    #[arg(long, help = "Automatically open the generated html file")]
    open: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("❌ error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let ignore_tags: Vec<String> = args
        .ignore_tags
        .iter()
        .flat_map(|item| item.split(',').map(String::from))
        .collect();
    let tag_filter = IgnoreFilter::new(&ignore_tags)?;
    let path_filter = IgnoreFilter::new(&args.ignore_paths)?;
    let schema_filter = IgnoreFilter::new(&args.ignore_schemas)?;

    if !tag_filter.is_empty() {
        println!("ignoring tags:    {}", tag_filter.patterns().join(", "));
    }
    if !path_filter.is_empty() {
        println!("ignoring paths:   {}", path_filter.patterns().join(", "));
    }
    if !schema_filter.is_empty() {
        println!("ignoring schemas: {}", schema_filter.patterns().join(", "));
    }

    let input_path = Path::new(&args.input);
    if !input_path.exists() {
        return Err(format!("File not found: {}", input_path.display()).into());
    }

    println!("parsing {}...", input_path.display());
    let data = fs::read_to_string(input_path)?;
    let mut spec: OpenApi =
        serde_json::from_str(&data).map_err(|e| format!("invalid json: {}", e))?;
    expand::expand_all(&mut spec);

    let components = Rc::new(spec.components.clone());

    let mut groups_map: BTreeMap<String, Vec<(String, String, String)>> = BTreeMap::new();
    let mut endpoints: Vec<(String, String, Operation)> = Vec::new();
    let mut skipped = 0usize;

    for (path, item) in &spec.paths {
        if path_filter.is_ignored(path) {
            skipped += operations_of(item).len();
            continue;
        }

        for (method, op) in operations_of(item) {
            let kept_tags: Vec<String> = op
                .tags
                .iter()
                .filter(|t| !tag_filter.is_ignored(t))
                .cloned()
                .collect();

            let effective_tags = if op.tags.is_empty() {
                vec!["default".to_string()]
            } else {
                kept_tags
            };

            if effective_tags.is_empty() {
                skipped += 1;
                continue;
            }

            let tag = effective_tags[0].clone();
            let a = anchor(method, path);

            groups_map
                .entry(tag)
                .or_default()
                .push((method.to_string(), path.clone(), a));

            let mut op_filtered = op.clone();
            op_filtered.tags = effective_tags;

            endpoints.push((method.to_string(), path.clone(), op_filtered));
        }
    }

    let visible_schemas: Vec<&String> = spec
        .components
        .schemas
        .keys()
        .filter(|name| !schema_filter.is_ignored(name))
        .collect();

    let groups: Vec<(String, Vec<(String, String, String)>)> =
        groups_map.clone().into_iter().collect();

    let title = spec.info.title.clone();
    let version = spec.info.version.clone();
    let ignored_tags = tag_filter.patterns().to_vec();

    let html = view! {
        <MainPage
            title=title
            version=version
            groups=groups
            endpoints=endpoints
            ignored_tags=ignored_tags
            skipped=skipped
            components=components
        />
    }
    .to_html();

    fs::write(&args.output, html)?;

    let total: usize = groups_map.values().map(|v| v.len()).sum();
    println!("generated: {}", args.output);
    println!("   {} endpoints", total);
    println!(
        "   {} visible schemas ({} total)",
        visible_schemas.len(),
        spec.components.schemas.len()
    );
    if skipped > 0 {
        println!("   {} ignored endpoints", skipped);
    }

    if args.open {
        let abs = fs::canonicalize(&args.output)
            .unwrap_or_else(|_| Path::new(&args.output).to_path_buf());
        let url = format!("file://{}", abs.display());
        let _ = open_in_browser(&url);
    }

    Ok(())
}

fn open_in_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Stdio;

        std::process::Command::new("xdg-open")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    Ok(())
}
