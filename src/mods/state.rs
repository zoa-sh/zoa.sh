// PATH: src/mods/state.rs
use css_minify::optimizations::{Level, Minifier};
use std::fs;
use std::sync::Arc;

// Minify CSS
pub struct AppState {
    pub minified_css: Arc<String>,
}

pub fn init_app_state() -> AppState {
    let css_content = fs::read_to_string("static/css/styles.css").expect("Failed to read CSS file");
    let minified_css = minify_css(&css_content);

    AppState {
        minified_css: Arc::new(minified_css),
    }
}

fn minify_css(input: &str) -> String {
    let mut minifier = Minifier::default();
    minifier
        .minify(input, Level::One)
        .expect("Failed to minify CSS")
}
