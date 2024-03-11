mod utils;

use svgbob::to_svg_with_settings;
use svgbob::Settings;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Debug)]
pub struct SVGRenderer {
    pub font_size: usize,
    pub font_family: String,
    pub fill_color: String,
    pub background: String,
    pub stroke_color: String,
    pub stroke_width: f32,
    pub scale: f32,
}

impl From<SVGRenderer> for Settings {
    fn from(value: SVGRenderer) -> Self {
        Settings {
            font_size: value.font_size,
            font_family: value.font_family,
            fill_color: value.fill_color,
            background: value.background,
            stroke_color: value.stroke_color,
            stroke_width: value.stroke_width,
            scale: value.scale,
            ..Settings::default()
        }
    }
}

#[wasm_bindgen]
impl SVGRenderer {
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            font_size: 14,
            font_family: "Iosevka Fixed, monospace".into(),
            fill_color: "black".into(),
            background: "transparent".into(),
            stroke_color: "black".into(),
            stroke_width: 2.0,
            scale: 8.0,
        }
    }

    pub fn font_size(mut self, font_size: usize) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn font_family(mut self, font_family: &str) -> Self {
        self.font_family = font_family.into();
        self
    }

    pub fn fill_color(mut self, fill_color: &str) -> Self {
        self.fill_color = fill_color.into();
        self
    }

    pub fn background(mut self, background: &str) -> Self {
        self.background = background.into();
        self
    }

    pub fn stroke_color(mut self, stroke_color: &str) -> Self {
        self.stroke_color = stroke_color.into();
        self
    }

    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn render(self, ascii: &str) -> String {
        let settings = Settings::from(self);
        to_svg_with_settings(ascii, &settings)
    }
}

#[wasm_bindgen]
pub fn renderer() -> SVGRenderer {
    SVGRenderer::default()
}

#[wasm_bindgen]
pub fn render(ascii: &str) -> String {
    let settings = Settings {
        background: "transparent".into(),
        ..Settings::default()
    };

    to_svg_with_settings(ascii, &settings)
}
