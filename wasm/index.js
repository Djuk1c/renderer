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
let ctx = app.getContext("2d", {alpha: false});

async function o_renderDemo() {
	WebAssembly.instantiateStreaming(fetch("drawing.wasm"), {
		"env": make_environment()
	})
		.then(wasmModule => {
			//console.log(wasmModule);
			const width = wasmModule.instance.exports.wasm_get_width();
			const height = wasmModule.instance.exports.wasm_get_height();
			app.width = width;
			app.height = height;

			let i = 0;
			function myLoop() {
				setTimeout(function() {
					const pixels = wasmModule.instance.exports.wasm_get_pixels(i);
					const buffer = wasmModule.instance.exports.memory.buffer;
					const image = new ImageData(new Uint8ClampedArray(buffer, pixels, width * height * 4), width);

					ctx.putImageData(image, 0, 0);
					i++;
					myLoop();
				}, 1)
			}
			myLoop();
		});
}

async function renderDemo() {
	const wasmModule = await WebAssembly.instantiateStreaming(fetch("drawing.wasm"), {
		"env": make_environment()
	});
	const width = wasmModule.instance.exports.wasm_get_width();
	const height = wasmModule.instance.exports.wasm_get_height();
	app.width = width;
	app.height = height;

	function render(i) {
		const pixels = wasmModule.instance.exports.wasm_cube_test(i, 0.0);
		const buffer = wasmModule.instance.exports.memory.buffer;
		const image = new ImageData(new Uint8ClampedArray(buffer, pixels, width * height * 4), width);

		ctx.putImageData(image, 0, 0);
	}
    let prev = null;
	let i = 0;
    function first(timestamp) {
        prev = timestamp;
        render(0);
        window.requestAnimationFrame(loop);
    }
    function loop(timestamp) {
        const dt = timestamp - prev;
		console.log(i, dt);
		i++;
        prev = timestamp;
        render(i, dt);
        window.requestAnimationFrame(loop);
    }
	window.requestAnimationFrame(first);
}
renderDemo();
