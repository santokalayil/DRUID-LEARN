use druid::{Widget};
use druid::widget::{Flex, Label, Button};

pub fn layout() -> impl Widget<()> {
    // let titlebar = Flex::row().with_flex_child(Label::new("window title"), 1.0);
    Flex::column()
        .with_flex_child( // titlebar
            Flex::row()
                .with_flex_child(Label::new("1"), 1.0)
                .with_flex_child(Button::new("2"), 1.0)
        , 1.0)
        .with_flex_child( // body
            Flex::row()
                .with_flex_child(Label::new("3"), 1.0)
                .with_flex_child(Button::new("4"), 1.0)
        , 1.0)
}

