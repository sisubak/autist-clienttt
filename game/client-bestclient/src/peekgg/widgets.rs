use egui::{Align2, Color32, CornerRadius, Id, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
use game_config::config::Config;

use super::bind::Bind;
use super::data::{RATIO_PRESETS, Row};
use super::page::{
    by_utf8xbot_4510, by_utf8xbot_4511, by_utf8xbot_4512, by_utf8xbot_4513, by_utf8xbot_4514,
    by_utf8xbot_4520, by_utf8xbot_4525,
};
use super::state::{self as pstate, State};
use super::theme::MenuTheme;

fn th() -> MenuTheme {
    super::theme::by_utf8xbot_4900()
}

pub(super) fn by_utf8xbot_4730(
    ui: &mut egui::Ui,
    state: &mut State,
    cfg: &mut Config,
    id: &str,
    bind: Option<Bind>,
    painter: &egui::Painter,
    rect: Rect,
    s: f32,
) {
    let Some(bind) = bind else { return };
    let on = bind.by_utf8xbot_4101(cfg) != 0;
    let t = pstate::by_utf8xbot_4409(state, &format!("tg{id}"), if on { 1.0 } else { 0.0 }, 12.0);

    if t > 0.01 {
        painter.rect_stroke(
            rect.expand(5.5 * s),
            CornerRadius::same(255),
            Stroke::new(5.5 * s, by_utf8xbot_4513(th().accent, 0.05 * t)),
            StrokeKind::Outside,
        );
        painter.rect_stroke(
            rect.expand(2.5 * s),
            CornerRadius::same(255),
            Stroke::new(2.5 * s, by_utf8xbot_4513(th().accent, 0.09 * t)),
            StrokeKind::Outside,
        );
    }

    painter.rect_filled(
        rect,
        CornerRadius::same(255),
        by_utf8xbot_4514(
            by_utf8xbot_4513(Color32::WHITE, 0.16),
            by_utf8xbot_4513(th().accent, 0.92),
            t,
        ),
    );

    let knob_size = rect.height() - 4.5 * s;
    let knob_x = rect.left() + 1.75 * s + t * (rect.width() - knob_size - 3.5 * s);
    let knob = Rect::from_min_size(
        pos2(knob_x, rect.top() + (rect.height() - knob_size) * 0.5),
        vec2(knob_size, knob_size),
    );
    painter.circle_filled(
        knob.center() + vec2(0.0, 1.0 * s),
        knob_size * 0.5,
        Color32::from_black_alpha(72),
    );
    painter.circle_filled(
        knob.center(),
        knob_size * 0.5,
        by_utf8xbot_4513(Color32::WHITE, 0.80 + 0.18 * t),
    );

    if by_utf8xbot_4511(ui, rect, &format!("tg{id}")) {
        bind.by_utf8xbot_4102(cfg, if on { 0 } else { 1 });
    }
}

pub(super) fn by_utf8xbot_4740(
    ui: &mut egui::Ui,
    cfg: &mut Config,
    id: &str,
    row: &Row,
    painter: &egui::Painter,
    row_rect: Rect,
    s: f32,
) {
    let Some(bind) = row.bind else { return };
    let value_width = 32.0 * s;
    let track_width = 66.0 * s;
    let track = Rect::from_min_size(
        pos2(
            row_rect.right() - 8.0 * s - value_width - track_width,
            row_rect.top() + (row_rect.height() - 3.0 * s) * 0.5,
        ),
        vec2(track_width, 3.0 * s),
    );
    let hit = Rect::from_min_size(
        pos2(track.left(), track.top() - 8.0 * s),
        vec2(track.width(), track.height() + 16.0 * s),
    );

    let value = bind.by_utf8xbot_4101(cfg);
    let ratio = if row.max > row.min {
        ((value - row.min) as f32 / (row.max - row.min) as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };

    painter.rect_filled(track, CornerRadius::same(2), th().track);
    let filled = Rect::from_min_size(track.min, vec2(track.width() * ratio, track.height()));
    painter.rect_filled(
        filled,
        CornerRadius::same(2),
        th().accent.gamma_multiply(0.85),
    );

    let knob = pos2(track.left() + track.width() * ratio, track.center().y);
    painter.circle_filled(knob, 5.0 * s, by_utf8xbot_4513(th().text, 0.95));
    painter.circle_filled(knob, 3.0 * s, th().accent);

    let value_rect = Rect::from_min_size(
        pos2(track.right() + 4.0 * s, row_rect.top()),
        vec2(value_width, row_rect.height()),
    );
    by_utf8xbot_4512(
        painter,
        value_rect,
        &format!("{}{}", value, row.suffix),
        6.5 * s,
        Align2::RIGHT_CENTER,
        th().muted,
    );

    let response = ui.interact(
        hit,
        Id::new(("peekgg", format!("sl{id}"))),
        Sense::click_and_drag(),
    );
    if (response.dragged() || response.clicked())
        && let Some(pos) = ui.ctx().pointer_interact_pos()
    {
        let new_ratio = ((pos.x - track.left()) / track.width()).clamp(0.0, 1.0);
        let new_value = row.min + (new_ratio * (row.max - row.min) as f32).round() as i64;
        bind.by_utf8xbot_4102(cfg, new_value.clamp(row.min, row.max));
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn by_utf8xbot_4750(
    ui: &mut egui::Ui,
    state: &mut State,
    cfg: &mut Config,
    id: &str,
    row: &Row,
    painter: &egui::Painter,
    row_rect: Rect,
    s: f32,
    card_index: usize,
    row_index: usize,
) {
    let Some(bind) = row.bind else { return };
    let rect = Rect::from_min_size(
        pos2(row_rect.right() - 118.0 * s, row_rect.top()),
        vec2(112.0 * s, row_rect.height()),
    );
    let value = bind.by_utf8xbot_4101(cfg);
    let text = row
        .options
        .get(value.max(0) as usize)
        .copied()
        .unwrap_or("?");

    let hover = ui.rect_contains_pointer(rect);
    by_utf8xbot_4510(
        painter,
        rect,
        rect.height() * 0.5,
        1.0 * s,
        by_utf8xbot_4513(Color32::WHITE, if hover { 0.40 } else { 0.20 }),
    );
    by_utf8xbot_4512(
        painter,
        rect,
        text,
        6.5 * s,
        Align2::CENTER_CENTER,
        th().text,
    );
    let arrow = Rect::from_min_size(
        pos2(
            rect.right() - 12.0 * s,
            rect.top() + (rect.height() - 8.0 * s) * 0.5,
        ),
        vec2(8.0 * s, 8.0 * s),
    );
    by_utf8xbot_4525(painter, arrow, true, th().muted);

    if by_utf8xbot_4511(ui, rect, &format!("dd{id}")) {
        state.dropdown = if state.dropdown == Some((card_index, row_index)) {
            None
        } else {
            Some((card_index, row_index))
        };
    }

    if state.dropdown == Some((card_index, row_index)) {
        let item_h = 16.0 * s;
        let list_h = item_h * row.options.len() as f32 + 6.0 * s;
        let list = Rect::from_min_size(
            pos2(rect.left(), rect.bottom() + 3.0 * s),
            vec2(rect.width().max(120.0 * s), list_h),
        );
        painter.rect_filled(
            list,
            CornerRadius::same(8),
            by_utf8xbot_4513(th().panel_fill, 0.98),
        );
        by_utf8xbot_4510(
            painter,
            list,
            8.0,
            1.0,
            by_utf8xbot_4513(Color32::WHITE, 0.24),
        );
        for (i, option) in row.options.iter().enumerate() {
            let item = Rect::from_min_size(
                pos2(
                    list.left() + 4.0 * s,
                    list.top() + 3.0 * s + i as f32 * item_h,
                ),
                vec2(list.width() - 8.0 * s, item_h),
            );
            let selected = value == i as i64;
            let item_hover = ui.rect_contains_pointer(item);
            if item_hover || selected {
                painter.rect_filled(
                    item,
                    CornerRadius::same(5),
                    if selected {
                        by_utf8xbot_4513(th().accent, 0.35)
                    } else {
                        th().hover
                    },
                );
            }
            by_utf8xbot_4512(
                painter,
                item,
                option,
                6.2 * s,
                Align2::LEFT_CENTER,
                if selected { th().text } else { th().muted },
            );
            if by_utf8xbot_4511(ui, item, &format!("ddi{id}.{i}")) {
                bind.by_utf8xbot_4102(cfg, i as i64);
                state.dropdown = None;
            }
        }
    }
}

pub(super) fn by_utf8xbot_4760(
    ui: &mut egui::Ui,
    cfg: &mut Config,
    id: &str,
    row: &Row,
    painter: &egui::Painter,
    row_rect: Rect,
    s: f32,
) {
    let Some(bind) = row.bind else { return };
    let rect = Rect::from_min_size(
        pos2(row_rect.right() - 96.0 * s, row_rect.top()),
        vec2(90.0 * s, row_rect.height()),
    );
    by_utf8xbot_4510(
        painter,
        rect,
        rect.height() * 0.5,
        1.0 * s,
        by_utf8xbot_4513(Color32::WHITE, 0.20),
    );

    let value = bind.by_utf8xbot_4101(cfg);
    let minus = Rect::from_min_size(rect.min, vec2(18.0 * s, rect.height()));
    let plus = Rect::from_min_size(
        pos2(rect.right() - 18.0 * s, rect.top()),
        vec2(18.0 * s, rect.height()),
    );
    by_utf8xbot_4512(
        painter,
        minus,
        "-",
        8.0 * s,
        Align2::CENTER_CENTER,
        th().muted,
    );
    by_utf8xbot_4512(
        painter,
        plus,
        "+",
        8.0 * s,
        Align2::CENTER_CENTER,
        th().muted,
    );
    by_utf8xbot_4512(
        painter,
        rect,
        &value.to_string(),
        6.5 * s,
        Align2::CENTER_CENTER,
        th().text,
    );

    let changed = ui.input(|i| i.modifiers.shift).then_some(10).unwrap_or(1);
    if by_utf8xbot_4511(ui, minus, &format!("st-{id}")) {
        bind.by_utf8xbot_4102(cfg, (value - changed).clamp(row.min, row.max));
    }
    if by_utf8xbot_4511(ui, plus, &format!("st+{id}")) {
        bind.by_utf8xbot_4102(cfg, (value + changed).clamp(row.min, row.max));
    }
}

pub(super) fn by_utf8xbot_4770(
    ui: &mut egui::Ui,
    state: &mut State,
    cfg: &mut Config,
    id: &str,
    row: &Row,
    painter: &egui::Painter,
    row_rect: Rect,
    s: f32,
) {
    let Some(color_bind) = row.color else { return };
    let swatch = Rect::from_min_size(
        pos2(
            row_rect.right() - 36.0 * s,
            row_rect.top() + (row_rect.height() - 14.0 * s) * 0.5,
        ),
        vec2(30.0 * s, 14.0 * s),
    );
    let color = color_bind.by_utf8xbot_4103(cfg);
    painter.rect_filled(swatch, CornerRadius::same(4), color);
    by_utf8xbot_4510(
        painter,
        swatch,
        4.0,
        1.0,
        by_utf8xbot_4513(Color32::WHITE, 0.35),
    );

    if by_utf8xbot_4511(ui, swatch, &format!("cl{id}")) {
        state.color_popup = if state.color_popup == Some(color_bind) {
            None
        } else {
            Some(color_bind)
        };
    }

    if state.color_popup == Some(color_bind) {
        let mut rgb = [color.r(), color.g(), color.b()];
        let popup = Rect::from_min_size(
            pos2(swatch.left() - 90.0 * s, swatch.bottom() + 4.0 * s),
            vec2(120.0 * s, 74.0 * s),
        );
        painter.rect_filled(
            popup,
            CornerRadius::same(8),
            by_utf8xbot_4513(th().panel_fill, 0.98),
        );
        by_utf8xbot_4510(
            painter,
            popup,
            8.0,
            1.0,
            by_utf8xbot_4513(Color32::WHITE, 0.24),
        );
        let mut changed = false;
        let mut field_rect = popup.shrink(6.0 * s);
        for (label, index) in [("R", 0usize), ("G", 1), ("B", 2)] {
            let line = Rect::from_min_size(field_rect.min, vec2(field_rect.width(), 20.0 * s));
            let label_rect = Rect::from_min_size(line.min, vec2(12.0 * s, line.height()));
            by_utf8xbot_4512(
                painter,
                label_rect,
                label,
                6.5 * s,
                Align2::LEFT_CENTER,
                th().muted,
            );
            let mut value = rgb[index] as f32;
            let slider_rect = Rect::from_min_size(
                pos2(line.left() + 12.0 * s, line.top() + 8.0 * s),
                vec2(line.width() - 46.0 * s, 3.0 * s),
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(slider_rect), |ui| {
                ui.spacing_mut().slider_width = slider_rect.width();
                if ui
                    .add(egui::Slider::new(&mut value, 0.0..=255.0).show_value(false))
                    .changed()
                {
                    rgb[index] = value.round().clamp(0.0, 255.0) as u8;
                    changed = true;
                }
            });
            let value_rect = Rect::from_min_size(
                pos2(line.right() - 32.0 * s, line.top()),
                vec2(30.0 * s, line.height()),
            );
            by_utf8xbot_4512(
                painter,
                value_rect,
                &rgb[index].to_string(),
                6.0 * s,
                Align2::RIGHT_CENTER,
                th().text,
            );
            field_rect = Rect::from_min_size(
                pos2(field_rect.left(), line.bottom() + 2.0 * s),
                vec2(field_rect.width(), field_rect.height() - 22.0 * s),
            );
        }
        if changed {
            color_bind.by_utf8xbot_4104(cfg, Color32::from_rgb(rgb[0], rgb[1], rgb[2]));
        }
    }
}

pub(super) fn by_utf8xbot_4780(
    ui: &mut egui::Ui,
    cfg: &mut Config,
    id: &str,
    row: &Row,
    painter: &egui::Painter,
    row_rect: Rect,
    s: f32,
) {
    let Some(bind) = row.bind else { return };
    let on = bind.by_utf8xbot_4101(cfg) == row.min;
    let dot = Rect::from_min_size(
        pos2(
            row_rect.left() + 7.0 * s,
            row_rect.top() + (row_rect.height() - 7.0 * s) * 0.5,
        ),
        vec2(7.0 * s, 7.0 * s),
    );
    by_utf8xbot_4520(painter, dot, if on { 1.0 } else { 0.0 });

    let text = Rect::from_min_size(
        pos2(dot.right() + 7.0 * s, row_rect.top()),
        vec2(
            row_rect.width() - (dot.width() + 21.0 * s),
            row_rect.height(),
        ),
    );
    by_utf8xbot_4512(
        painter,
        text,
        row.label,
        6.5 * s,
        Align2::LEFT_CENTER,
        if on { th().text } else { th().muted },
    );

    if by_utf8xbot_4511(ui, row_rect, &format!("pk{id}")) {
        bind.by_utf8xbot_4102(cfg, row.min);
    }
}

pub(super) fn by_utf8xbot_4790(
    ui: &mut egui::Ui,
    cfg: &mut Config,
    id: &str,
    row: &Row,
    painter: &egui::Painter,
    row_rect: Rect,
    s: f32,
) {
    let Some(bind) = row.bind else { return };
    let rect = Rect::from_min_size(
        pos2(row_rect.right() - 118.0 * s, row_rect.top()),
        vec2(112.0 * s, row_rect.height()),
    );
    let value = bind.by_utf8xbot_4101(cfg);
    let text = RATIO_PRESETS
        .get(value.max(0) as usize)
        .copied()
        .unwrap_or("?");
    let hover = ui.rect_contains_pointer(rect);
    by_utf8xbot_4510(
        painter,
        rect,
        rect.height() * 0.5,
        1.0 * s,
        by_utf8xbot_4513(Color32::WHITE, if hover { 0.40 } else { 0.20 }),
    );
    by_utf8xbot_4512(
        painter,
        rect,
        text,
        6.5 * s,
        Align2::CENTER_CENTER,
        th().text,
    );
    by_utf8xbot_4525(
        painter,
        Rect::from_min_size(
            pos2(
                rect.right() - 12.0 * s,
                rect.top() + (rect.height() - 8.0 * s) * 0.5,
            ),
            vec2(8.0 * s, 8.0 * s),
        ),
        true,
        th().muted,
    );

    let next = if by_utf8xbot_4511(ui, rect, &format!("ra{id}")) {
        Some((value + 1) % RATIO_PRESETS.len() as i64)
    } else if ui.input(|i| i.modifiers.shift) && ui.rect_contains_pointer(rect) {
        None
    } else {
        None
    };
    if let Some(next) = next {
        bind.by_utf8xbot_4102(cfg, next);
    }
}

pub(super) fn by_utf8xbot_4800(
    ui: &mut egui::Ui,
    state: &mut State,
    cfg: &mut Config,
    painter: &egui::Painter,
    row_rect: Rect,
    s: f32,
) {
    let label = Rect::from_min_size(
        row_rect.min,
        vec2(row_rect.width() - 96.0 * s, row_rect.height()),
    );
    by_utf8xbot_4512(
        painter,
        label,
        "Открыть меню",
        7.0 * s,
        Align2::LEFT_CENTER,
        th().muted,
    );

    let button = Rect::from_min_size(
        pos2(
            row_rect.right() - 92.0 * s,
            row_rect.top() + (row_rect.height() - 17.0 * s) * 0.5,
        ),
        vec2(92.0 * s, 17.0 * s),
    );
    let hover = ui.rect_contains_pointer(button);
    by_utf8xbot_4510(
        painter,
        button,
        button.height() * 0.5,
        1.0 * s,
        by_utf8xbot_4513(Color32::WHITE, if hover { 0.40 } else { 0.20 }),
    );

    if state.awaiting_key {
        if let Some(name) = ui.input(|i| {
            i.events.iter().find_map(|ev| match ev {
                egui::Event::Key {
                    physical_key,
                    pressed: true,
                    ..
                } => Some(format!("{physical_key:?}")),
                _ => None,
            })
        }) {
            let cleaned = name
                .trim_start_matches("Code(")
                .trim_end_matches(')')
                .to_string();
            if cleaned == "Escape" {
                state.awaiting_key = false;
            } else {
                cfg.game.peekgg.cl.key = cleaned;
                state.awaiting_key = false;
            }
        }
    }

    let text = if state.awaiting_key {
        "...".to_string()
    } else {
        cfg.game.peekgg.cl.key.to_uppercase()
    };
    by_utf8xbot_4512(
        painter,
        button,
        &text,
        6.5 * s,
        Align2::CENTER_CENTER,
        if state.awaiting_key {
            th().accent
        } else {
            th().text
        },
    );

    if by_utf8xbot_4511(ui, button, "keybind") {
        state.awaiting_key = true;
    }
}
