mod app;
pub mod body;
pub mod widgets;
pub mod controllers;
pub mod colors;
pub mod assets;
pub mod data;
pub mod functions;

// #[macro_use]
// extern crate rust_embed;


fn main() {
    let _ = crate::app::App::new().run();
}