use std::{io::Read, path::PathBuf};

use ch_ar_t::{AppState, MIXED_TEXTURE, UNICODE_TEXTURE};
use clap::*;
use reqwest::blocking::Client;

fn main() -> anyhow::Result<()> {
    let Args {
        url,
        path,
        width,
        texture,
    } = Args::parse();
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

    let mut app = AppState::new(&image, width)?;

    texture.map(|t| match t {
        1 => app.set_texture(UNICODE_TEXTURE),
        2 => app.set_texture(MIXED_TEXTURE),
        _ => (),
    });

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

    #[arg(short, long)]
    width: Option<usize>,

    #[arg(short, long)]
    texture: Option<usize>,
}
