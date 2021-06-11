use druid::widget::{Label, Painter, WidgetExt};
use druid::{Widget, RenderContext,};
use crate::app::{AppState, MODE};

pub fn tool_tab(label: String, tool_text_size: f64) -> impl Widget<AppState>  {
    let tool_icon_size = tool_text_size.clone();
    let tool_tab_painter = Painter::new(
        | ctx, _, _env | {
            let stroke_width = 2.0;
    
            let rounded_rect = ctx.size()
                .to_rect()
                .inset(-stroke_width / 2.0)
                .to_rounded_rect(50.0);

            ctx.fill(rounded_rect, &MODE.tool_icon_bg);
            
            if ctx.is_hot(){
                ctx.fill(rounded_rect, &MODE.tool_icon_bg_active);
                ctx.stroke(rounded_rect, &MODE.tool_icon_border, stroke_width);
            }
        }
    );
    
    Label::new(label)
        .with_text_size(tool_text_size * 0.5).center() // aligning center within the circle
        .fix_height(tool_icon_size).fix_width(tool_icon_size) //.padding(5.0)
        .background(tool_tab_painter)
        .on_click(move | _ctx, _: &mut AppState, _ | { println!("Tab Tool Pressed");}) // .align_right()
}
