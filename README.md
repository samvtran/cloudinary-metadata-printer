# Cloudinary Metadata Printer
A WebAssembly package to print all available metadata available to [custom functions](https://cloudinary.com/documentation/custom_functions).

# Prerequisites
- Node >= 8
- Rust >= 1.39
- `CLOUDINARY_CLOUD_NAME`, `CLOUDINARY_API_KEY`, and `CLOUDINARY_API_SECRET` should all be available as envvars either locally to `make` and `npm start` or globally

# Usage
- `cargo install wasm-pack`
- `npm install`
- `make`
- Given a Cloudinary URL for an image (ideally with a dark background), modify the URL to include the custom function like
 `.../image/upload/fn_wasm:print_meta_bg<hash>.wasm/...` to print all of the metadata values available