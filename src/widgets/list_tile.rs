use druid::widget::{Flex, Label};
use druid::{Widget, WidgetExt};
use crate::data::appdata::AppState;
use crate::data::family_data::FamilyData;
use crate::app::MODE;

const FONTSIZE: f64 = 10.0;

pub fn family_tile_generate(
    main_text: druid::widget::LensWrap<FamilyData, String, crate::data::family_data::family_data_derived_lenses::head_of_family, druid::widget::RawLabel<std::string::String>>,
    icon:&str,
    left_spacer_size: f64
 )
-> impl Widget<FamilyData> {
    Flex::row()
        .with_spacer(left_spacer_size)
        .with_flex_child(Label::new(icon).with_text_size(FONTSIZE), 1.0)
        .with_flex_child(main_text, 1.0)
        // .with_flex_child(Label::new("=>").with_text_size(FONTSIZE), 1.0)
        .background(MODE.primary_light)
}

pub fn tile_title_generate(heading: &str) -> impl Widget<AppState> {
    Flex::row()
    .with_spacer(2.0)
    .with_flex_child(Label::new("★").with_text_size(14.0), 1.0)
    .with_flex_child(Label::new(heading).with_text_size(10.0), 1.0)
    // .with_flex_child(Label::new("=>").with_text_size(FONTSIZE), 1.0)
    .background(MODE.side_bar_title_heading_bg)
}