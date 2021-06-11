use druid::{Widget, WidgetExt, }; // theme, RenderContext, Color, Application, Event
use druid::widget::{Flex, Label, CrossAxisAlignment, }; // Painter, 

use crate::app::AppState;
use crate::widgets::titlebar::generate_titlebar;
use crate::widgets::custom_widget::custom_label;


pub fn layout() -> impl Widget<AppState> {

    Flex::column()
        .with_child(generate_titlebar(String::from("santo")))  // TitleBar("TITLEBAR".to_string())
        .with_flex_spacer(0.2)
        .with_child(Label::new("DISPLAY").with_text_size(32.0).padding(5.0))
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child( // body
            Flex::row()
                .with_flex_child(custom_label("H".to_string()), 1.0)
                .with_spacer(1.0)
                .with_flex_child(custom_label("V".to_string()), 1.0)
                .with_spacer(1.0)
                .with_flex_child(custom_label("X".to_string()), 1.0)
                .with_spacer(1.0)
                .with_flex_child(custom_label("Z".to_string()), 1.0)
        , 1.0)
}

