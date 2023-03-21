// Browser runtime for the Demo Virtual Console
function make_environment(...envs) {
	return new Proxy(envs, {
		get(target, prop, receiver) {
			for (let env of envs) {
				if (env.hasOwnProperty(prop)) {
					return env[prop];
				}
			}
			return (...args) => {console.error("NOT IMPLEMENTED: "+prop, args)}
		}
	});
}

let app = document.getElementById("app");
let ctx = app.getContext("2d");
console.log(ctx);

WebAssembly.instantiateStreaming(fetch("drawing.wasm"), {
	"env": make_environment()
})
	.then(wasmModule => {
		console.log(wasmModule);
		const width = wasmModule.instance.exports.wasm_get_width();
		const height = wasmModule.instance.exports.wasm_get_height();
		const pixels = wasmModule.instance.exports.wasm_get_pixels();
		const buffer = wasmModule.instance.exports.memory.buffer;
		console.log(new Uint8Array(buffer, pixels, width * height * 4));
		const image = new ImageData(new Uint8ClampedArray(buffer, pixels, width * height * 4), width);

		app.width = width;
		app.height = height;
		ctx.putImageData(image, 0, 0);
	});
