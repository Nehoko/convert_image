use clap::{Arg, ArgAction, ArgMatches, Command};
use image::DynamicImage;

pub struct Matcher {
    input_file_path: String,
    extension: String,
    image: DynamicImage,
    quality: u8,
    dimensions: Option<(u32, u32)>,

}

impl Matcher {
    pub fn new() -> Matcher {
        let arg_matches = Command::new("Image converter")
            .version("1.0")
            .author("Your Name <you@example.com>")
            .about("Converts images to webp format")
            .arg(Arg::new("input")
                .help("Sets the input file to convert")
                .required(true)
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
            .arg(Arg::new("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .help("Sets the width of the output image")
                .action(ArgAction::Set))
            .arg(Arg::new("height")
                .short('a')
                .long("height")
                .value_name("HEIGHT")
                .help("Sets the height of the output image")
                .action(ArgAction::Set))
            .get_matches();

        let (input_file_path, _input_file_name) = Matcher::get_file_name_tuple(&arg_matches);
        let image = Matcher::read_image(&input_file_path);
        let quality = Matcher::read_quality(&arg_matches);
        let dimensions = Matcher::read_dimensions(&arg_matches);
        let extension = Matcher::read_convert_extension(&arg_matches);

        Matcher { input_file_path, extension, image, quality, dimensions }
    }

    fn get_file_name_tuple(matcher: &ArgMatches) -> (String, String) {
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

    fn read_dimensions(matcher: &ArgMatches) -> Option<(u32, u32)> {
        let width = matcher.get_one::<String>("width");
        let height = matcher.get_one::<String>("height");

        match width.is_some() && height.is_some() {
            true => Some((width.unwrap().parse::<u32>().unwrap(),
                          height.unwrap().parse::<u32>().unwrap())),
            false => None
        }
    }

    fn read_convert_extension(matcher: &ArgMatches) -> String {
        matcher.get_one::<String>("extension")
            .unwrap_or(&String::from("webp"))
            .to_string()
    }

    pub fn get_input_file_path(&self) -> &String {
        &self.input_file_path
    }

    pub fn get_extension(&self) -> &String {
        &self.extension
    }

    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }

    pub fn get_quality(&self) -> u8 {
        self.quality
    }

    pub fn get_dimensions(&self) -> &Option<(u32, u32)> {
        &self.dimensions
    }
}
