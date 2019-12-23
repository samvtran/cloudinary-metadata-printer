use wasm_bindgen::prelude::*;

extern crate byteorder;
extern crate image;
extern crate serde;
extern crate serde_json;

use byteorder::{BigEndian, WriteBytesExt};
use std::mem;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{FontCollection, Scale};

pub fn write_text(x: Vec<String>, width: u32, height: u32, bytes: Vec<u8>) -> Vec<u8> {
    let mut img = RgbaImage::from_raw(width, height, bytes).unwrap();

    let font = Vec::from(include_bytes!("SourceCodePro-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();
    let height = 12.0;
    for (idx, str) in x.iter().enumerate() {
        let scale = Scale {
            x: height * 1.5,
            y: height,
        };

        draw_text_mut(
            &mut img,
            Rgba([130u8, 255u8, 0u8, 1u8]),
            width / 2,
            (height * (idx as f32 + 1.0)) as u32,
            scale,
            &font,
            str,
        );
    }

    img.into_raw()
}


#[wasm_bindgen]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr
}

#[wasm_bindgen]
pub extern "C" fn dealloc(ptr: *mut u8, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[wasm_bindgen]
pub extern "C" fn transform(
    width: u32,
    height: u32,
    image_ptr: *mut u8,
    meta_ptr: *mut u8,
    meta_size: usize,
) -> u32 {
    let size = (width * height * 4) as usize;
    let bytes = unsafe { Vec::from_raw_parts(image_ptr, size, size) };

    let meta_bytes = unsafe { Vec::from_raw_parts(meta_ptr, meta_size, meta_size) };
    let metadata: serde_json::Value =
        serde_json::from_slice(&meta_bytes).expect("Failed to deserialize metadata json");

    let mut out_buffer = write_text(
        serde_json::to_string_pretty(&metadata).unwrap().split('\n').map(|s| s.to_string()).collect(),
        width,
        height,
        bytes,
    );

    let mut dims = vec![];
    let _ = dims.write_u32::<BigEndian>(width);
    let _ = dims.write_u32::<BigEndian>(height);
    dims.append(&mut out_buffer);
    let out_buffer = dims;
    let out_ptr = out_buffer.as_ptr() as u32;
    mem::forget(out_buffer);
    out_ptr
}