mod matches;
mod image_converter;
mod image_extension;

fn main() {
    let matcher = matches::Matcher::new();


    if matcher.show_list() {
        let extensions_table = image_converter::ImageConverter::get_extensions_table();
        println!("{}", extensions_table);
        return
    }

    let converter = image_converter::ImageConverter::new(&matcher);

    println!("file converted: {}", converter.convert())
}