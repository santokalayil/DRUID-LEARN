use druid::Color as C;

pub struct Color {
    pub primary_dark: C,
    pub primary_light: C,
    pub frame_border: C,
    pub split_bar_color: C,
    pub split_bar_color_draggable: C,
    pub tool_icon_bg : C, 
    pub tool_icon_border : C,
    pub tool_icon_bg_active : C,
    pub side_bar_title_heading_bg: C,
    pub side_bar_item_bg: C,
    pub side_bar_item_text_color: C,
    pub side_bar_item_icon_color: C,


}

pub const DARK: Color = Color { 
    primary_dark: C::rgba8(0x11, 0x11, 0x11, 0xff),
    primary_light: C::rgba8(0xee, 0xee, 0xee, 0xff),
    frame_border: C::rgba8(0x01, 0x01, 0x01, 0xff),
    split_bar_color: C::rgba8(0x22, 0x22, 0x22, 0xff),
    split_bar_color_draggable: C::rgba8(0x33, 0x44, 0x44, 0xff),
    tool_icon_bg : C::rgba8(0x44, 0x44, 0x44, 0xff), 
    tool_icon_border : C::rgba8(0x22, 0x22, 0x22, 0xff),  // 0x22, 0x22, 0x22, 0xff
    tool_icon_bg_active : C::rgba8(0x55, 0x55, 0x55, 0xff),  // 0x55, 0x55, 0x55, 0xff
    side_bar_title_heading_bg: C::rgba8(0x11, 0x11, 0x11, 0xff),
    side_bar_item_bg: C::rgba8(0x20, 0x20, 0x20, 0xff), // transparent
    side_bar_item_text_color: C::rgba8(0x9a, 0x9a, 0x9a, 0xff),
    side_bar_item_icon_color: C::rgba8(0x40, 0x9a, 0xf5, 0xff),
    

};

// pub const LIGHT: Color = Color { 
//     primary_dark: C::rgba8(0x11, 0x11, 0x11, 0xff),
//     primary_light: C::rgba8(0xee, 0xee, 0xee, 0xff),
//     // background_dark: 
//     frame_border: C::rgba8(0x11, 0x11, 0x11, 0xff),
// };