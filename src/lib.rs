use anyhow::Error;
use itertools::Itertools;
use zune_image::codecs::qoi::zune_core::options::DecoderOptions;
mod conversions;
pub use conversions::*;

const ASCII_SIZE: usize = 8;
const DEFAULT_TEXTURE: &str = " .,:-=*#@";
pub const UNICODE_TEXTURE: &str = " ░▒▓█";
pub const MIXED_TEXTURE: &str = " .-=*#@░▒▓█";

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
    pub fn new(buffer: &[u8], resize_width: Option<usize>) -> anyhow::Result<Self> {
        let decode_options = DecoderOptions::default();

        let image = zune_image::image::Image::read(buffer, decode_options).unwrap();

        let meta = image.metadata();
        let (width, height) = meta.get_dimensions();

        let mut state = AppState {
            og_image: ImageRepr {
                image,
                dimensions: ImgDimensions { width, height },
            },
            ascii_size: ASCII_SIZE,
            resized_image: None,
            texture: DEFAULT_TEXTURE.to_owned(),
        };

        let aspect_ratio = width / height;

        let resize_dimensions = resize_width.and_then(|w| {
            Some(ImgDimensions {
                width: w,
                height: w / aspect_ratio,
            })
        });
        state.resize(resize_dimensions)?;

        Ok(state)
    }

    pub fn set_pixel_size(&mut self, ascii_pixel_size: usize) {
        self.ascii_size = ascii_pixel_size;
    }

    pub fn set_texture(&mut self, texture: &str) {
        self.texture = texture.to_owned()
    }

    fn resize(&mut self, dimensions: Option<ImgDimensions>) -> anyhow::Result<()> {
        let og_dims = &self.og_image.dimensions;
        let out_dims = dimensions.or_else(|| {
            term_size::dimensions().map(|d| ImgDimensions {
                width: d.0,
                height: d.1,
            })
        });
        let out_dims = out_dims.ok_or(Error::msg("failed to get term dimentions"))?;
        let (resized_width, resized_height) = if og_dims.width > og_dims.height {
            let new_width = out_dims.width - ASCII_SIZE;
            let new_height = (new_width * og_dims.height) / og_dims.width;
            (new_width, new_height)
        } else {
            let new_height = out_dims.height - ASCII_SIZE;
            let new_width = (new_height * og_dims.width) / og_dims.height;
            (new_width, new_height)
        };
        let resized = resize_image(&self.og_image.image, resized_width, resized_height);

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

    fn quantized_level(&self, value: u8) -> u32 {
        // distance between each bins (in our case 255 / 8  will give us 8 bins or 8 quantized levels)
        // 255 - max luma value, 0 - min
        let distance_bw_levels = (255 - 0) / self.texture.chars().count() as u32;
        // divide the value with quatized level to bring it to one of the 8 value range
        let value = value as u32 / distance_bw_levels;
        // for indexing it to 0
        value.saturating_sub(1)
    }

    fn to_luma(&self) -> anyhow::Result<Vec<u8>> {
        let (r, g, b) = self.resized_rgb_channels()?;
        Ok(itertools::izip!(r, g, b)
            // convert rgb to luma
            .map(|(r, g, b)| (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8)
            .collect::<Vec<_>>())
    }

    pub fn apply_texture(&self) -> anyhow::Result<String> {
        let luma = self.to_luma()?;
        let mapped_texture = self.texture.chars().collect::<Vec<_>>();
        let l = luma
            .iter()
            .map(|&l| {
                mapped_texture
                    .get(self.quantized_level(l) as usize)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        Ok(l.chunks(self.resized_image.as_ref().unwrap().width())
            .map(|cs| cs.iter().join(""))
            .join("\n"))
    }
}
