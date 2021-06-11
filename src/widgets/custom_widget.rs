use druid::{Widget, WidgetExt, theme, RenderContext, Color, }; // Application, Event
use druid::widget::{Label, Painter, }; // Flex, 

use crate::app::AppState;

pub fn custom_label(label: String) -> impl Widget<AppState> {
    let painter = Painter::new(
        | ctx, _, env | {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));
            // ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
            if ctx.is_hot(){
                ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0)
            }
            if ctx.is_active(){
                ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
            }
            
        }
    );
    Label::new(label)
        .with_text_size(24.0)
        .center()
        .background(painter)
        .expand()
        .on_click(move | _, data: &mut AppState, _ | data.custom_function())
}