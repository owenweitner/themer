use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
    pub name: String,
    pub color0:  [f32; 3],
    pub color1:  [f32; 3],
    pub color2:  [f32; 3],
    pub color3:  [f32; 3],
    pub color4:  [f32; 3],
    pub color5:  [f32; 3],
    pub color6:  [f32; 3],
    pub color7:  [f32; 3],
    pub color8:  [f32; 3],
    pub color9: [f32; 3],
    pub color10: [f32; 3],
    pub color11: [f32; 3],
    pub color12: [f32; 3],
    pub color13: [f32; 3],
    pub color14: [f32; 3],
    pub color15: [f32; 3],
    pub foreground: [f32; 3],
    pub background: [f32; 3],
    pub polybar_background: [f32; 3],
    pub polybar_foreground: [f32; 3],
    pub chrome_background: [f32; 3],
    pub chrome_foreground: [f32; 3],
    pub imagepath: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppData {
    pub themes: Vec<Theme>,
    pub entries: Vec<String>,
    pub selected_index: usize,
    pub tempname: String,
    pub temppath: Option<String>,
    pub current_theme: String,
}