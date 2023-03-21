#cargo run && display output.ppm
#cargo run && convert output.ppm -resize 2000x2000 new_image.jpg
cargo run && convert output.ppm output.jpg && display output.jpg
rm output.ppm
