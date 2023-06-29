use image::ImageFormat;

pub struct ImageExtension {
    extension: String,
    readable: bool,
    writable: bool
}

impl ImageExtension {
    pub fn vec_from(image_format: &ImageFormat) -> Vec<ImageExtension> {
        image_format
            .extensions_str()
            .iter()
            .cloned()
            .map(|extension| -> ImageExtension {
                ImageExtension {
                    extension: String::from(extension),
                    readable: image_format.can_read(),
                    writable: image_format.can_write() }
            })
            .collect()
    }

    pub fn get_extension(&self) -> &String {
        &self.extension
    }

    pub fn is_readable(&self) -> &bool {
        &self.readable
    }

    pub fn is_writable(&self) -> &bool {
        &self.writable
    }
}