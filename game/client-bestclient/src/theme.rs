use egui::Color32;
use game_config::config::ConfigBestClientUi;
use ui_base::theme::UiTheme;

pub fn by_utf8xbot_2101(cfg: &ConfigBestClientUi) -> UiTheme {
    UiTheme {
        enabled: cfg.better_ui,
        accent: Color32::from_rgb(cfg.accent.r, cfg.accent.g, cfg.accent.b),
        corner_rounding: cfg.corner_rounding as f32,
        animation_time: cfg.animation_time as f32,
        better_scrollbar: cfg.better_scrollbar,
        scrollbar_width: cfg.scrollbar_width as f32,
        font_scale: cfg.font_scale as f32,
    }
}

pub fn by_utf8xbot_2102(cfg: &ConfigBestClientUi) {
    let theme = by_utf8xbot_2101(cfg);
    let current = ui_base::theme::by_utf8xbot_9109();
    if by_utf8xbot_2103(&current, &theme) {
        return;
    }
    ui_base::theme::by_utf8xbot_9108(theme);
}

fn by_utf8xbot_2103(a: &UiTheme, b: &UiTheme) -> bool {
    a.enabled == b.enabled
        && a.accent == b.accent
        && (a.corner_rounding - b.corner_rounding).abs() < f32::EPSILON
        && (a.animation_time - b.animation_time).abs() < f32::EPSILON
        && a.better_scrollbar == b.better_scrollbar
        && (a.scrollbar_width - b.scrollbar_width).abs() < f32::EPSILON
        && (a.font_scale - b.font_scale).abs() < f32::EPSILON
}
