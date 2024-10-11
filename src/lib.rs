use anyhow::Error;
use zune_image::codecs::qoi::zune_core::options::DecoderOptions;
mod conversions;
pub use conversions::*;

const ASCII_SIZE: usize = 8;
const DEFAULT_TEXTURE: &str = " .,:-=*#@░";

pub struct ImageRepr {
    pub image: zune_image::image::Image,
    pub dimensions: ImgDimensions,
}

impl ImageRepr {
    pub fn width(&self) -> usize {
        self.dimensions.width
    }
    pub fn height(&self) -> usize {
        self.dimensions.height
    }
}

pub struct ImgDimensions {
    pub width: usize,
    pub height: usize,
}

pub struct AppState {
    pub og_image: ImageRepr,
    pub resized_image: Option<ImageRepr>,
    pub ascii_size: usize,
    pub texture: String,
}

impl AppState {
    pub fn new(buffer: &[u8], ascii_pixel_size: Option<usize>) -> anyhow::Result<Self> {
        let decode_options = DecoderOptions::default();

        let image = zune_image::image::Image::read(buffer, decode_options).unwrap();

        let meta = image.metadata();
        let (width, height) = meta.get_dimensions();

        let mut state = AppState {
            og_image: ImageRepr {
                image,
                dimensions: ImgDimensions { width, height },
            },
            ascii_size: ascii_pixel_size.unwrap_or(ASCII_SIZE),
            resized_image: None,
            texture: DEFAULT_TEXTURE.to_owned(),
        };

        state.resize()?;

        Ok(state)
    }

    pub fn set_texture(&mut self, texture: &str) {
        self.texture = texture.to_owned()
    }

    fn resize(&mut self) -> anyhow::Result<()> {
        let ImgDimensions { width, height } = self.og_image.dimensions;
        let (term_width, term_height) =
            term_size::dimensions().ok_or(Error::msg("failed to get term dimentions"))?;
        let (resized_width, resized_height) = if width > height {
            let new_width = term_width - ASCII_SIZE;
            let new_height = (new_width * height) / width;
            (new_width, new_height)
        } else {
            let new_height = term_height - ASCII_SIZE;
            let new_width = (new_height * width) / height;
            (new_width, new_height)
        };
        let resized = resize_image(&self.og_image.image, resized_width, resized_height);
        // might not need it
        let (width, height) = resized.dimensions();

        self.resized_image = Some(ImageRepr {
            image: resized,
            dimensions: ImgDimensions {
                width: resized_width,
                height: resized_height,
            },
        });
        Ok(())
    }

    fn resized_rgb_channels(&self) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        if let Some(resized) = self.resized_image.as_ref() {
            let channels = resized
                .image
                .channels_ref(true)
                .into_iter()
                .map(|c| c.reinterpret_as::<u8>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| anyhow::Error::msg("cannot get channels"))?;
            //rgb
            Ok((
                channels[0].to_vec(),
                channels[1].to_vec(),
                channels[2].to_vec(),
            ))
        } else {
            Err(anyhow::Error::msg("please resize image first"))
        }
    }

    fn to_luma(&self) -> anyhow::Result<Vec<f32>> {
        let (r, g, b) = self.resized_rgb_channels()?;
        Ok(itertools::izip!(r, g, b)
            // convert rgb to luma
            .map(|(r, g, b)| (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8)
            // convert luma 0-255 u8 values to 0-1 f32
            .map(|l| l as f32 / 255.0)
            // convert 0-1 infinite range value to 10 value ( .0, .1, .2, .3, .4, .5, .6, .7, .8, .9 )
            .map(|fl| (fl * 10.0).floor() / 10.0)
            .collect::<Vec<_>>())
    }

    pub fn apply_texture(&self) -> anyhow::Result<String> {
        let luma = self.to_luma()?;
        let mut textured_img = String::new();
        let texture = self.texture.chars().collect::<Vec<_>>();
        for (idx, luma) in luma.iter().enumerate() {
            push_texture(&mut textured_img, *luma, &texture)?;
            if (idx + 1)
                % self
                    .resized_image
                    .as_ref()
                    .ok_or(Error::msg("resize image first"))?
                    .dimensions
                    .width
                == 0
            {
                textured_img.push('\n');
            }
        }
        Ok(textured_img)
    }
}

pub fn push_texture(
    textured_img: &mut String,
    luma_value: f32,
    texture: &[char],
) -> anyhow::Result<()> {
    Ok(match luma_value {
        0.0 => textured_img.push(*texture.get(0).ok_or(Error::msg("could not get texture"))?),
        0.1 => textured_img.push(*texture.get(1).ok_or(Error::msg("could not get texture"))?),
        0.2 => textured_img.push(*texture.get(2).ok_or(Error::msg("could not get texture"))?),
        0.3 => textured_img.push(*texture.get(3).ok_or(Error::msg("could not get texture"))?),
        0.4 => textured_img.push(*texture.get(4).ok_or(Error::msg("could not get texture"))?),
        0.5 => textured_img.push(*texture.get(5).ok_or(Error::msg("could not get texture"))?),
        0.6 => textured_img.push(*texture.get(6).ok_or(Error::msg("could not get texture"))?),
        0.7 => textured_img.push(*texture.get(7).ok_or(Error::msg("could not get texture"))?),
        0.8 => textured_img.push(*texture.get(8).ok_or(Error::msg("could not get texture"))?),
        0.9 => textured_img.push(*texture.get(9).ok_or(Error::msg("could not get texture"))?),
        _ => (),
    })
}
