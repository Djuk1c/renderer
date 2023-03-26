#cargo run && display output.ppm
#cargo run && convert output.ppm -resize 2000x2000 new_image.jpg
if [ "$1" == "wasm" ]; then
	cargo build --target wasm32-unknown-unknown
	wasm-gc target/wasm32-unknown-unknown/debug/drawing.wasm -o wasm/drawing.wasm
elif [ "$1" == "r" ]; then
	cargo test --release -- --nocapture
else
	#cargo run && convert output.ppm output.jpg && display output.jpg
	#rm output.ppm
	cargo test -- --nocapture
fi
