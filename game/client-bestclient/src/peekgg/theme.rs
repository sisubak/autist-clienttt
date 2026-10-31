use std::sync::{OnceLock, RwLock};

use egui::Color32;

#[derive(Debug, Clone, Copy)]
pub struct MenuTheme {
    pub text: Color32,
    pub muted: Color32,
    pub dim: Color32,
    pub accent: Color32,

    pub glass_fill: Color32,
    pub panel_fill: Color32,
    pub shadow: Color32,
    pub track: Color32,
    pub hover: Color32,
    pub border: Color32,

    pub dim_strength_in_game: f32,
    pub dim_strength_offline: f32,

    pub rail_w: f32,
    pub island_gap: f32,
    pub rail_item: f32,
    pub rail_radius: f32,
    pub panel_w: f32,
    pub panel_h: f32,
    pub panel_radius: f32,
    pub header_h: f32,
    pub margin_x: f32,
    pub margin_y: f32,

    pub search_h: f32,
    pub search_gap: f32,
    pub search_max_w: f32,
    pub search_ratio: f32,

    pub card_gap: f32,
    pub card_radius: f32,
    pub card_header_h: f32,
    pub card_dot: f32,
    pub three_columns_min_w: f32,

    pub row_toggle_h: f32,
    pub row_slider_h: f32,
    pub row_wide_h: f32,
    pub row_label_h: f32,
    pub row_pad_x: f32,

    pub toggle_w: f32,
    pub toggle_h: f32,
    pub slider_track_w: f32,
    pub slider_value_w: f32,
    pub swatch_w: f32,
    pub swatch_h: f32,
    pub key_button_w: f32,
    pub key_button_h: f32,

    pub font_title: f32,
    pub font_header: f32,
    pub font_row: f32,
    pub font_value: f32,
    pub font_hint: f32,

    pub open_speed: f32,
    pub close_speed: f32,
    pub hover_speed: f32,
    pub page_speed: f32,
    pub row_glow: f32,
}

impl Default for MenuTheme {
    fn default() -> Self {
        Self {
            text: Color32::from_rgb(247, 247, 255),
            muted: Color32::from_rgb(199, 201, 219),
            dim: Color32::from_rgb(148, 150, 171),
            accent: Color32::from_rgb(97, 82, 235),

            glass_fill: Color32::from_rgba_premultiplied(9, 10, 14, 230),
            panel_fill: Color32::from_rgba_premultiplied(9, 10, 14, 214),
            shadow: Color32::from_rgba_premultiplied(0, 0, 0, 90),
            track: Color32::from_rgba_premultiplied(36, 36, 36, 36),
            hover: Color32::from_rgba_premultiplied(18, 18, 18, 18),
            border: Color32::from_rgba_premultiplied(56, 56, 56, 56),

            dim_strength_in_game: 0.55,
            dim_strength_offline: 0.22,

            rail_w: 46.0,
            island_gap: 14.0,
            rail_item: 32.0,
            rail_radius: 10.0,
            panel_w: 700.0,
            panel_h: 398.0,
            panel_radius: 12.0,
            header_h: 46.0,
            margin_x: 22.0,
            margin_y: 20.0,

            search_h: 28.0,
            search_gap: 10.0,
            search_max_w: 430.0,
            search_ratio: 0.82,

            card_gap: 11.0,
            card_radius: 12.0,
            card_header_h: 22.0,
            card_dot: 6.0,
            three_columns_min_w: 596.0,

            row_toggle_h: 19.0,
            row_slider_h: 20.0,
            row_wide_h: 21.0,
            row_label_h: 12.0,
            row_pad_x: 9.0,

            toggle_w: 24.0,
            toggle_h: 11.5,
            slider_track_w: 66.0,
            slider_value_w: 32.0,
            swatch_w: 30.0,
            swatch_h: 14.0,
            key_button_w: 92.0,
            key_button_h: 17.0,

            font_title: 12.0,
            font_header: 6.5,
            font_row: 7.0,
            font_value: 6.5,
            font_hint: 5.5,

            open_speed: 1.9,
            close_speed: 2.2,
            hover_speed: 13.0,
            page_speed: 4.0,
            row_glow: 0.10,
        }
    }
}

fn by_utf8xbot_4902() -> &'static RwLock<MenuTheme> {
    static THEME: OnceLock<RwLock<MenuTheme>> = OnceLock::new();
    THEME.get_or_init(|| RwLock::new(MenuTheme::default()))
}

pub fn by_utf8xbot_4900() -> MenuTheme {
    by_utf8xbot_4902()
        .read()
        .map(|theme| *theme)
        .unwrap_or_default()
}

pub fn by_utf8xbot_4901(theme: MenuTheme) {
    if let Ok(mut guard) = by_utf8xbot_4902().write() {
        *guard = theme;
    }
}

pub fn by_utf8xbot_4903(color: Color32, alpha: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (alpha * 255.0).clamp(0.0, 255.0) as u8,
    )
}

pub fn by_utf8xbot_4904(a: Color32, b: Color32, t: f32) -> Color32 {
    let f = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_unmultiplied(
        f(a.r(), b.r()),
        f(a.g(), b.g()),
        f(a.b(), b.b()),
        f(a.a(), b.a()),
    )
}
