use std::ffi::OsStr;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use image::{DynamicImage, ImageFormat, ImageOutputFormat};
use image::imageops::FilterType;
use webp::{Encoder, WebPMemory};

use crate::image_extension::ImageExtension;
use crate::matches::Matcher;

pub struct ImageConverter {
    img: DynamicImage,
    output_file_name: String,
    multiplier: f32,
    quality: u8,
    extension: String,
}

impl ImageConverter {
    pub fn new(matcher: &Matcher) -> ImageConverter {
        let img = matcher.get_image().as_ref().unwrap().clone();
        let output_file_name = ImageConverter::format_file_name(
            matcher.get_input_file_path().as_ref().unwrap(),
            matcher.get_extension().as_ref().unwrap().as_str(),
        );
        ImageConverter {
            img,
            output_file_name,
            multiplier: matcher.get_multiplier().unwrap_or(1.0),
            quality: matcher.get_quality().unwrap(),
            extension: matcher.get_extension().as_ref().unwrap().clone(),
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
        let width = (self.img.width() as f32 * self.multiplier).round() as u32;
        let height = (self.img.height() as f32 * self.multiplier).round() as u32;
        self.img.resize(width, height, FilterType::Lanczos3)
    }

    fn format_file_name(input_file_path: &String, extension: &str) -> String {
        let input_extension = Path::new(input_file_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");
        let suggested_file_name = &input_file_path
            .strip_suffix(format!(".{}", input_extension).as_str())
            .unwrap_or(input_file_path)
            .to_string();
        ImageConverter::new_file_name(suggested_file_name, extension)
    }

    fn new_file_name(input_file_name: &String, extension: &str) -> String {
        let mut index = 1;

        loop {
            let file_path = if index == 1 {
                format!("{}.{}", input_file_name, extension)
            } else {
                format!("{} ({}).{}", input_file_name, index, extension)
            };

            let path = Path::new(&file_path);

            if !path.exists() {
                return file_path;
            }

            index += 1;
        }
    }

    pub fn get_extensions_table() -> String {
        let image_extensions: Vec<ImageExtension> = vec![
            ImageFormat::Png,
            ImageFormat::Jpeg,
            ImageFormat::Gif,
            ImageFormat::WebP,
            ImageFormat::Pnm,
            ImageFormat::Tiff,
            ImageFormat::Tga,
            ImageFormat::Dds,
            ImageFormat::Bmp,
            ImageFormat::Ico,
            ImageFormat::Hdr,
            ImageFormat::OpenExr,
            ImageFormat::Farbfeld,
            ImageFormat::Avif,
            ImageFormat::Qoi,
        ]
            .iter()
            .flat_map(ImageExtension::vec_from)
            .collect();

        let splitter = "+--------+-----------+----------+\n";
        let mut about_text = String::from(splitter);
        about_text.push_str("| Format | Readable  | Writable |\n");
        about_text.push_str(splitter);

        for image_extension in image_extensions {
            let mut row = String::new();
            let extension = image_extension.get_extension();
            let readable = image_extension.is_readable();
            let writable = image_extension.is_writable();

            writeln!(&mut row, "| {:<6} | {:<9} | {:<8} |\n", extension, readable, writable).unwrap();
            about_text.push_str(&row);
            about_text.push_str(splitter);
        }

        about_text
    }
}