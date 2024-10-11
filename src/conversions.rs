use fast_image_resize::PixelType;
use zune_image::{codecs::qoi::zune_core::colorspace::ColorSpace, image::Image};

pub fn resize_image(src_image: &Image, resize_width: usize, resize_height: usize) -> Image {
    let (width, height) = src_image.dimensions();
    let src_colorspace = src_image.colorspace();
    let mut buffer = src_image
        .flatten_to_u8()
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();
    let pixel_type = colorspace_to_pixel(src_colorspace);
    let rimage = fast_image_resize::images::Image::from_slice_u8(
        width as u32,
        height as u32,
        &mut buffer,
        pixel_type,
    )
    .unwrap();
    let mut resized_image = fast_image_resize::images::Image::new(
        resize_width as u32,
        resize_height as u32,
        pixel_type,
    );
    let mut resizer = fast_image_resize::Resizer::new();
    resizer.resize(&rimage, &mut resized_image, None).unwrap();
    let buf = resized_image.into_vec();

    Image::from_u8(buf.as_slice(), resize_width, resize_height, src_colorspace)
}

pub fn pixel_to_colorspace(pixel: fast_image_resize::PixelType) -> ColorSpace {
    match pixel {
        fast_image_resize::PixelType::U8 => ColorSpace::Luma,
        fast_image_resize::PixelType::U8x2 => ColorSpace::LumaA,
        fast_image_resize::PixelType::U8x3 => ColorSpace::RGB,
        fast_image_resize::PixelType::U8x4 => ColorSpace::RGBA,
        _ => std::process::exit(0),
    }
}

pub fn colorspace_to_pixel(colorspace: ColorSpace) -> fast_image_resize::PixelType {
    match colorspace {
        ColorSpace::RGB => fast_image_resize::PixelType::U8x3,
        ColorSpace::RGBA => PixelType::U8x4,
        ColorSpace::Luma => PixelType::U8,
        ColorSpace::LumaA => PixelType::U8x2,
        ColorSpace::YCCK => PixelType::U8x4,
        ColorSpace::CMYK => PixelType::U8x4,
        ColorSpace::BGR => PixelType::U8x3,
        ColorSpace::BGRA => PixelType::U8x4,
        ColorSpace::ARGB => PixelType::U8x4,
        ColorSpace::HSL => PixelType::U8x3,
        ColorSpace::HSV => PixelType::U8x3,
        _ => std::process::exit(0),
    }
}
