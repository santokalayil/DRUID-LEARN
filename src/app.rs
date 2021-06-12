use druid::{AppLauncher, WindowDesc, Widget, PlatformError,};
use druid::im::{Vector, vector};

use crate::colors::{Color, DARK, };
pub use crate::data::appdata::AppState;


pub const MODE:Color = DARK;



pub struct App {}

impl App {
    pub fn new() -> Self {
        App{}
    }
    pub fn run (&self) -> Result<(), PlatformError>{
        let window_title = "Window Title";
        AppLauncher::with_window(
            WindowDesc::new(self.build_ui())
                .title(window_title)
                // .menu(crate::widgets::menu::generate_menu)
                .window_size(druid::Screen::get_display_rect().size() * 0.4 )
                // .set_position(
                //     (druid::Screen::get_display_rect().size() / 4.0).to_vec2().to_point()
                // )
                // .show_titlebar(false)
        ).launch(
            AppState{
                window_title: String::from(window_title),
                // vector: vector![1, 2],
                // vector: Vector::from(vec![1,2,3,4])
                vector: (0..50).collect(),
                families: crate::data::family_data::get_family_data_from_db(),
                checkbox_data: false
            }
        )?;
        Ok(())
    }

    pub fn build_ui(&self) -> impl Widget<AppState> {
        crate::body::view4::layout()
    }
}