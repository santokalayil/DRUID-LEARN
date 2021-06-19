use druid::Color as C;

pub struct Color {
    pub btn_bg: C,
    pub btn_fg: C,
    pub btn_stroke: C,
    pub btn_bg_hot: C,
    pub btn_bg_active: C,
}

pub const DARK: Color = Color {
    btn_bg: C::rgba8(0x11, 0x11, 0x11, 0xff),
    btn_fg: C::YELLOW,
    btn_stroke: C::RED,
    btn_bg_hot: C::rgba8(0xff, 0x22, 0xee, 0xff),
    btn_bg_active: C::WHITE,
};

