use std::fs::File;
use std::io::Write;

use clap::{Arg, ArgAction, ArgMatches, Command};
use image::DynamicImage;
use webp::{Encoder, WebPMemory};

fn main() {
    let matches = matches_init();
    let (input_file, input_file_name) = get_file_name_tuple(&matches);
    let img = read_image(input_file);
    let quality = get_quality(&matches);
    let extension = get_convert_extension(&matches);

    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(quality);
    let output_file_name = format!("{}.{}", input_file_name, extension.as_str());


    let mut file = File::create(&output_file_name).unwrap();
    let result = file.write_all(&webp);

    match result {
        Ok(()) => println!("file converted: {}", output_file_name),
        Err(error) => println!("{}", error)
    }
}

fn matches_init() -> ArgMatches {
    Command::new("Image converter")
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
        .get_matches()
}

fn get_file_name_tuple(matches: &ArgMatches) -> (String, String) {
    let input_file = matches
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

fn read_image(input_file: String) -> DynamicImage {
    image::io::Reader::open(input_file)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
}

fn get_quality(matches: &ArgMatches) -> f32 {
    matches
        .get_one::<String>("quality")
        .unwrap_or(&String::from("100"))
        .parse::<f32>().unwrap_or_else(|_| panic!("quality should be from 0 to 100"))
}

fn get_convert_extension(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("extension")
        .unwrap_or(&String::from("webp"))
        .to_string()
}