extern crate cfg_if;
extern crate wasm_bindgen;

extern crate svg;
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;


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

#[wasm_bindgen]
pub fn convert_string(data: String) -> String{
    let g = Grid::from_str(&*data, &Settings::default());
    let svg = g.get_svg();
    let mut file = Vec::new();

    // Return empty string if error occurred during writing
    if let Err(_e) = svg::write(&mut file, &svg) {
        return String::new();
    }

    String::from_utf8(file).unwrap()
}