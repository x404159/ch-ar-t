use std::{io::Read, path::PathBuf};

use anyhow::Error;
use ch_ar_t::{resize_image, AppState};
use clap::*;
use reqwest::blocking::Client;
use zune_image::codecs::qoi::zune_core::options::DecoderOptions;

const ASCII_SIZE: usize = 8;

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

    // let decode_options = DecoderOptions::default();

    // let image = zune_image::image::Image::read(&image, decode_options).unwrap();

    // let meta = image.metadata();
    // let (width, height) = meta.get_dimensions();
    // let (term_width, term_height) =
    //     term_size::dimensions().ok_or(Error::msg("failed to get term dimentions"))?;
    // let (new_width, new_height) = if width > height {
    //     let new_width = term_width - ASCII_SIZE;
    //     let new_height = (new_width * height) / width;
    //     (new_width, new_height)
    // } else {
    //     let new_height = term_height - ASCII_SIZE;
    //     let new_width = (new_height * width) / height;
    //     (new_width, new_height)
    // };
    // let resized = resize_image(&image, new_width, new_height);
    // let (width, height) = resized.dimensions();

    // let channels = resized
    //     .channels_ref(true)
    //     .into_iter()
    //     .map(|c| c.reinterpret_as::<u8>().unwrap())
    //     .collect::<Vec<_>>();
    // let (r, g, b) = (channels[0], channels[1], channels[2]);

    // let luma = itertools::izip!(r, g, b)
    //     // convert rgb to luma
    //     .map(|(&r, &g, &b)| (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8)
    //     // convert luma 0-255 u8 values to 0-1 f32
    //     .map(|l| l as f32 / 255.0)
    //     // convert 0-1 infinite range value to 10 value ( .0, .1, .2, .3, .4, .5, .6, .7, .8, .9 )
    //     .map(|fl| (fl * 10.0).floor() / 10.0)
    //     .collect::<Vec<_>>();

    // let texture = [' ', '.', ',', ':', '-', '=', '*', '#', '@', '░'];

    // let mut img = String::new();

    // luma.iter().enumerate().for_each(|(idx, luma)| {
    //     match luma {
    //         0.0 => img.push(*texture.get(0).unwrap()),
    //         0.1 => img.push(*texture.get(1).unwrap()),
    //         0.2 => img.push(*texture.get(2).unwrap()),
    //         0.3 => img.push(*texture.get(3).unwrap()),
    //         0.4 => img.push(*texture.get(4).unwrap()),
    //         0.5 => img.push(*texture.get(5).unwrap()),
    //         0.6 => img.push(*texture.get(6).unwrap()),
    //         0.7 => img.push(*texture.get(7).unwrap()),
    //         0.8 => img.push(*texture.get(8).unwrap()),
    //         0.9 => img.push(*texture.get(9).unwrap()),
    //         _ => (),
    //     }
    //     if (idx + 1) % width == 0 {
    //         img.push('\n');
    //     }
    // });

    print!("{}", img);
    Ok(())

    // let limage = zune_image::image::Image::from_u8(&luma, width, height, ColorSpace::Luma);
    // limage.save("new_luma.jpg").unwrap();
    // dbg!(luma.len(), width * height);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,

    #[arg(short, long)]
    path: Option<PathBuf>,
}
