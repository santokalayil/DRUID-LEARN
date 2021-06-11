// use druid::widget::prelude::*;
// // use druid::{Point};

// use druid::widget::{Controller,};

// pub struct RestoreMaximizeController {
//     pub maximized: bool, 
//     pub mouse_down_and_active: bool,
// }

// impl<T, W: Widget<T>> Controller<T, W> for RestoreMaximizeController {
//     fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
//         match event {
            
//             Event::MouseDown(me) if me.buttons.has_left() => {
//                 ctx.set_active(true)
//                 // self.mouse_down_and_active = true
//             }
//             Event::MouseUp(me) if ctx.is_active() && me.buttons.has_left() => {
//                 ctx.window().set_window_state(druid::WindowState::Maximized);
//                 ctx.set_active(false)

//                 }
//             }
//             // _ => (),
//             // _ => child.event(ctx, event, data, env)
//         }
//         // child.event(ctx, event, data, env)
//     }

