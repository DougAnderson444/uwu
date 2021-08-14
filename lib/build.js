const { build } = require("esbuild");

// By default, esbuild's bundler is configured to generate code intended for the browser

// legacy options:
build({
  entryPoints: ["./lib/index.ts"],
  outdir: "./dist",
  minify: false,
  bundle: false,
  platform: 'node',
  format: "cjs"
}).catch(() => process.exit(1));

// explicit nodejs options
build({
  entryPoints: ["./lib/index.ts"],
  outfile: './dist/index.cjs.js',
  minify: false,
  bundle: false,
  platform: 'node',
  format: "cjs"
}).catch(() => process.exit(1));

// explicit browser options
build({
  entryPoints: ["./lib/index.ts"],
  outfile: './dist/index.iife.js',
  minify: false,
  bundle: true, // bundled
  format: "iife", // iife format is the default format
  target: ['chrome58', 'firefox57', 'safari11', 'edge16']
}).catch(() => process.exit(1));

build({
  entryPoints: ["./lib/index.ts"],
  outfile: './dist/index.esm.js',
  minify: false,
  bundle: false,
  format: "esm"
}).catch(() => process.exit(1));