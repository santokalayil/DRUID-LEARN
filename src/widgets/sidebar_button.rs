use druid::{Widget, Env};
use druid::{EventCtx, Event}; // event
use druid::{LifeCycleCtx, LifeCycle}; // lifecycle
use druid::{UpdateCtx, };  // update
use druid::{LayoutCtx, BoxConstraints, Size}; // layout
use druid::{PaintCtx, }; // paint
use druid::RenderContext; 

use crate::colors::sidebarbtn::DARK as THEME;

pub struct SidebarButton{
    pub size: Size,
    pub label_text: String,
}

use crate::data::family_data::Family;
impl Widget<Family>  for SidebarButton {

    fn event(&mut self, ctx : &mut EventCtx, event: &Event, _data : &mut Family, _env: &Env) { // here data is mutable
        println!("{:?}",event);
        match event {
            Event::MouseDown(_mouse) => {
                if ctx.is_hot() && !ctx.is_disabled() {
                    ctx.set_active(true);
                    // data.background = THEME.btn_bg_active;
                    println!("Button {:?} pressed", ctx.widget_id());
                    ctx.request_paint(); // ctx.set_handled();
                }
            }
            Event::MouseUp(_mouse) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    // data.background = THEME.btn_bg;
                    println!("Button {:?} released", ctx.widget_id());
                    ctx.request_paint(); // ctx.set_handled();
                }
                ctx.set_active(false);
            }
            _ => {
                if ctx.is_hot()  {
                    // data.background = THEME.btn_bg_hot;
                    println!("Button {:?} hot", ctx.widget_id());
                    ctx.request_paint();
                } else
                if !ctx.is_hot() {
                    // data.background = THEME.btn_bg;
                    ctx.request_paint();
                }
            },
        }
    }
    fn lifecycle(&mut self, _ctx : &mut LifeCycleCtx, _event: &LifeCycle, _data : &Family, _env: &Env) { // here data is not mutable

    }
    fn update(&mut self, _ctx : &mut UpdateCtx, _old_data: &Family, _data : &Family, _env: &Env) {
        // ctx.request_update()
    }

    fn layout(&mut self, _ctx : &mut LayoutCtx, bc: &BoxConstraints, _data : &Family, _env: &Env) -> Size { // here bc is not unused
        if bc.is_width_bounded() | bc.is_height_bounded() {
            // let size = Size::new(100.0, 100.0);
            bc.constrain(self.size)
        } else {
            bc.max()
        }
        // self.size = bc.max();
        // self.size
    }

    fn paint(&mut self, ctx : &mut PaintCtx, _data : &Family, _env: &Env) { // here ctx and data are not unused 
        let size = ctx.size(); let rect = size.to_rect();
        // ctx.fill(rect, &data.background);
        // ctx.stroke(rect, &data.stroke, 1.0);
        if ctx.is_hot() {
            ctx.stroke(rect.inset(-0.5), &THEME.btn_bg_hot, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(rect, &THEME.btn_bg_active);
        }

        use druid::piet::{Text, TextLayoutBuilder, FontFamily};
        let text = ctx.text();
        let text_label = text
            .new_text_layout(format!("{}", &self.label_text))
            .font(FontFamily::SANS_SERIF, 10.0)
            // .text_color(data.foreground.clone())
            .build()
            .unwrap();
        ctx.draw_text(&text_label, (size.width * 0.3, size.height * 0.32));

        let text = ctx.text();
        let text_label = text
            .new_text_layout("[TICK]")
            .font(FontFamily::SANS_SERIF, 10.0)
            // .text_color(data.foreground.clone())
            .build()
            .unwrap();
        ctx.draw_text(&text_label, (size.width * 0.1, size.height * 0.32));
    }

}