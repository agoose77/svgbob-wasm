mod utils;

use svgbob::to_svg_with_settings;
use svgbob::Settings;

use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn render(ascii: &str) -> String{
    let settings = Settings {
        background: "transparent".into(),
        ..Settings::default()
    };

    to_svg_with_settings(ascii, &settings)
}
