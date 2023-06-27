use std::fs::File;
use std::io::{BufWriter, Write};

use image::{DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat};
use image::imageops::FilterType;
use webp::{Encoder, PixelLayout, WebPMemory};

use crate::matches::Matcher;

pub struct ImageConverter {
    img: DynamicImage,
    output_file_name: String,
    dimensions: (u32, u32),
    quality: u8,
    extension: String,
}

impl ImageConverter {
    pub fn new(matcher: &Matcher) -> ImageConverter {
        ImageConverter {
            img: matcher.get_image().clone(),
            output_file_name: ImageConverter::format_file_name(matcher.get_input_file(), matcher.get_extension()),
            dimensions: matcher.get_dimensions().unwrap_or(matcher.get_image().dimensions()),
            quality: matcher.get_quality(),
            extension: String::from(matcher.get_extension()),
        }
    }

    pub fn convert(&self) -> &String {
        match self.extension.as_str() {
            "webp" => self.convert_to_webp(),
            "png" => self.convert_to(ImageFormat::Png),
            "jpg" | "jpeg" => self.convert_to_jpg(),
            "gif" => self.convert_to(ImageFormat::Gif),
            "bmp" => self.convert_to(ImageFormat::Bmp),
            "ico" => self.convert_to(ImageFormat::Ico),
            "tiff" => self.convert_to(ImageFormat::Tiff),
            "tga" => self.convert_to(ImageFormat::Tga),

            _ => {
                panic!("Unsupported file format")
            }
        }
    }

    fn convert_to_webp(&self) -> &String {
        let (width, height) = self.dimensions;
        let encoder: Encoder = Encoder::new(self.img.as_bytes(), PixelLayout::Rgba, width, height);
        let webp: WebPMemory = encoder.encode(self.quality as f32);
        let mut file = File::create(&self.output_file_name).unwrap();
        file.write_all(&webp).unwrap();

        &self.output_file_name
    }

    fn convert_to_jpg(&self) -> &String {
        let output_image = self.resize_image();
        let mut out = BufWriter::new(
            File::create(
                &self.output_file_name
            )
                .unwrap()
        );
        let format = ImageOutputFormat::Jpeg(self.quality);
        output_image.write_to(&mut out, format).unwrap();

        &self.output_file_name
    }

    fn convert_to(&self, image_format: ImageFormat) -> &String {
        self.resize_image()
            .save_with_format(&self.output_file_name, image_format)
            .unwrap();

        &self.output_file_name
    }

    fn resize_image(&self) -> DynamicImage {
        let (width, height) = self.dimensions;
        self.img.resize(width, height, FilterType::Lanczos3)
    }

    fn format_file_name(input_file_name: &String, extension: &str) -> String {
        format!("{}.{}", input_file_name, extension)
    }
}