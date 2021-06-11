use druid::widget::prelude::*;
use druid::{ Point //Selector, TimerToken, 
};
// use druid::keyboard_types::Key;
use druid::widget::{Controller,};
// use crate::app::AppState;

pub struct TitlebarController {
    pub init_pos: Option<Point>, 
}

impl<T, W: Widget<T>> Controller<T, W> for TitlebarController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            // Event::MouseDown(event.clone().into())
            // Event::KeyDown(k) if k.key == Key::Enter => {
            //     ctx.submit_command(druid::commands::UNDO);
            //     println!("Controller started working. now KeyDown")
            // }
            // Event::KeyUp(k) if k.key == Key::Enter => {
            //     ctx.submit_command(druid::commands::UNDO);
            //     child.event(ctx, event, data, env); // if muted the ctx then use this
            //     println!("Controller started working. now KeyUp")
            // }
            // ============================================================================
            // Event::MouseDown(_) => {
            //     if !ctx.is_disabled() {
            //         ctx.set_active(true);
            //         // ctx.request_paint();
            //         // println!("Button {:?} pressed", ctx.widget_id());
            //     }
            // }

            // Event::MouseUp(_) => {
            //     if ctx.is_active() && !ctx.is_disabled() {
            //         // ctx.request_paint();
            //         // println!("Button {:?} released", ctx.widget_id());
            //     }
            //     ctx.set_active(false);
            // }
            // Event::MouseMove(me) if ctx.is_hot() => {
            //     // TODO another timer on leaving
            //     // println!("Current Position of Mouse Cursor is:  {:?}", me.window_pos);
            //     // ctx.submit_command(druid::commands::CLOSE_WINDOW.to(*win_id));
            //     // Some(TooltipState::Waiting {
            //     //     last_move: now,
            //     //     timer_expire: now + wait_duration,
            //     //     token: ctx.request_timer(wait_duration),
            //     //     window_pos: me.window_pos,
            //     // })
            // }
            // Event::MouseMove(me) if ctx.is_active() => { // DRAGGING WINDOW using titlebar
            //     ctx.window().get_position();
            //     println!("Mouse Cursor at {:?}\nWindow at {:?}", me.window_pos, ctx.window().get_position());
            //     ctx.window().set_position((ctx.window().get_position().to_vec2()+me.window_pos.to_vec2()).to_point());
            // }
            Event::MouseDown(me) if me.buttons.has_left() => {
                ctx.set_active(true);
                self.init_pos = Some(me.window_pos)
            }
            Event::MouseMove(me) if ctx.is_active() && me.buttons.has_left() => {
                if let Some(init_pos) = self.init_pos {
                    let within_window_change = me.window_pos.to_vec2() - init_pos.to_vec2();
                    let old_pos = ctx.window().get_position();
                    let new_pos = old_pos + within_window_change;
                    // println!(
                    //     "Drag {:?} ",
                    //     (
                    //         init_pos,
                    //         me.window_pos,
                    //         within_window_change,
                    //         old_pos,
                    //         new_pos
                    //     )
                    // );
                    ctx.window().set_position(new_pos)
                }
            }
            Event::MouseUp(_me) if ctx.is_active() => {
                self.init_pos = None;
                ctx.set_active(false)
            }
            // _ => (),
            _ => child.event(ctx, event, data, env)
        }
        // child.event(ctx, event, data, env)
    }
}
