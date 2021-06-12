use druid::{Widget, WidgetExt, Data, RenderContext, Color, Lens, im, LocalizedString}; // theme, RenderContext, Color, Application, Event, ImageBuf
use druid::widget::{Flex, List, Scroll, Label, Button, Switch, Painter, CrossAxisAlignment, MainAxisAlignment}; // Image, Svg, SvgData, CrossAxisAlignment 

// const TOOLBAR_SIZE: f64 = 20.0;
use crate::data::appdata::AppState;
use crate::app::MODE;




pub fn layout() -> impl Widget<AppState> {
    const FONTSIZE: f64 = 10.0;
    use crate::widgets::list_tile;
    let mut scrollframe = Flex::column()
        .with_child(list_tile::tile_title_generate("SECTIONS"));
    use crate::data::family_data::FamilyData;
    use druid::widget::Button;
    
    fn fam_item() -> impl Widget<FamilyData> {
        let item_label = Label::raw()
            .lens(FamilyData::head_of_family);
        let row = Flex::row()
            .with_spacer(5.0)
            .with_flex_child(Label::new("👪")
                .with_text_size(FONTSIZE)
                .with_text_color(MODE.side_bar_item_icon_color),
                1.0
            )
            .with_flex_child(item_label, 1.0);
        row       
    }
        // .with_flex_child(test_label, 1.0);
    scrollframe.add_flex_child(
        Scroll::new(
            List::new(fam_item) //.lens(Color)/ }
        )
        .vertical()
        .background(MODE.tool_icon_bg)
        .lens(AppState::families),
        1.0,
    );
    scrollframe
}
