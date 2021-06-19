use druid::{Data, Color}; // Lens


#[derive(Clone, Data, Debug)]  // , Lens
pub struct SidebarButtonData {
    pub background: Color,
    pub foreground: Color,
    pub stroke: Color,

    pub label_text: String,
    pub label_icon: String,
}
// use crate::colors::sidebarbtn::DARK as THEME;
// impl SidebarButtonData {
//     pub fn new() -> Self {
//         SidebarButtonData {
//             background: THEME.btn_bg,
//             foreground: THEME.btn_fg,
//             stroke: THEME.btn_stroke,

//             label_text: "EMPTY".to_string(),
//             label_icon: "".to_string(),
//         }
//     }
// }