mod matches;
mod image_converter;

fn main() {
    let matcher = matches::Matcher::new();
    let converter = image_converter::ImageConverter::new(&matcher);

    println!("file converted: {}", converter.convert())
}