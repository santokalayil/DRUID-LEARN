// use druid::kurbo::BezPath;
// use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{WidgetExt, Color, Application, //Selector, TimerToken, 
};
// use druid::keyboard_types::Key;
use druid::widget::{Label, Flex, Painter};
use crate::app::AppState;
use crate::controllers;
use crate::app::MODE;
use crate::assets::logo;




pub fn generate_titlebar(label: String) -> impl Widget<AppState> {
    let title_bar_size = 20.0;
    let title_font_size = 12.0;
    let titlebar_painter = Painter::new(
        | ctx, _, _env | {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &MODE.primary_dark);
        }
    );
    let close_button_painter = Painter::new(
        | ctx, _, _env | {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &MODE.primary_dark);
            if ctx.is_hot(){
                ctx.fill(bounds, &Color::RED);
            }
        }
    );
    let restore_button_painter = Painter::new(
        | ctx, _, _env | {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &MODE.primary_dark);
            if ctx.is_hot(){
                ctx.fill(bounds, &MODE.primary_light);
            }
        }
    );
    Flex::row().with_spacer(1.0)
        .with_child(logo().fix_size(title_bar_size, title_bar_size).align_right())
        .with_flex_spacer(0.2)
        .with_flex_child(
            Label::new(label)
                .with_text_size(title_font_size)
                .align_left()
                .fix_height(title_bar_size)
                .background(titlebar_painter)
                // .padding(2.0)
                .controller(controllers::titlebar::TitlebarController {init_pos: None})
        , 1.0) //.padding(1.0)
        .with_child(
            Label::new("❐".to_string())
            .with_text_size(title_font_size).center()
            .fix_height(title_bar_size).fix_width(title_bar_size)
            
            .background(restore_button_painter)
            // .controller(controllers::restore_button::RestoreMaximizeController {maximized: false, mouse_down_and_active: false})
            // .on_click(move | ctx, _: &mut AppState, _ | {ctx.window().set_window_state(druid::WindowState::Maximized)})
            .on_click(move | _ctx, _: &mut AppState, _ | {println!("Maximize button is hot now")})
        )
        .with_child(
            Label::new("X".to_string())
            .with_text_size(title_font_size).center()
            .fix_height(title_bar_size).fix_width(title_bar_size)
            
            .background(close_button_painter)
            .on_click(move | ctx, _: &mut AppState, _ | {ctx.submit_command(druid::commands::CLOSE_WINDOW); Application::global().quit()})
            // Label::new("X").fix_height(30.0).fix_width(30.0).center().expand()
        )
}


