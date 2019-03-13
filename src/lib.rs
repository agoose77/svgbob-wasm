extern crate cfg_if;
extern crate wasm_bindgen;

extern crate svg;
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;
use std::io::{Error, ErrorKind};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};


mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Define external JS functions
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


// Rust functions
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, svgbob-wasm YAY!");
}


#[wasm_bindgen]
pub fn convert_string(data: String) -> String{
    let g = Grid::from_str(&*data, &Settings::default());
    let svg = g.get_svg();


    // Create fake "file"
    let mut file = Vec::new();


    if let Err(e) = svg::write(&mut file, &svg) {
        return String::from("Error occurred");
    }

    let mut out = Vec::new();
    let mut c = file.as_slice();
    c.read_to_end(&mut out).unwrap();

    String::from_utf8(out).unwrap()
}