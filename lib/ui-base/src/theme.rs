use std::sync::{OnceLock, RwLock};

use egui::{Color32, CornerRadius, Stroke, Style};

#[derive(Debug, Clone, Copy)]
pub struct UiTheme {
    pub enabled: bool,
    pub accent: Color32,
    pub corner_rounding: f32,
    pub animation_time: f32,
    pub better_scrollbar: bool,
    pub scrollbar_width: f32,
    pub font_scale: f32,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            enabled: false,
            accent: Color32::from_rgb(0, 170, 255),
            corner_rounding: 5.0,
            animation_time: 0.16,
            better_scrollbar: false,
            scrollbar_width: 8.0,
            font_scale: 1.0,
        }
    }
}

fn by_utf8xbot_9107() -> &'static RwLock<UiTheme> {
    static THEME: OnceLock<RwLock<UiTheme>> = OnceLock::new();
    THEME.get_or_init(|| RwLock::new(UiTheme::default()))
}

pub fn by_utf8xbot_9108(theme: UiTheme) {
    if let Ok(mut guard) = by_utf8xbot_9107().write() {
        *guard = theme;
    }
}

pub fn by_utf8xbot_9109() -> UiTheme {
    by_utf8xbot_9107()
        .read()
        .map(|guard| *guard)
        .unwrap_or_default()
}

fn by_utf8xbot_9110(style: &mut Style, theme: UiTheme) {
    let rounding: CornerRadius = theme.corner_rounding.into();
    let stroke_width: f32 = 1.0;

    style.animation_time = if theme.animation_time > 0.0 {
        theme.animation_time
    } else {
        0.0
    };

    let visuals = &mut style.visuals;
    visuals.selection.bg_fill =
        Color32::from_rgba_unmultiplied(theme.accent.r(), theme.accent.g(), theme.accent.b(), 140);
    visuals.selection.stroke = Stroke::new(stroke_width, theme.accent);
    visuals.hyperlink_color = theme.accent;
    visuals.slider_trailing_fill = true;

    let widgets = &mut visuals.widgets;
    widgets.noninteractive.corner_radius = rounding;
    widgets.inactive.corner_radius = rounding;
    widgets.hovered.corner_radius = rounding;
    widgets.active.corner_radius = rounding;
    widgets.open.corner_radius = rounding;

    widgets.hovered.bg_fill =
        Color32::from_rgba_unmultiplied(theme.accent.r(), theme.accent.g(), theme.accent.b(), 60);
    widgets.hovered.weak_bg_fill =
        Color32::from_rgba_unmultiplied(theme.accent.r(), theme.accent.g(), theme.accent.b(), 40);
    widgets.hovered.bg_stroke = Stroke::new(stroke_width, theme.accent);
    widgets.active.bg_fill =
        Color32::from_rgba_unmultiplied(theme.accent.r(), theme.accent.g(), theme.accent.b(), 110);
    widgets.active.bg_stroke = Stroke::new(stroke_width, theme.accent);

    if theme.better_scrollbar {
        style.spacing.scroll.floating = false;
        style.spacing.scroll.bar_width = theme.scrollbar_width;
        style.spacing.scroll.bar_inner_margin = 2.0;
        style.spacing.scroll.bar_outer_margin = 0.0;
        style.spacing.scroll.handle_min_length = theme.scrollbar_width * 2.0;
        style.spacing.scroll.floating_width = theme.scrollbar_width;
        style.spacing.scroll.foreground_color = true;
    }

    if (theme.font_scale - 1.0).abs() > f32::EPSILON {
        for (_, font_id) in style.text_styles.iter_mut() {
            font_id.size = (font_id.size * theme.font_scale).max(1.0);
        }
    }
}

pub fn by_utf8xbot_9111(style: &mut Style) {
    let theme = by_utf8xbot_9109();
    if theme.enabled {
        by_utf8xbot_9110(style, theme);
    }
}
