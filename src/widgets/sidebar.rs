use druid::{
    Widget, WidgetExt, RenderContext, 
    UnitPoint, Color, Data, Lens,
}; // Application, Event
use druid::widget::{
    Label, Painter, Flex, Scroll, List, Button,
    CrossAxisAlignment
}; // Flex, 
use druid::im::Vector;
use druid::LensExt;

use crate::app::AppState;
use crate::app::MODE;

// #[derive(Clone, Data, Lens)]
// struct AppData {
//     left: Vector<u32>,
//     right: Vector<u32>,
//     l_index: usize,
//     r_index: usize,
// }

pub fn generate_sidebar(label: String) -> impl Widget<AppState> {
    let painter = Painter::new(
        | ctx, _, _env | {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &MODE.primary_dark);
            // ctx.stroke(bounds.inset(-0.5), &MODE.frame_border, 1.0)
            // ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
            // if ctx.is_hot(){
            //     ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0)
            // }
            // if ctx.is_active(){
            //     ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
            // }
            
        }
    );
    let test_label = Label::new(label)
        .with_text_size(24.0)
        .center()
        .background(painter)
        .expand()
        .on_click(move | _, data: &mut AppState, _ | data.custom_function());
    let mut lists = Flex::column();
        // .with_flex_child(test_label, 1.0);
    lists.add_flex_child(
        Scroll::new(
            List::new(|| 
                {
                    crate::widgets::list_tile::Label::new(|number: &u32, _env: &_| format!("List item #{}", number))
                        .with_text_size(10.0)
                        .with_text_color(MODE.tool_icon_bg_active)
                        // .with_text_alignment(druid::TextAlignment::End)
                        .align_vertical(UnitPoint::LEFT)
                        .padding(5.0)
                        .expand()
                        .height(30.0)
                        // .background(painter) // NOT WORKING
                        // .background(MODE.primary_dark) // Color::rgb(0.5, 0.5, 0.5)
                        // .on_click(move | _, data: &mut AppState, _ | data.custom_function())
                }
            )  
        )
        .vertical()
        .background(MODE.tool_icon_bg)
        .lens(AppState::vector),
        1.0,
    );
    lists

    // let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    // // Build a simple list
    // lists.add_flex_child(
    //     Scroll::new(List::new(|| {
    //         Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
    //             .align_vertical(UnitPoint::LEFT)
    //             .padding(10.0)
    //             .expand()
    //             .height(50.0)
    //             .background(Color::rgb(0.5, 0.5, 0.5))
    //             // .on_click(move | _, data: &mut AppState, _ | data.custom_function())
    //     }))
    //     .vertical()
    //     .lens(AppData::left),
    //     1.0,
    // );

    // // Build a list with shared data
    // lists.add_flex_child(test_label, 1.0);
    // lists.add_flex_child(
    //     Scroll::new(
    //         List::new(|| {
    //             Flex::row()
    //                 .with_child(
    //                     Label::new(|(_, item): &(Vector<u32>, u32), _env: &_| {
    //                         format!("List item #{}", item)
    //                     })
    //                     .align_vertical(UnitPoint::LEFT),
    //                 )
    //                 .with_flex_spacer(1.0)
    //                 .with_child(
    //                     Button::new("Delete")
    //                         .on_click(|_ctx, (shared, item): &mut (Vector<u32>, u32), _env| {
    //                             // We have access to both child's data and shared data.
    //                             // Remove element from right list.
    //                             shared.retain(|v| v != item);
    //                         })
    //                         .fix_size(80.0, 20.0)
    //                         .align_vertical(UnitPoint::CENTER),
    //                 )
    //                 .padding(10.0)
    //                 .background(Color::rgb(0.5, 0.0, 0.5))
    //                 .fix_height(50.0)
    //         })
    //         .with_spacing(10.),
    //     )
    //     .vertical()
    //     .lens(druid::lens::Identity.map(
    //         // Expose shared data with children data
    //         |d: &AppData| (d.right.clone(), d.right.clone()),
    //         |d: &mut AppData, x: (Vector<u32>, Vector<u32>)| {
    //             // If shared data was changed reflect the changes in our AppData
    //             d.right = x.0
    //         },
    //     )),
    //     1.0,
    // );

    // lists
}