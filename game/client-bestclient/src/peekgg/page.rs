use egui::{
    Align2, Color32, CornerRadius, FontId, Id, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2, pos2,
    vec2,
};
use game_config::config::Config;
use ui_base::types::{UiRenderPipe, UiState};
use ui_generic::traits::UiPageInterface;

use super::bind::Bind;
use super::data::{Card, PAGES, Page, RATIO_PRESETS, Row, RowKind};
use super::state::{self as pstate, State};
use super::style::*;

pub const PEEKGG_PATH: &str = "peekgg";

#[derive(Default)]
pub struct PeekggUi;

impl UiPageInterface<Config> for PeekggUi {
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        pipe: &mut UiRenderPipe<Config>,
        ui_state: &mut UiState,
    ) {
        let mut state = pstate::by_utf8xbot_4404();
        pstate::by_utf8xbot_4408(&mut state, &pipe.cur_time);
        if !state.open && state.open_anim <= 0.002 {
            return;
        }
        by_utf8xbot_4501(ui, &mut state, pipe.user_data, ui_state);
    }
}

fn by_utf8xbot_4510(painter: &egui::Painter, rect: Rect, radius: f32, width: f32, color: Color32) {
    painter.rect_stroke(
        rect,
        CornerRadius::same(radius as u8),
        Stroke::new(width, color),
        StrokeKind::Inside,
    );
}

fn by_utf8xbot_4511(ui: &mut egui::Ui, rect: Rect, id: &str) -> bool {
    ui.interact(rect, Id::new(("peekgg", id)), Sense::click())
        .clicked()
}

fn by_utf8xbot_4512(
    painter: &egui::Painter,
    rect: Rect,
    text: &str,
    size: f32,
    align: Align2,
    color: Color32,
) {
    painter.text(
        match align {
            Align2::LEFT_CENTER => rect.left_center(),
            Align2::CENTER_CENTER => rect.center(),
            Align2::RIGHT_CENTER => rect.right_center(),
            _ => rect.center(),
        },
        align,
        text,
        FontId::proportional(size),
        color,
    );
}

fn by_utf8xbot_4513(color: Color32, alpha: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (alpha * 255.0).clamp(0.0, 255.0) as u8,
    )
}

fn by_utf8xbot_4514(a: Color32, b: Color32, t: f32) -> Color32 {
    let f = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_unmultiplied(
        f(a.r(), b.r()),
        f(a.g(), b.g()),
        f(a.b(), b.b()),
        f(a.a(), b.a()),
    )
}

fn by_utf8xbot_4520(painter: &egui::Painter, rect: Rect, progress: f32) {
    painter.circle_filled(
        rect.center(),
        rect.height() * 0.5,
        by_utf8xbot_4513(COL_ACCENT, 0.25 * progress),
    );
    painter.circle_filled(
        rect.center(),
        rect.height() * 0.5 - 2.0,
        COL_ACCENT.gamma_multiply(0.35 + 0.4 * progress),
    );
}

fn by_utf8xbot_4521(painter: &egui::Painter, rect: Rect, color: Color32) {
    let r = rect.width().min(rect.height()) * 0.5;
    let c = rect.center();
    let mut pts = Vec::with_capacity(10);
    for i in 0..10 {
        let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::PI / 5.0;
        let radius = if i % 2 == 0 { r } else { r * 0.45 };
        pts.push(pos2(c.x + radius * angle.cos(), c.y + radius * angle.sin()));
    }
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

fn by_utf8xbot_4522(painter: &egui::Painter, rect: Rect, color: Color32) {
    let r = rect.width().min(rect.height()) * 0.5;
    let c = rect.center();
    painter.circle_filled(pos2(c.x, c.y - r * 0.35), r * 0.36, color);
    painter.add(egui::Shape::convex_polygon(
        vec![
            pos2(c.x - r * 0.75, c.y + r * 0.9),
            pos2(c.x + r * 0.75, c.y + r * 0.9),
            pos2(c.x, c.y - r * 0.05),
        ],
        color,
        Stroke::NONE,
    ));
}

fn by_utf8xbot_4523(painter: &egui::Painter, rect: Rect, color: Color32) {
    let r = rect.width().min(rect.height()) * 0.5;
    let c = rect.center();
    painter.circle_stroke(c, r * 0.5, Stroke::new(r * 0.28, color));
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0;
        let inner = r * 0.62;
        let outer = r * 0.98;
        painter.line_segment(
            [
                pos2(c.x + inner * angle.cos(), c.y + inner * angle.sin()),
                pos2(c.x + outer * angle.cos(), c.y + outer * angle.sin()),
            ],
            Stroke::new(r * 0.24, color),
        );
    }
}

fn by_utf8xbot_4524(painter: &egui::Painter, rect: Rect, color: Color32) {
    let r = rect.width().min(rect.height()) * 0.5;
    let c = rect.center();
    painter.circle_stroke(c, r * 0.62, Stroke::new(r * 0.18, color));
    painter.line_segment(
        [
            pos2(c.x + r * 0.35, c.y + r * 0.35),
            pos2(c.x + r * 0.95, c.y + r * 0.95),
        ],
        Stroke::new(r * 0.2, color),
    );
}

fn by_utf8xbot_4525(painter: &egui::Painter, rect: Rect, collapsed: bool, color: Color32) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.32;
    let dy = if collapsed { r * 0.55 } else { -r * 0.55 };
    painter.line_segment(
        [pos2(c.x - r, c.y - dy), pos2(c.x, c.y + dy)],
        Stroke::new(1.4_f32, color),
    );
    painter.line_segment(
        [pos2(c.x + r, c.y - dy), pos2(c.x, c.y + dy)],
        Stroke::new(1.4_f32, color),
    );
}

fn by_utf8xbot_4526(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.42;
    painter.line_segment(
        [pos2(c.x - r, c.y - r), pos2(c.x + r, c.y + r)],
        Stroke::new(1.5_f32, color),
    );
    painter.line_segment(
        [pos2(c.x - r, c.y + r), pos2(c.x + r, c.y - r)],
        Stroke::new(1.5_f32, color),
    );
}

fn by_utf8xbot_4501(
    ui: &mut egui::Ui,
    state: &mut State,
    cfg: &mut Config,
    ui_state: &mut UiState,
) {
    let ease = pstate::by_utf8xbot_4410(state.open_anim);
    let screen = ui.ctx().content_rect();
    let scale_base = screen.height() / 600.0 * (cfg.game.peekgg.cl.scale as f32 / 100.0);
    let s = scale_base * (0.04 + 0.96 * ease);

    let rail_w = RAIL_W * s;
    let gap = GAP * s;
    let search_h = SEARCH_H * s;
    let search_gap = SEARCH_GAP * s;
    let panel_w = (PANEL_W * s).min(screen.width() - 90.0 * s);
    let panel_h = (PANEL_H * s).min(screen.height() - 90.0 * s);
    let total_w = rail_w + gap + panel_w;
    let total_h = panel_h + search_gap + search_h;
    let x = screen.left() + (screen.width() - total_w) * 0.5;
    let centered_y = screen.top() + (screen.height() - total_h) * 0.5;
    let anchored_y = screen.bottom() - total_h - 2.0 * s;
    let y = anchored_y + (centered_y - anchored_y) * ease;

    let rail = Rect::from_min_size(pos2(x, y), vec2(rail_w, panel_h));
    let panel = Rect::from_min_size(pos2(x + rail_w + gap, y), vec2(panel_w, panel_h));
    let search_w = (430.0 * s).min(panel_w * 0.82);
    let search = Rect::from_min_size(
        pos2(
            panel.left() + (panel_w - search_w) * 0.5,
            y + panel_h + search_gap,
        ),
        vec2(search_w, search_h),
    );

    let dim_base = if state.prev_path.is_empty() {
        0.22
    } else {
        0.55
    };
    let dim_alpha = (cfg.game.peekgg.cl.opacity as f32 / 100.0) * dim_base * (0.15 + 0.85 * ease);
    let painter = ui.painter().with_clip_rect(screen);
    painter.rect_filled(
        screen,
        CornerRadius::ZERO,
        by_utf8xbot_4513(Color32::from_rgb(5, 5, 8), dim_alpha),
    );

    ui_state.add_blur_rect(rail, 10.0 * s);
    ui_state.add_blur_rect(panel, 12.0 * s);
    ui_state.add_blur_rect(search, (search_h * 0.5) as f32);

    let glass = by_utf8xbot_4513(COL_GLASS_FILL, 0.9 * ease.max(0.2));
    painter.rect_filled(rail, CornerRadius::same(10), glass);
    painter.rect_filled(panel, CornerRadius::same(12), glass);
    by_utf8xbot_4510(&painter, rail, 10.0, 1.0, by_utf8xbot_4513(COL_BORDER, 0.5));
    by_utf8xbot_4510(
        &painter,
        panel,
        12.0,
        1.0,
        by_utf8xbot_4513(COL_BORDER, 0.5),
    );

    let close_clicked = by_utf8xbot_4600(ui, state, cfg, &screen, &rail, &panel, &search, s);
    if close_clicked {
        let prev = pstate::by_utf8xbot_4406();
        cfg.engine
            .ui
            .path
            .route(if prev.is_empty() { "" } else { prev.as_str() });
        ui_state.is_ui_open = false;
        return;
    }

    by_utf8xbot_4700(ui, state, cfg, &panel, s);
    by_utf8xbot_4601(ui, state, cfg, &search, s);

    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        let prev = pstate::by_utf8xbot_4406();
        cfg.engine
            .ui
            .path
            .route(if prev.is_empty() { "" } else { prev.as_str() });
        ui_state.is_ui_open = false;
    }
}

fn by_utf8xbot_4600(
    ui: &mut egui::Ui,
    state: &mut State,
    cfg: &mut Config,
    screen: &Rect,
    rail: &Rect,
    panel: &Rect,
    _search: &Rect,
    s: f32,
) -> bool {
    let painter = ui.painter().with_clip_rect(*screen);
    let item = RAIL_ITEM * s;
    for (i, page) in PAGES.iter().enumerate() {
        let rect = if i == 2 {
            Rect::from_min_size(
                pos2(
                    rail.left() + (rail.width() - item) * 0.5,
                    rail.bottom() - item - 14.0 * s,
                ),
                vec2(item, item),
            )
        } else {
            Rect::from_min_size(
                pos2(
                    rail.left() + (rail.width() - item) * 0.5,
                    rail.top() + 14.0 * s + i as f32 * (item + 10.0 * s),
                ),
                vec2(item, item),
            )
        };
        let active = state.page.0 == *page;
        let act = pstate::by_utf8xbot_4409(
            state,
            &format!("nav{i}"),
            if active { 1.0 } else { 0.0 },
            16.0,
        );
        let hover = ui.rect_contains_pointer(rect);

        if act > 0.01 {
            by_utf8xbot_4510(
                &painter,
                rect,
                rect.width() * 0.5,
                4.5 * s,
                by_utf8xbot_4513(COL_ACCENT, 0.12 * act),
            );
            by_utf8xbot_4510(
                &painter,
                rect,
                rect.width() * 0.5,
                1.3 * s,
                by_utf8xbot_4513(COL_ACCENT, 0.95 * act),
            );
        }
        if act < 0.99 {
            by_utf8xbot_4510(
                &painter,
                rect,
                rect.width() * 0.5,
                1.0 * s,
                by_utf8xbot_4513(
                    Color32::WHITE,
                    if hover { 0.32 } else { 0.14 } * (1.0 - act),
                ),
            );
        }

        let icon_color = by_utf8xbot_4513(COL_TEXT, 0.55 + 0.45 * act);
        match page {
            Page::Visual => by_utf8xbot_4521(&painter, rect.shrink(item * 0.24), icon_color),
            Page::Accessories => by_utf8xbot_4522(&painter, rect.shrink(item * 0.24), icon_color),
            Page::Settings => by_utf8xbot_4523(&painter, rect.shrink(item * 0.24), icon_color),
        }

        if by_utf8xbot_4511(ui, rect, &format!("nav{i}")) && !active {
            let previous = state.page.by_utf8xbot_4401();
            state.page_scroll[previous] = state.scroll;
            state.page = pstate::PageState::by_utf8xbot_4402(i);
            state.scroll = state.page_scroll[i];
            state.page_anim = 0.0;
            state.dropdown = None;
            state.color_popup = None;
        }
    }

    let header_h = HEADER_H * s;
    let title_rect = Rect::from_min_size(
        pos2(panel.left() + 16.0 * s, panel.top()),
        vec2(300.0 * s, header_h),
    );
    by_utf8xbot_4512(
        &painter,
        title_rect,
        state.page.0.by_utf8xbot_4201(),
        12.0 * s,
        Align2::LEFT_CENTER,
        COL_TEXT,
    );

    let close_size = 20.0 * s;
    let close_rect = Rect::from_min_size(
        pos2(
            panel.right() - close_size - 18.0 * s,
            panel.top() + (header_h - close_size) * 0.5,
        ),
        vec2(close_size, close_size),
    );
    let hover = ui.rect_contains_pointer(close_rect);
    by_utf8xbot_4510(
        &painter,
        close_rect,
        close_size * 0.5,
        1.0 * s,
        by_utf8xbot_4513(Color32::WHITE, if hover { 0.45 } else { 0.2 }),
    );
    by_utf8xbot_4526(
        &painter,
        close_rect.shrink(close_size * 0.28),
        if hover { COL_TEXT } else { COL_MUTED },
    );

    let _ = cfg;
    by_utf8xbot_4511(ui, close_rect, "close")
}

fn by_utf8xbot_4601(ui: &mut egui::Ui, state: &mut State, cfg: &mut Config, search: &Rect, s: f32) {
    let painter = ui.painter().with_clip_rect(*search);
    let focused = ui.memory(|m| m.has_focus(Id::new(("peekgg", "search"))));
    painter.rect_filled(
        *search,
        CornerRadius::same((search.height() * 0.5) as u8),
        by_utf8xbot_4513(Color32::from_rgb(3, 4, 6), 0.72),
    );
    if focused {
        by_utf8xbot_4510(
            &painter,
            *search,
            search.height() * 0.5,
            4.0 * s,
            by_utf8xbot_4513(COL_ACCENT, 0.14),
        );
    }
    by_utf8xbot_4510(
        &painter,
        *search,
        search.height() * 0.5,
        1.2 * s,
        by_utf8xbot_4513(Color32::WHITE, if focused { 0.45 } else { 0.24 }),
    );

    let icon = Rect::from_min_size(
        pos2(
            search.left() + 6.0 * s,
            search.top() + (search.height() - 14.0 * s) * 0.5,
        ),
        vec2(14.0 * s, 14.0 * s),
    );
    by_utf8xbot_4524(&painter, icon, if focused { COL_TEXT } else { COL_MUTED });

    let field = Rect::from_min_size(
        pos2(icon.right() + 4.0 * s, search.top() + 2.0 * s),
        vec2(
            search.width() - (icon.width() + 18.0 * s),
            search.height() - 4.0 * s,
        ),
    );
    ui.scope_builder(egui::UiBuilder::new().max_rect(field), |ui| {
        ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
        ui.add(
            egui::TextEdit::singleline(&mut state.search)
                .id(Id::new(("peekgg", "search")))
                .hint_text("Поиск модулей...")
                .frame(egui::Frame::NONE)
                .desired_width(field.width())
                .font(FontId::proportional(6.2 * s)),
        );
    });

    if !state.search.is_empty() {
        let clear = Rect::from_min_size(
            pos2(
                search.right() - 21.0 * s,
                search.top() + (search.height() - 17.0 * s) * 0.5,
            ),
            vec2(17.0 * s, 17.0 * s),
        );
        let hover = ui.rect_contains_pointer(clear);
        by_utf8xbot_4510(
            &painter,
            clear,
            clear.height() * 0.5,
            1.0 * s,
            by_utf8xbot_4513(Color32::WHITE, if hover { 0.45 } else { 0.22 }),
        );
        by_utf8xbot_4526(
            &painter,
            clear.shrink(5.0 * s),
            if hover { COL_TEXT } else { COL_MUTED },
        );
        if by_utf8xbot_4511(ui, clear, "search_clear") {
            state.search.clear();
        }
    }

    let _ = cfg;
}

fn by_utf8xbot_4700(ui: &mut egui::Ui, state: &mut State, cfg: &mut Config, panel: &Rect, s: f32) {
    let header_h = HEADER_H * s;
    let area = Rect::from_min_size(
        pos2(panel.left() + 22.0 * s, panel.top() + header_h + 6.0 * s),
        vec2(
            panel.width() - 44.0 * s,
            panel.height() - header_h - 6.0 * s - 20.0 * s,
        ),
    );

    if state.page_anim < 1.0 {
        state.page_anim = (state.page_anim + state.dt * 4.0).min(1.0);
    }
    let _page_ease = pstate::by_utf8xbot_4411(state.page_anim);

    let gap = CARD_GAP * s;
    let columns = if area.width() >= 596.0 * s { 3 } else { 2 };
    let card_w = (area.width() - gap * (columns as f32 - 1.0)) / columns as f32;

    let cards = state.page.0.by_utf8xbot_4202();
    let query = state.search.to_lowercase();

    if ui.rect_contains_pointer(area) {
        let wheel = ui.input(|i| i.smooth_scroll_delta.y);
        if wheel.abs() > 0.1 {
            state.scroll = (state.scroll - wheel).max(0.0);
            state.dropdown = None;
        }
    }

    let mut layout: Vec<(usize, f32, f32, Vec<usize>, bool, f32)> = Vec::new();
    let mut column_y = [0.0f32; 3];

    for (card_index, card) in cards.iter().enumerate() {
        let title_matches = query.is_empty() || card.title.to_lowercase().contains(&query);
        let mut visible: Vec<usize> = Vec::new();
        let mut rows_height = 0.0f32;
        for (row_index, row) in card.rows.iter().enumerate() {
            if let Some((bind, expected)) = row.show_if
                && bind.by_utf8xbot_4101(cfg) != expected
            {
                continue;
            }
            if !title_matches && !query.is_empty() && !row.label.to_lowercase().contains(&query) {
                continue;
            }
            visible.push(row_index);
            rows_height += by_utf8xbot_4710(row, s);
        }
        let collapsed = query.is_empty()
            && *state
                .collapsed
                .get(&(state.page.by_utf8xbot_4401(), card_index))
                .unwrap_or(&false);
        if visible.is_empty() && !title_matches {
            continue;
        }
        let card_anim = pstate::by_utf8xbot_4409(
            state,
            &format!("card{}{}", state.page.by_utf8xbot_4401(), card_index),
            if collapsed { 0.0 } else { 1.0 },
            9.0 * s.max(1.0),
        );
        let ease_anim = pstate::by_utf8xbot_4411(card_anim);
        let card_h = 8.0 * s + 14.0 * s + rows_height * ease_anim + 7.0 * s;

        let mut col = 0;
        for i in 1..columns {
            if column_y[i] < column_y[col] {
                col = i;
            }
        }
        let outer_y = column_y[col];
        column_y[col] += card_h + gap;
        layout.push((
            card_index,
            col as f32 * (card_w + gap),
            outer_y,
            visible,
            collapsed,
            card_h,
        ));
    }

    let content_height = column_y.iter().cloned().fold(0.0f32, f32::max);
    let max_scroll = (content_height - area.height()).max(0.0);
    state.scroll = state.scroll.clamp(0.0, max_scroll);

    let painter = ui.painter().with_clip_rect(area);
    for (card_index, x_off, y_off, visible, collapsed, card_h) in layout {
        let card = &cards[card_index];
        let outer = Rect::from_min_size(
            pos2(area.left() + x_off, area.top() + y_off - state.scroll),
            vec2(card_w, card_h),
        );
        if outer.bottom() < area.top() - 4.0 * s || outer.top() > area.bottom() + 4.0 * s {
            continue;
        }
        by_utf8xbot_4510(
            &painter,
            outer,
            12.0 * s,
            1.0 * s,
            by_utf8xbot_4513(Color32::WHITE, 0.22),
        );

        let inner = outer.shrink(1.0 * s);
        let dot = Rect::from_min_size(
            pos2(inner.left() + 9.0 * s, inner.top() + 9.0 * s),
            vec2(6.0 * s, 6.0 * s),
        );
        painter.circle_filled(
            dot.center(),
            dot.width() * 0.5 + 2.0 * s,
            by_utf8xbot_4513(card.color, 0.25),
        );
        painter.circle_filled(dot.center(), dot.width() * 0.5, card.color);

        let title_rect = Rect::from_min_size(
            pos2(dot.right() + 6.0 * s, inner.top() + 6.5 * s),
            vec2(
                inner.width() - (dot.right() + 6.0 * s - inner.left()) - 8.0 * s,
                10.0 * s,
            ),
        );
        by_utf8xbot_4512(
            &painter,
            title_rect,
            card.title,
            6.5 * s,
            Align2::LEFT_CENTER,
            by_utf8xbot_4513(COL_TEXT, 0.94),
        );

        let card_header = Rect::from_min_size(inner.min, vec2(inner.width(), CARD_HEADER_H * s));
        let header_hover = ui.rect_contains_pointer(card_header);
        let chevron = Rect::from_min_size(
            pos2(
                card_header.right() - 18.0 * s,
                card_header.top() + (CARD_HEADER_H * s - 14.0 * s) * 0.5,
            ),
            vec2(14.0 * s, 14.0 * s),
        );
        by_utf8xbot_4525(
            &painter,
            chevron,
            collapsed,
            if header_hover { COL_TEXT } else { COL_MUTED },
        );

        if !collapsed {
            let inner_painter = painter.with_clip_rect(outer);
            by_utf8xbot_4720(
                ui,
                state,
                cfg,
                card,
                &inner_painter,
                &inner,
                &visible,
                s,
                card_index,
            );
        }

        if by_utf8xbot_4511(
            ui,
            card_header,
            &format!("card{}{}", state.page.by_utf8xbot_4401(), card_index),
        ) && state.search.is_empty()
        {
            let key = (state.page.by_utf8xbot_4401(), card_index);
            let entry = state.collapsed.entry(key).or_insert(false);
            *entry = !*entry;
        }
    }

    if state.page.0 == Page::Settings {
        let hint = Rect::from_min_size(
            pos2(area.left(), area.bottom() + 3.0 * s),
            vec2(area.width(), 11.0 * s),
        );
        by_utf8xbot_4512(
            &painter,
            hint,
            "Клик по кнопке клавиши и нажми любую клавишу, чтобы сменить бинд.",
            5.5 * s,
            Align2::LEFT_CENTER,
            COL_DIM,
        );
    }
}

fn by_utf8xbot_4710(row: &Row, s: f32) -> f32 {
    match row.kind {
        RowKind::Toggle => ROW_TOGGLE_H * s,
        RowKind::Label => ROW_LABEL_H * s,
        RowKind::Key
        | RowKind::Dropdown
        | RowKind::Step
        | RowKind::Color
        | RowKind::Pick
        | RowKind::Ratio => ROW_WIDE_H * s,
        RowKind::Slider => ROW_SLIDER_H * s,
    }
}

#[allow(clippy::too_many_arguments)]
fn by_utf8xbot_4720(
    ui: &mut egui::Ui,
    state: &mut State,
    cfg: &mut Config,
    card: &Card,
    painter: &egui::Painter,
    inner: &Rect,
    visible: &[usize],
    s: f32,
    card_index: usize,
) {
    let mut row_y = inner.top() + 8.0 * s + 14.0 * s;
    for (visible_index, row_index) in visible.iter().enumerate() {
        let row = &card.rows[*row_index];
        let row_h = by_utf8xbot_4710(row, s);
        let row_rect = Rect::from_min_size(
            pos2(inner.left() + 9.0 * s, row_y + 1.0 * s),
            vec2(inner.width() - 18.0 * s, row_h - 2.0 * s),
        );
        let id = format!("r{card_index}.{visible_index}");

        let on = row
            .bind
            .map(|bind| bind.by_utf8xbot_4101(cfg) != 0)
            .unwrap_or(false);
        let pill = Rect::from_min_size(
            pos2(row_rect.left() - 5.0 * s, row_rect.top()),
            vec2(row_rect.width() + 10.0 * s, row_rect.height()),
        );
        let on_anim =
            pstate::by_utf8xbot_4409(state, &format!("on{id}"), if on { 1.0 } else { 0.0 }, 11.0);
        if on_anim > 0.01 {
            for (dist, alpha) in [(3.0, 0.10), (6.5, 0.06), (10.5, 0.03)] {
                let glow = pill.expand(dist * s);
                by_utf8xbot_4510(
                    painter,
                    glow,
                    glow.height() * 0.5,
                    dist * s,
                    by_utf8xbot_4513(COL_ACCENT, alpha * on_anim),
                );
            }
        }
        by_utf8xbot_4510(
            painter,
            pill,
            pill.height() * 0.5,
            1.1 * s,
            by_utf8xbot_4514(
                by_utf8xbot_4513(Color32::WHITE, 0.10),
                by_utf8xbot_4513(COL_ACCENT, 0.62),
                on_anim,
            ),
        );

        let label_color = if on { COL_TEXT } else { COL_MUTED };
        match row.kind {
            RowKind::Toggle => {
                let label = Rect::from_min_size(
                    row_rect.min,
                    vec2(row_rect.width() - 36.0 * s, row_rect.height()),
                );
                by_utf8xbot_4512(
                    painter,
                    label,
                    row.label,
                    7.0 * s,
                    Align2::LEFT_CENTER,
                    label_color,
                );
                let toggle_rect = Rect::from_min_size(
                    pos2(
                        row_rect.right() - 32.0 * s,
                        row_rect.top() + (row_rect.height() - 11.5 * s) * 0.5,
                    ),
                    vec2(24.0 * s, 11.5 * s),
                );
                by_utf8xbot_4730(ui, state, cfg, &id, row.bind, painter, toggle_rect, s);
            }
            RowKind::Slider => {
                let label = Rect::from_min_size(
                    row_rect.min,
                    vec2(row_rect.width() - 112.0 * s, row_rect.height()),
                );
                by_utf8xbot_4512(
                    painter,
                    label,
                    row.label,
                    7.0 * s,
                    Align2::LEFT_CENTER,
                    label_color,
                );
                by_utf8xbot_4740(ui, cfg, &id, row, painter, row_rect, s);
            }
            RowKind::Dropdown => {
                let label = Rect::from_min_size(
                    row_rect.min,
                    vec2(row_rect.width() - 124.0 * s, row_rect.height()),
                );
                by_utf8xbot_4512(
                    painter,
                    label,
                    row.label,
                    7.0 * s,
                    Align2::LEFT_CENTER,
                    COL_MUTED,
                );
                by_utf8xbot_4750(
                    ui, state, cfg, &id, row, painter, row_rect, s, card_index, *row_index,
                );
            }
            RowKind::Step => {
                let label = Rect::from_min_size(
                    row_rect.min,
                    vec2(row_rect.width() - 102.0 * s, row_rect.height()),
                );
                by_utf8xbot_4512(
                    painter,
                    label,
                    row.label,
                    7.0 * s,
                    Align2::LEFT_CENTER,
                    COL_MUTED,
                );
                by_utf8xbot_4760(ui, cfg, &id, row, painter, row_rect, s);
            }
            RowKind::Color => {
                let label = Rect::from_min_size(
                    row_rect.min,
                    vec2(row_rect.width() - 44.0 * s, row_rect.height()),
                );
                by_utf8xbot_4512(
                    painter,
                    label,
                    row.label,
                    7.0 * s,
                    Align2::LEFT_CENTER,
                    COL_MUTED,
                );
                by_utf8xbot_4770(ui, state, cfg, &id, row, painter, row_rect, s);
            }
            RowKind::Pick => {
                by_utf8xbot_4780(ui, cfg, &id, row, painter, row_rect, s);
            }
            RowKind::Ratio => {
                let label = Rect::from_min_size(
                    row_rect.min,
                    vec2(row_rect.width() - 124.0 * s, row_rect.height()),
                );
                by_utf8xbot_4512(
                    painter,
                    label,
                    row.label,
                    7.0 * s,
                    Align2::LEFT_CENTER,
                    COL_MUTED,
                );
                by_utf8xbot_4790(ui, cfg, &id, row, painter, row_rect, s);
            }
            RowKind::Key => {
                by_utf8xbot_4800(ui, state, cfg, painter, row_rect, s);
            }
            RowKind::Label => {
                by_utf8xbot_4512(
                    painter,
                    row_rect,
                    row.label,
                    6.0 * s,
                    Align2::LEFT_CENTER,
                    COL_DIM,
                );
            }
        }

        row_y += row_h;
    }
}

fn by_utf8xbot_4730(
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
            Stroke::new(5.5 * s, by_utf8xbot_4513(COL_ACCENT, 0.05 * t)),
            StrokeKind::Outside,
        );
        painter.rect_stroke(
            rect.expand(2.5 * s),
            CornerRadius::same(255),
            Stroke::new(2.5 * s, by_utf8xbot_4513(COL_ACCENT, 0.09 * t)),
            StrokeKind::Outside,
        );
    }

    painter.rect_filled(
        rect,
        CornerRadius::same(255),
        by_utf8xbot_4514(
            by_utf8xbot_4513(Color32::WHITE, 0.16),
            by_utf8xbot_4513(COL_ACCENT, 0.92),
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

fn by_utf8xbot_4740(
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

    painter.rect_filled(track, CornerRadius::same(2), COL_TRACK);
    let filled = Rect::from_min_size(track.min, vec2(track.width() * ratio, track.height()));
    painter.rect_filled(
        filled,
        CornerRadius::same(2),
        COL_ACCENT.gamma_multiply(0.85),
    );

    let knob = pos2(track.left() + track.width() * ratio, track.center().y);
    painter.circle_filled(knob, 5.0 * s, by_utf8xbot_4513(COL_TEXT, 0.95));
    painter.circle_filled(knob, 3.0 * s, COL_ACCENT);

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
        COL_MUTED,
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
fn by_utf8xbot_4750(
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
        COL_TEXT,
    );
    let arrow = Rect::from_min_size(
        pos2(
            rect.right() - 12.0 * s,
            rect.top() + (rect.height() - 8.0 * s) * 0.5,
        ),
        vec2(8.0 * s, 8.0 * s),
    );
    by_utf8xbot_4525(painter, arrow, true, COL_MUTED);

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
            by_utf8xbot_4513(COL_PANEL_FILL, 0.98),
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
                        by_utf8xbot_4513(COL_ACCENT, 0.35)
                    } else {
                        COL_HOVER
                    },
                );
            }
            by_utf8xbot_4512(
                painter,
                item,
                option,
                6.2 * s,
                Align2::LEFT_CENTER,
                if selected { COL_TEXT } else { COL_MUTED },
            );
            if by_utf8xbot_4511(ui, item, &format!("ddi{id}.{i}")) {
                bind.by_utf8xbot_4102(cfg, i as i64);
                state.dropdown = None;
            }
        }
    }
}

fn by_utf8xbot_4760(
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
        COL_MUTED,
    );
    by_utf8xbot_4512(
        painter,
        plus,
        "+",
        8.0 * s,
        Align2::CENTER_CENTER,
        COL_MUTED,
    );
    by_utf8xbot_4512(
        painter,
        rect,
        &value.to_string(),
        6.5 * s,
        Align2::CENTER_CENTER,
        COL_TEXT,
    );

    let changed = ui.input(|i| i.modifiers.shift).then_some(10).unwrap_or(1);
    if by_utf8xbot_4511(ui, minus, &format!("st-{id}")) {
        bind.by_utf8xbot_4102(cfg, (value - changed).clamp(row.min, row.max));
    }
    if by_utf8xbot_4511(ui, plus, &format!("st+{id}")) {
        bind.by_utf8xbot_4102(cfg, (value + changed).clamp(row.min, row.max));
    }
}

fn by_utf8xbot_4770(
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
            by_utf8xbot_4513(COL_PANEL_FILL, 0.98),
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
                COL_MUTED,
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
                COL_TEXT,
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

fn by_utf8xbot_4780(
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
        if on { COL_TEXT } else { COL_MUTED },
    );

    if by_utf8xbot_4511(ui, row_rect, &format!("pk{id}")) {
        bind.by_utf8xbot_4102(cfg, row.min);
    }
}

fn by_utf8xbot_4790(
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
        COL_TEXT,
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
        COL_MUTED,
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

fn by_utf8xbot_4800(
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
        COL_MUTED,
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
            COL_ACCENT
        } else {
            COL_TEXT
        },
    );

    if by_utf8xbot_4511(ui, button, "keybind") {
        state.awaiting_key = true;
    }
}

pub fn by_utf8xbot_4810(rect: Rect, size: Vec2) -> Rect {
    Rect::from_min_size(rect.min, size)
}

pub fn by_utf8xbot_4811(pos: Pos2) -> Vec2 {
    vec2(pos.x, pos.y)
}
