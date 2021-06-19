use druid::{Widget, WidgetExt, Data, RenderContext, Color, Lens, im, LocalizedString}; // theme, RenderContext, Color, Application, Event, ImageBuf
use druid::widget::{Flex, List, Scroll, Label, Button, Switch, Painter, CrossAxisAlignment, MainAxisAlignment}; // Image, Svg, SvgData, CrossAxisAlignment 

// const TOOLBAR_SIZE: f64 = 20.0;
use crate::data::appdata::AppState;
use crate::app::MODE;




pub fn layout() -> impl Widget<AppState> {
    const FONTSIZE: f64 = 10.0;
    use crate::widgets::list_tile;
    use druid::Env;
    use druid::LensExt;

    let initial_data = AppState{
        window_title: String::from("THIS IS"),
        // vector: vector![1, 2],
        // vector: Vector::from(vec![1,2,3,4])
        vector: (0..50).collect(),
        families: crate::data::family_data::get_family_data_from_db(),
        checkbox_data: false
    };

    let mut scrollframe = Flex::column()
        .with_child(list_tile::tile_title_generate("SECTIONS"));
    use crate::widgets::responsible_tilebutton::TileButton;

    // tiles = 
    println!("{:?}", &initial_data.families);
    let mut list = Flex::column();
    for i in 0..initial_data.families.len() {
        let button: TileButton<AppState> = TileButton::new(move |data: &AppState, _: &Env| format!("{:?}", 
        data.families.get(i).unwrap().head_of_family)); 
        list.add_flex_child(button, 1.0);
    }
    
    
    
        // .with_flex_child(test_label, 1.0);
    scrollframe.add_flex_child(
        Scroll::new(
            list
        )
        .vertical()
        .background(MODE.tool_icon_bg),
        // .lens(AppState::families),
        1.0,
    );
    scrollframe
}
