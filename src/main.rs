mod app;
pub mod body;
pub mod widgets;
pub mod controllers;
pub mod colors;
pub mod assets;

// #[macro_use]
// extern crate rust_embed;


fn main() {
    let _ = crate::app::App::new().run();
}