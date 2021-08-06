# UWU Svelte Component

This [Svelte](https://svelte.dev/) app embeds the WebAssembly (wasm) code inline to make for easy use in a webpage or svelte app.

## Use

```
cd svelte
npm run dev
```

Then open your browser to localhost.


## Build notes

If you get a wasm-pack error when building on windows, try changing this line in `@wasm-tools/rollup-plugin-rust/index.js` (as of v 1.0.7)

```
const command = (process.platform === "win32" ? "wasm-pack.cmd" : "wasm-pack");

// to:

const command = (process.platform === "win32" ? "wasm-pack.exe" : "wasm-pack");
```

