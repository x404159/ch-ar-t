use std::{io::Read, path::PathBuf};

use ch_ar_t::AppState;
use clap::*;
use reqwest::blocking::Client;

fn main() -> anyhow::Result<()> {
    let Args { url, path } = Args::parse();
    let client = Client::new();
    let image = match (url, path) {
        (Some(url), None) => {
            let mut image = Vec::new();
            client.get(url).send()?.read_to_end(&mut image)?;
            image
        }
        (None, Some(path)) => std::fs::read(path)?,
        _ => std::process::exit(1),
    };

    let app = AppState::new(&image, None)?;
    let img = app.apply_texture()?;

    print!("{}", img);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,

    #[arg(short, long)]
    path: Option<PathBuf>,
}
