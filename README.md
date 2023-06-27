# Convert Image
#### Simple image converter written in Rust

## Flags
* -q (--quality). Shrinks image quality in percents of source quality if it's possible by selected extension. (Example: `-q90`. Default: `100`).
* -e (--extension). Sets desired file extension. (Example: `-ejpg`. Default: `webp`)
* -w (--width). Sets image width. (Example: `-w1024`. Default: source image width).
* -a (--height). Sets image height. Since `-h` flag is occupied by `--help` flag it was decided to use `-a` flag (stands for altitude). (Example: `-a768`. Default: source image width).

## Example
`image_converter /path/to/image.png -q80 -ewebp -w80 -a60`

## Build
simple command, as for every rust build: `cargo build --release`