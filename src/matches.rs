use clap::{Arg, ArgAction, ArgMatches, Command};
use image::DynamicImage;

use crate::image_converter::ImageConverter;

pub struct Matcher {
    show_list: bool,
    input_file_path: Option<String>,
    extension: Option<String>,
    image: Option<DynamicImage>,
    quality: Option<u8>,
    multiplier: Option<f32>,
}

impl Matcher {
    pub fn new() -> Matcher {
        let table = ImageConverter::get_extensions_table();
        let arg_matches = Command::new("Image converter")
            .version("1.0")
            .author("Your Name <you@example.com>")
            .about(
                format!("Converts images to desired format. \n\
                Formats: \n\
                {}", table))
            .arg(Arg::new("input")
                .help("Sets the input file to convert")
                .required(false)
                .index(1))
            .arg(Arg::new("extension")
                .short('e')
                .long("extension")
                .value_name("EXTENSION")
                .help("Sets the extension of the output image")
                .default_value("webp")
                .action(ArgAction::Set))
            .arg(Arg::new("quality")
                .short('q')
                .long("quality")
                .value_name("QUALITY")
                .default_value("100")
                .help("Sets the quality of the output image")
                .action(ArgAction::Set))
            .arg(Arg::new("multiplier")
                .short('x')
                .long("multiplier")
                .value_name("MULTIPLIER")
                .default_value("1.0")
                .help("Sets size image multiplier. Final result will be rounded")
                .action(ArgAction::Set))
            .arg(Arg::new("list")
                .short('l')
                .long("list")
                .value_name("LIST")
                .help("Sets size image multiplier. Final result will be rounded")
                .action(ArgAction::SetTrue))
            .get_matches();

        let show_list = Matcher::read_show_list(&arg_matches);

        if show_list {
            return Matcher {
                show_list,
                input_file_path: None,
                extension: None,
                image: None,
                quality: None,
                multiplier: None,
            };
        }

        let (input_file_path, _input_file_name) = Matcher::read_file_name_tuple(&arg_matches);
        let image = Matcher::read_image(&input_file_path);
        let quality = Matcher::read_quality(&arg_matches);
        let multiplier = Matcher::read_multiplier(&arg_matches);
        let extension = Matcher::read_convert_extension(&arg_matches);

        Matcher {
            show_list,
            input_file_path: Some(input_file_path),
            extension: Some(extension),
            image: Some(image),
            quality: Some(quality),
            multiplier: Some(multiplier),
        }
    }

    fn read_show_list(matcher: &ArgMatches) -> bool {
        matcher.get_flag("list")
    }

    fn read_file_name_tuple(matcher: &ArgMatches) -> (String, String) {
        let input_file = matcher
            .get_one::<String>("input")
            .expect("You must specify file path.")
            .to_string();

        let input_file_name: String = input_file
            .split('.')
            .next()
            .unwrap_or(&input_file)
            .to_string();

        (input_file, input_file_name)
    }

    fn read_image(input_file_path: &String) -> DynamicImage {
        image::io::Reader::open(input_file_path)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
    }

    fn read_quality(matcher: &ArgMatches) -> u8 {
        matcher.get_one::<String>("quality").unwrap().parse::<u8>().unwrap()
    }

    fn read_multiplier(matcher: &ArgMatches) -> f32 {
        matcher.get_one::<String>("multiplier")
            .unwrap()
            .parse::<f32>()
            .unwrap()
    }

    fn read_convert_extension(matcher: &ArgMatches) -> String {
        matcher.get_one::<String>("extension")
            .unwrap_or(&String::from("webp"))
            .to_string()
    }

    pub fn show_list(&self) -> bool {
        self.show_list
    }

    pub fn get_input_file_path(&self) -> &Option<String> {
        &self.input_file_path
    }

    pub fn get_extension(&self) -> &Option<String> {
        &self.extension
    }

    pub fn get_image(&self) -> &Option<DynamicImage> {
        &self.image
    }

    pub fn get_quality(&self) -> Option<u8> {
        self.quality
    }

    pub fn get_multiplier(&self) -> &Option<f32> {
        &self.multiplier
    }
}
