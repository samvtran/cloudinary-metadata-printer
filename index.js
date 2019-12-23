const cloudinary = require('cloudinary').v2;
const path = require('path');

const {
  CLOUDINARY_CLOUD_NAME,
  CLOUDINARY_API_KEY,
  CLOUDINARY_API_SECRET,
} = process.env;

if (!CLOUDINARY_API_SECRET || !CLOUDINARY_API_KEY || !CLOUDINARY_CLOUD_NAME) {
  throw new Error("Please set CLOUDINARY_(API_KEY|API_SECRET|CLOUD_NAME)")
}

cloudinary.config({
  cloud_name: CLOUDINARY_CLOUD_NAME,
  api_key: CLOUDINARY_API_KEY,
  api_secret: CLOUDINARY_API_SECRET,
});

const wasmFile = path.resolve(__dirname, 'pkg/print_meta_bg.wasm');

cloudinary.uploader.upload(wasmFile, {
  use_filename: true,
  resource_type: 'raw',
  type: 'authenticated',
  invalidate: true,
}, (err, result) => {
  console.log(err, result);
});