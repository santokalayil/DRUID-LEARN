use druid::{Widget, WidgetExt,
    //  Data, RenderContext, Color, Lens, im
    }; // theme, RenderContext, Color, Application, Event, ImageBuf
use druid::widget::{Flex, Label, Switch, 
    // Painter, CrossAxisAlignment, 
    MainAxisAlignment}; // Image, Svg, SvgData, CrossAxisAlignment 

use crate::app::AppState;
use crate::widgets::{
    titlebar::generate_titlebar,
    split::Split,
    sidebar::generate_sidebar,
    frame::generate_frame,
};
// use crate::assets::logo;
// use crate::app::MODE;
use crate::widgets::tool_tab::tool_tab;

const TOOLBAR_SIZE: f64 = 20.0;




pub fn layout() -> impl Widget<AppState> {
    Flex::column()
        .with_child(generate_titlebar(String::from(" ECCLESIASTICA")))  
        .with_child( // toolbar section with tabs
            Flex::row()
                .with_child(Label::new("DIRECTORY").with_text_size(TOOLBAR_SIZE).padding(5.0)) //.cross_axis_alignment(CrossAxisAlignment::Start)
                // .with_flex_spacer(1.0)
                .with_child(tool_tab("↻".to_string(), TOOLBAR_SIZE))
                .with_flex_child(Switch::new().lens(AppState::checkbox_data), 1.0).main_axis_alignment(MainAxisAlignment::End)
                // .with_spacer(15.0)
        )
        .with_flex_child( // body
            Split::columns(
                generate_sidebar(), 
                generate_frame(String::from("Split B"))
            )
                .bar_size(1.0)
                .split_point(0.15)
                .draggable(true)
                .solid_bar(true)
                .min_size(50.0, 100.0)
                .min_bar_area(0.0)
                .expand()
        , 1.0)
        .with_child(Label::new("DISPLAY").with_text_size(10.0).padding(1.0))
}

