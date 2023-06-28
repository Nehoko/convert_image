use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat};
use image::imageops::FilterType;
use webp::{Encoder, WebPMemory};

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
            output_file_name: ImageConverter::format_file_name(matcher.get_input_file_path(), matcher.get_extension()),
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
        let img = self.resize_image();
        let encoder: Encoder = Encoder::from_image(&img).unwrap();
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

    fn format_file_name(input_file_path: &String, extension: &str) -> String {
        let path = Path::new(input_file_path);
        let input_extension = path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");
        let suggested_file_name = &input_file_path
            .strip_suffix(format!(".{}", input_extension).as_str())
            .unwrap_or(input_file_path)
            .to_string();
        let file_name = ImageConverter::new_file_name(suggested_file_name, extension);
        format!("{}.{}", file_name, extension)
    }

    fn new_file_name(input_file_name: &String, extension: &str) -> String {
        let file_path = format!("{}.{}", input_file_name, extension);
        let path = Path::new(file_path.as_str());

        if path.exists() {
            let file_name = format!("{}_new", input_file_name);
            ImageConverter::new_file_name(&file_name, extension)
        } else {
            String::from(input_file_name)
        }
    }
}