use std::sync::Arc;

use druid::{
    widget::{Controller, ControllerHost, Flex, Label, List, ListIter, Painter, Scroll, SizedBox,},
    Color, Command, Data, Env, Event, RenderContext, Selector, Target, Widget, WidgetExt,
};

use crate::data::appdata::AppState;
use crate::data::family_data::Family;
use crate::colors::THEME;

impl ListIter<(Family, Option<usize>, usize)> for AppState {
    fn for_each(&self, mut cb: impl FnMut(&(Family, Option<usize>, usize), usize),) 
    {
        for (i, family) in self.families.iter().enumerate() {
            cb(&(family.clone(), self.selected, i), i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut (Family, Option<usize>, usize), usize),) 
    {
        let mut any_changed = false;
        let mut families = Vec::with_capacity(self.families.len());
        for (i, family) in self.families.iter().enumerate() {
            let mut list_item = (family.clone(), self.selected, i);
            cb(&mut list_item, i);
            if !list_item.0.same(family) {
                any_changed = true;
            }
            families.push(list_item.0);
        }
        if any_changed {
            self.families = Arc::new(families);
        }
    }

    fn data_len(&self) -> usize { self.families.len()}
}

pub fn generate_sidebar() -> impl Widget<AppState> {
    let list = Scroll::new(
        List::new(|| {
            let head_of_family = Label::new(
                |(family, _selected, _idx): &(
                    Family,
                    Option<usize>,
                    usize,
                ),
                 _env: &Env| { family.head_of_family.clone() },
            )
            .with_text_size(12.)
            .with_text_color(THEME.side_bar_item_text_color);
            let famid = Label::new(
                |(family, _selected, _idx): &(
                    Family,
                    Option<usize>,
                    usize,
                ),
                 _env: &Env| {
                    format!("{} -> ", family.famid)
                },
            )
            .with_text_size(12.)
            .with_text_color(THEME.side_bar_item_text_color);
            let layout = Flex::row().with_child(famid).with_child(head_of_family);
            let paint = Painter::new(|ctx, (_familiy, selected, idx), _env| {
                let is_hot = ctx.is_hot();
                let is_selected = if let Some(index) = selected {
                    index == idx
                } else {
                    false
                };
                let is_active = ctx.is_active();
                let stroke_color = if is_selected {
                    Color::rgb8(0x88, 0x88, 0x88)
                } else if is_active {
                    Color::rgb8(0x66, 0x66, 0x66)
                } else if is_hot {
                    Color::rgb8(0xbb, 0xbb, 0xbb)
                } else {
                    Color::rgb8(0x22, 0x22, 0x22)
                };

                let background_color = if is_selected {
                    THEME.side_bar_item_bg_selected
                } else if is_active {
                    THEME.side_bar_item_bg_active
                } else if is_hot {
                    THEME.side_bar_item_bg_hot
                } else {
                    THEME.side_bar_item_bg
                };
                // let rect = druid::kurbo::Size::new(1000. , 20.).to_rect() ; // 
                let rect = ctx.size().to_rect(); 
                // let rect = &size.to_rect();
                ctx.stroke(rect, &stroke_color, 1.);
                ctx.fill(rect, &background_color);
            });
            layout.padding(1.).background(paint).on_click(
                |event, (_familiy, _selected, idx), _env| {
                    event.submit_command(Command::new(
                        CHANGE_SELECTED,
                        *idx,
                        Target::Auto,
                    ));
                },
            )
        })
        .with_spacing(5.),
    )
    .expand_width();
    ControllerHost::new(list, AppController)
}

struct AppController;

impl Controller<AppState, SizedBox<AppState>> for AppController {
    fn event(
        &mut self,
        child: &mut SizedBox<AppState>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(selector) if selector.is(CHANGE_SELECTED) => {
                let selected = selector.get_unchecked(CHANGE_SELECTED);
                data.selected = Some(*selected);
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

const CHANGE_SELECTED: Selector<usize> =
    Selector::new("my_program-change_selected_item");
