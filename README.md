# Convert Image
#### Simple image converter written in Rust

## Flags
* -q (--quality). Shrinks image quality in percents of source quality if it's possible by selected extension. (Example: `-q90`. Default: `100`).
* -e (--extension). Sets desired file extension. (Example: `-ejpg`. Default: `webp`)
* -x (--resize). Sets size image multiplier. Final result will be rounded. (Example: `-x5.5`. Default: `1`)
* -l (--list). Shows all available extensions.

## Examples
- `image_converter /path/to/image.png -q80 -ewebp -x0.3`
- `image_converter -l`

## Build
simple command, as for every rust build: `cargo build --release`