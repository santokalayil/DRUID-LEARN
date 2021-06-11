use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens};
use crate::colors::{Color, DARK, }; // LIGHT

use druid::im::{Vector, vector};

pub const MODE:Color = DARK;

#[derive(Clone, Data, Lens)]
pub struct AppState { 
    pub window_title : String,
    pub vector: Vector<u32>,
}
impl AppState {
    pub fn custom_function(&mut self) {
        // println!("TEST_DATA : {:?}", self.ctx.window().get_position());
        println!("custom function");
        
    
    }
    // pub fn maximize_window(&mut self) {
    //     self.ctx.window().set_window_state(druid::WindowState::Maximized);
    //     println!("Window is maximized");
        
    
    // }
}


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
                .window_size(druid::Screen::get_display_rect().size() * 0.9 )
                // .set_position(
                //     (druid::Screen::get_display_rect().size() / 4.0).to_vec2().to_point()
                // )
                .show_titlebar(false)
        ).launch(
            AppState{
                window_title: String::from(window_title),
                // vector: vector![1, 2],
                // vector: Vector::from(vec![1,2,3,4])
                vector: (0..50).collect()
            }
        )?;
        Ok(())
    }

    pub fn build_ui(&self) -> impl Widget<AppState> {
        crate::body::view3::layout()
    }
}