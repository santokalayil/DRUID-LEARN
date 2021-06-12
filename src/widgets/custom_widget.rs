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

// =======================DERIVED FROM BUTTON============================


use druid::widget::prelude::*;
use druid::widget::{Click, ControllerHost, LabelText};
use druid::{Affine, Data, Insets, LinearGradient, UnitPoint};
use std::marker::PhantomData;

const LABEL_INSETS: Insets = Insets::uniform_xy(10., 4.); // 8., 2.

pub struct TileButton<T> {
    label: Label<T>,
    label_size: Size,
}

#[derive(Clone, Debug, PartialEq, Data)]
pub struct Key<T> {
    key: &'static str,
    value_type: PhantomData<T>,
}

const BORDERED_WIDGET_HEIGHT:f64 = 20. ;
const BUTTON_BORDER_WIDTH:f64 = 5.0;
const BUTTON_BORDER_RADIUS:f64 = 25.;
const DISABLED_BUTTON_LIGHT: Color = Color::YELLOW;
const DISABLED_BUTTON_DARK: Color = Color::GREEN;
const BUTTON_LIGHT: Color = Color::WHITE;
const BUTTON_DARK: Color = Color::GRAY;
const BORDER_LIGHT: Color = Color::GRAY;
const BORDER_DARK: Color = Color::BLACK;

impl<T: Data> TileButton<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> TileButton<T> {
        TileButton::from_label(Label::new(text))
    }
    pub fn from_label(label: Label<T>) -> TileButton<T> {
        TileButton {
            label,
            label_size: Size::ZERO,
        }
    }

    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        TileButton::new(text)
    }

    /// Provide a closure to be called when this button is clicked.
    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: Data> Widget<T> for TileButton<T> {
    // #[instrument(name = "Button", level = "trace", skip(self, ctx, event, _data, _env))]
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                    println!("Button {:?} pressed", ctx.widget_id());
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                    println!("Button {:?} released", ctx.widget_id());
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    // #[instrument(name = "Button", level = "trace", skip(self, ctx, event, data, env))]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    // #[instrument(name = "Button", level = "trace", skip(self, ctx, old_data, data, env))]
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    // #[instrument(name = "Button", level = "trace", skip(self, ctx, bc, data, env))]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Button");
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = BORDERED_WIDGET_HEIGHT;
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        let button_size = bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ));
        println!("Computed button size: {}", button_size);
        button_size
    }

    // #[instrument(name = "Button", level = "trace", skip(self, ctx, data, env))]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
        let size = ctx.size();
        let stroke_width = BUTTON_BORDER_WIDTH;

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(BUTTON_BORDER_RADIUS);

        let bg_gradient = if ctx.is_disabled() {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (
                    DISABLED_BUTTON_LIGHT,
                    DISABLED_BUTTON_DARK,
                ),
            )
        } else if is_active {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (BUTTON_DARK, BUTTON_LIGHT),
            )
        } else {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (BUTTON_LIGHT, BUTTON_DARK),
            )
        };

        let border_color = if is_hot && !ctx.is_disabled() {
            BORDER_LIGHT
        } else {
            BORDER_DARK
        };

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.fill(rounded_rect, &bg_gradient);

        let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}
