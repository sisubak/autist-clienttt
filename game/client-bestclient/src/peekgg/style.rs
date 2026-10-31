use egui::Color32;

pub const COL_TEXT: Color32 = Color32::from_rgb(247, 247, 255);
pub const COL_MUTED: Color32 = Color32::from_rgb(199, 201, 219);
pub const COL_DIM: Color32 = Color32::from_rgb(148, 150, 171);
pub const COL_ACCENT: Color32 = Color32::from_rgb(97, 82, 235);
pub const COL_GLASS_FILL: Color32 = Color32::from_rgba_premultiplied(9, 10, 14, 230);
pub const COL_PANEL_FILL: Color32 = Color32::from_rgba_premultiplied(9, 10, 14, 214);
pub const COL_SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 90);
pub const COL_TRACK: Color32 = Color32::from_rgba_premultiplied(36, 36, 36, 36);
pub const COL_PILL_OFF: Color32 = Color32::from_rgba_premultiplied(33, 33, 33, 33);
pub const COL_HOVER: Color32 = Color32::from_rgba_premultiplied(18, 18, 18, 18);
pub const COL_BORDER: Color32 = Color32::from_rgba_premultiplied(56, 56, 56, 56);

pub const RAIL_W: f32 = 46.0;
pub const GAP: f32 = 14.0;
pub const SEARCH_H: f32 = 28.0;
pub const SEARCH_GAP: f32 = 10.0;
pub const PANEL_W: f32 = 700.0;
pub const PANEL_H: f32 = 398.0;
pub const HEADER_H: f32 = 46.0;
pub const CARD_GAP: f32 = 11.0;
pub const CARD_RADIUS: f32 = 12.0;
pub const CARD_HEADER_H: f32 = 22.0;
pub const ROW_TOGGLE_H: f32 = 19.0;
pub const ROW_SLIDER_H: f32 = 20.0;
pub const ROW_WIDE_H: f32 = 21.0;
pub const ROW_LABEL_H: f32 = 12.0;
pub const RAIL_ITEM: f32 = 32.0;

pub fn by_utf8xbot_4301(value: f32) -> f32 {
    if value < 0.02 { 0.0 } else { value }
}
