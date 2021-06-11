use std::ops::{Deref, DerefMut};

use crate::app::MODE;

// use druid_shell::Cursor;

use druid::kurbo::Vec2;
use druid::text::TextStorage;
use druid::widget::prelude::*;
use druid::{
    ArcStr, Color, Data, FontDescriptor, KeyOrValue, LocalizedString, Point, TextAlignment,
    TextLayout,
};

// added padding between the edges of the widget and the text.
const LABEL_X_PADDING: f64 = 2.0;

pub struct Label<T> {
    label: RawLabel<ArcStr>,
    current_text: ArcStr,
    text: LabelText<T>,
    // for debuging, we track if the user modifies the text and we don't get
    // an update call, which might cause us to display stale text.
    text_should_be_updated: bool,
}


pub struct RawLabel<T> {
    layout: TextLayout<T>,
    line_break_mode: LineBreaking,

    disabled: bool,
    default_text_color: KeyOrValue<Color>,
}

/// Options for handling lines that are too wide for the label.
#[derive(Debug, Clone, Copy, PartialEq, Data)]
pub enum LineBreaking {
    /// Lines are broken at word boundaries.
    WordWrap,
    /// Lines are truncated to the width of the label.
    Clip,
    /// Lines overflow the label.
    Overflow,
}


pub enum LabelText<T> {
    Localized(LocalizedString<T>),
    Static(Static),
    Dynamic(Dynamic<T>),
}

/// Text that is computed dynamically.
pub struct Dynamic<T> {
    f: Box<dyn Fn(&T, &Env) -> String>,
    resolved: ArcStr,
}

/// Static text.
pub struct Static {
    /// The text.
    string: ArcStr,
    /// Whether or not the `resolved` method has been called yet.
    ///
    /// We want to return `true` from that method when it is first called,
    /// so that callers will know to retrieve the text. This matches
    /// the behaviour of the other variants.
    resolved: bool,
}

impl<T: TextStorage> RawLabel<T> {
    /// Create a new `RawLabel`.
    pub fn new() -> Self {
        Self {
            layout: TextLayout::new(),
            line_break_mode: LineBreaking::Overflow,
            disabled: false,
            default_text_color: druid::theme::TEXT_COLOR.into(),
        }
    }

    /// Builder-style method for setting the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn with_text_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_text_color(color);
        self
    }

    /// Builder-style method for setting the text size.
    ///
    /// The argument can be either an `f64` or a [`Key<f64>`].
    ///
    /// [`Key<f64>`]: ../struct.Key.html
    pub fn with_text_size(mut self, size: impl Into<KeyOrValue<f64>>) -> Self {
        self.set_text_size(size);
        self
    }

    /// Builder-style method for setting the font.
    ///
    /// The argument can be a [`FontDescriptor`] or a [`Key<FontDescriptor>`]
    /// that refers to a font defined in the [`Env`].
    ///
    /// [`Env`]: ../struct.Env.html
    /// [`FontDescriptor`]: ../struct.FontDescriptor.html
    /// [`Key<FontDescriptor>`]: ../struct.Key.html
    pub fn with_font(mut self, font: impl Into<KeyOrValue<FontDescriptor>>) -> Self {
        self.set_font(font);
        self
    }

    /// Builder-style method to set the [`LineBreaking`] behaviour.
    ///
    /// [`LineBreaking`]: enum.LineBreaking.html
    pub fn with_line_break_mode(mut self, mode: LineBreaking) -> Self {
        self.set_line_break_mode(mode);
        self
    }

    /// Builder-style method to set the [`TextAlignment`].
    ///
    /// [`TextAlignment`]: enum.TextAlignment.html
    pub fn with_text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.set_text_alignment(alignment);
        self
    }

    /// Set the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn set_text_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        let color = color.into();
        if !self.disabled {
            self.layout.set_text_color(color.clone());
        }
        self.default_text_color = color;
    }

    /// Set the text size.
    ///
    /// The argument can be either an `f64` or a [`Key<f64>`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Key<f64>`]: ../struct.Key.html
    pub fn set_text_size(&mut self, size: impl Into<KeyOrValue<f64>>) {
        self.layout.set_text_size(size);
    }

    /// Set the font.
    ///
    /// The argument can be a [`FontDescriptor`] or a [`Key<FontDescriptor>`]
    /// that refers to a font defined in the [`Env`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Env`]: ../struct.Env.html
    /// [`FontDescriptor`]: ../struct.FontDescriptor.html
    /// [`Key<FontDescriptor>`]: ../struct.Key.html
    pub fn set_font(&mut self, font: impl Into<KeyOrValue<FontDescriptor>>) {
        self.layout.set_font(font);
    }

    /// Set the [`LineBreaking`] behaviour.
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`LineBreaking`]: enum.LineBreaking.html
    pub fn set_line_break_mode(&mut self, mode: LineBreaking) {
        self.line_break_mode = mode;
    }

    /// Set the [`TextAlignment`] for this layout.
    ///
    /// [`TextAlignment`]: enum.TextAlignment.html
    pub fn set_text_alignment(&mut self, alignment: TextAlignment) {
        self.layout.set_text_alignment(alignment);
    }

    /// Draw this label's text at the provided `Point`, without internal padding.
    ///
    /// This is a convenience for widgets that want to use Label as a way
    /// of managing a dynamic or localized string, but want finer control
    /// over where the text is drawn.
    pub fn draw_at(&self, ctx: &mut PaintCtx, origin: impl Into<Point>) {
        self.layout.draw(ctx, origin)
    }

    /// Return the offset of the first baseline relative to the bottom of the widget.
    pub fn baseline_offset(&self) -> f64 {
        let text_metrics = self.layout.layout_metrics();
        text_metrics.size.height - text_metrics.first_baseline
    }
}

impl<T: TextStorage> Label<T> {
    /// Create a new [`RawLabel`].
    ///
    /// This can display text `Data` directly.
    pub fn raw() -> RawLabel<T> {
        RawLabel::new()
    }
}

impl<T: Data> Label<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> Self {
        let text = text.into();
        let current_text = text.display_text();
        Self {
            text,
            current_text,
            label: RawLabel::new(),
            text_should_be_updated: true,
        }
    }

    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        Label::new(text)
    }

    /// Return the current value of the label's text.
    pub fn text(&self) -> ArcStr {
        self.text.display_text()
    }

    pub fn set_text(&mut self, text: impl Into<LabelText<T>>) {
        self.text = text.into();
        self.text_should_be_updated = true;
    }

    pub fn with_text_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.label.set_text_color(color);
        self
    }

    pub fn with_text_size(mut self, size: impl Into<KeyOrValue<f64>>) -> Self {
        self.label.set_text_size(size);
        self
    }

    pub fn with_font(mut self, font: impl Into<KeyOrValue<FontDescriptor>>) -> Self {
        self.label.set_font(font);
        self
    }

    pub fn with_line_break_mode(mut self, mode: LineBreaking) -> Self {
        self.label.set_line_break_mode(mode);
        self
    }

    pub fn with_text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.label.set_text_alignment(alignment);
        self
    }

    pub fn draw_at(&self, ctx: &mut PaintCtx, origin: impl Into<Point>) {
        self.label.draw_at(ctx, origin)
    }
}

impl Static {
    fn new(s: ArcStr) -> Self {
        Static {
            string: s,
            resolved: false,
        }
    }

    fn resolve(&mut self) -> bool {
        let is_first_call = !self.resolved;
        self.resolved = true;
        is_first_call
    }
}

impl<T> Dynamic<T> {
    fn resolve(&mut self, data: &T, env: &Env) -> bool {
        let new = (self.f)(data, env);
        let changed = new.as_str() != self.resolved.as_ref();
        self.resolved = new.into();
        changed
    }
}

impl<T: Data> LabelText<T> {
    /// Call callback with the text that should be displayed.
    pub fn with_display_text<V>(&self, mut cb: impl FnMut(&str) -> V) -> V {
        match self {
            LabelText::Static(s) => cb(&s.string),
            LabelText::Localized(s) => cb(&s.localized_str()),
            LabelText::Dynamic(s) => cb(&s.resolved),
        }
    }

    /// Return the current resolved text.
    pub fn display_text(&self) -> ArcStr {
        match self {
            LabelText::Static(s) => s.string.clone(),
            LabelText::Localized(s) => s.localized_str(),
            LabelText::Dynamic(s) => s.resolved.clone(),
        }
    }

    /// Update the localization, if necessary.
    /// This ensures that localized strings are up to date.
    ///
    /// Returns `true` if the string has changed.
    pub fn resolve(&mut self, data: &T, env: &Env) -> bool {
        match self {
            LabelText::Static(s) => s.resolve(),
            LabelText::Localized(s) => s.resolve(data, env),
            LabelText::Dynamic(s) => s.resolve(data, env),
        }
    }
}

impl<T: Data> Widget<T> for Label<T> {
    // #[instrument(name = "Label", level = "trace", skip(self, _ctx, _event, _data, _env))]
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    // #[instrument(name = "Label", level = "trace", skip(self, ctx, event, data, env))]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if matches!(event, LifeCycle::WidgetAdded) {
            self.text.resolve(data, env);
            self.text_should_be_updated = false;
        }
        self.label
            .lifecycle(ctx, event, &self.text.display_text(), env);
    }

    // #[instrument(name = "Label", level = "trace", skip(self, ctx, _old_data, data, env))]
    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let data_changed = self.text.resolve(data, env);
        self.text_should_be_updated = false;
        if data_changed {
            let new_text = self.text.display_text();
            self.label.update(ctx, &self.current_text, &new_text, env);
            self.current_text = new_text;
        } else if ctx.env_changed() {
            self.label
                .update(ctx, &self.current_text, &self.current_text, env);
        }
    }

    // #[instrument(name = "Label", level = "trace", skip(self, ctx, bc, _data, env))]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        self.label.layout(ctx, bc, &self.current_text, env)
    }

    // #[instrument(name = "Label", level = "trace", skip(self, ctx, _data, env))]
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let bounds = ctx.size().to_rect(); // druid::Rect::from((Point::new(0., 0.),Point::new(0., 0.)));
        let stroke_width = 2.0;
    
        let rounded_rect = bounds
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(5.0);

        ctx.fill(rounded_rect, &MODE.tool_icon_bg);
        
        if ctx.is_hot(){
            ctx.fill(rounded_rect, &MODE.tool_icon_bg_active);
            ctx.stroke(rounded_rect, &MODE.tool_icon_border, stroke_width);
        }
        self.label.paint(ctx, &self.current_text, env)
    }
}

impl<T: TextStorage> Widget<T> for RawLabel<T> {
    // #[instrument(
    //     name = "RawLabel",
    //     level = "trace",
    //     skip(self, ctx, event, _data, _env)
    // )]
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseUp(event) => {
                // Account for the padding
                let pos = event.pos - Vec2::new(LABEL_X_PADDING, 0.0);
                if let Some(link) = self.layout.link_for_pos(pos) {
                    ctx.submit_command(link.command.clone());
                }
            }
            Event::MouseMove(event) => {
                // Account for the padding
                let pos = event.pos - Vec2::new(LABEL_X_PADDING, 0.0);

                if self.layout.link_for_pos(pos).is_some() {
                    ctx.set_cursor(&druid::Cursor::Pointer);
                } else {
                    ctx.clear_cursor();
                }
            }
            _ => {}
        }
    }

    // #[instrument(name = "RawLabel", level = "trace", skip(self, ctx, event, data, _env))]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, _env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.layout.set_text(data.to_owned());
            }
            LifeCycle::DisabledChanged(disabled) => {
                let color = if *disabled {
                    KeyOrValue::Key(druid::theme::DISABLED_TEXT_COLOR)
                } else {
                    self.default_text_color.clone()
                };
                self.layout.set_text_color(color);
                ctx.request_layout();
            }
            _ => {}
        }
    }

    // #[instrument(
    //     name = "RawLabel",
    //     level = "trace",
    //     skip(self, ctx, old_data, data, _env)
    // )]
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, _env: &Env) {
        if !old_data.same(data) {
            self.layout.set_text(data.clone());
            ctx.request_layout();
        }
        if self.layout.needs_rebuild_after_update(ctx) {
            ctx.request_layout();
        }
    }

    // #[instrument(name = "RawLabel", level = "trace", skip(self, ctx, bc, _data, env))]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        bc.debug_check("Label");

        let width = match self.line_break_mode {
            LineBreaking::WordWrap => bc.max().width - LABEL_X_PADDING * 2.0,
            _ => f64::INFINITY,
        };

        self.layout.set_wrap_width(width);
        self.layout.rebuild_if_needed(ctx.text(), env);

        let text_metrics = self.layout.layout_metrics();
        ctx.set_baseline_offset(text_metrics.size.height - text_metrics.first_baseline);
        let size = bc.constrain(Size::new(
            text_metrics.size.width + 2. * LABEL_X_PADDING,
            text_metrics.size.height,
        ));
        // trace!("Computed size: {}", size);
        size
    }

    // #[instrument(name = "RawLabel", level = "trace", skip(self, ctx, _data, _env))]
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        let origin = Point::new(LABEL_X_PADDING, 0.0);
        let label_size = ctx.size();

        if self.line_break_mode == LineBreaking::Clip {
            ctx.clip(label_size.to_rect());
        }
        self.draw_at(ctx, origin)
    }
}

impl<T: TextStorage> Default for RawLabel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for Label<T> {
    type Target = RawLabel<ArcStr>;
    fn deref(&self) -> &Self::Target {
        &self.label
    }
}

impl<T> DerefMut for Label<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.label
    }
}
impl<T> From<String> for LabelText<T> {
    fn from(src: String) -> LabelText<T> {
        LabelText::Static(Static::new(src.into()))
    }
}

impl<T> From<&str> for LabelText<T> {
    fn from(src: &str) -> LabelText<T> {
        LabelText::Static(Static::new(src.into()))
    }
}

impl<T> From<ArcStr> for LabelText<T> {
    fn from(string: ArcStr) -> LabelText<T> {
        LabelText::Static(Static::new(string))
    }
}

impl<T> From<LocalizedString<T>> for LabelText<T> {
    fn from(src: LocalizedString<T>) -> LabelText<T> {
        LabelText::Localized(src)
    }
}

impl<T, F: Fn(&T, &Env) -> String + 'static> From<F> for LabelText<T> {
    fn from(src: F) -> LabelText<T> {
        let f = Box::new(src);
        LabelText::Dynamic(Dynamic {
            f,
            resolved: ArcStr::from(""),
        })
    }
}
