# peekgg menu — как переделывать

Меню портировано из `CPeekggGui` (`so.loud/src/game/client/components/peekgg_gui.cpp`).
Всё разложено так, чтобы перерисовка не требовала лезть в данные.

## Слои

| Файл | Что внутри | Трогать при переделке вида |
|---|---|---|
| `peekgg/theme.rs` | `MenuTheme` — все цвета, радиусы, размеры, шрифты, скорости анимаций | **да, основной файл** |
| `peekgg/data.rs` | карточки, строки, подписи, опции дропдаунов, цвета карточек | только контент |
| `peekgg/widgets.rs` | отрисовка контролов строки: тумблер, слайдер, дропдаун, степпер, свотч, пик, пресет соотношения, кнопка бинда | да, если меняется форма контрола |
| `peekgg/page.rs` | каркас: три острова (рейл/панель/поиск), диммер, блюр, хедер, раскладка карточек по колонкам, скролл, поиск | да, если меняется геометрия каркаса |
| `peekgg/state.rs` | открытие/закрытие, анимации по id, скролл по страницам, буфер поиска | редко |
| `peekgg/bind.rs` | маппинг строк на поля конфига (`Bind`, `ColorBind`) | нет |

## Тема

Всё визуальное — в `MenuTheme`. Значения по умолчанию сняты 1:1 с C++ версии.
Подмена на лету:

```rust
use client_bestclient::peekgg::theme::{by_utf8xbot_4900, by_utf8xbot_4901, MenuTheme};

let mut theme = by_utf8xbot_4900();
theme.accent = egui::Color32::from_rgb(255, 90, 90);
theme.card_radius = 8.0;
by_utf8xbot_4901(theme);
```

Поля сгруппированы: цвета (`text`, `muted`, `dim`, `accent`, `glass_fill`, `panel_fill`,
`track`, `hover`, `border`), геометрия островов (`rail_w`, `island_gap`, `panel_w/h`,
`header_h`, `margin_x/y`), поиск (`search_h`, `search_max_w`, `search_ratio`),
карточки (`card_gap`, `card_radius`, `card_header_h`, `card_dot`, `three_columns_min_w`),
строки (`row_*_h`, `row_pad_x`), контролы (`toggle_w/h`, `slider_track_w`, `swatch_w/h`,
`key_button_w/h`), шрифты (`font_*`), анимации (`open_speed`, `close_speed`, `hover_speed`,
`page_speed`, `row_glow`), затемнение (`dim_strength_in_game/offline`).

## Веб-превью

`game/client-bestclient/preview/menu.html` — тот же контент и та же геометрия на HTML/CSS.
Открывается в браузере без сборки: переключаются страницы, работают тумблеры, слайдеры,
дропдауны, сворачивание карточек и поиск «Поиск модулей...».

CSS-переменные в `:root` — зеркало `MenuTheme`. Быстрый цикл: правишь переменные в браузере →
говоришь, что зашло → переношу значения в `theme.rs` (и, если надо, в `page.rs`/`widgets.rs`
для изменения формы, а не только чисел).

## Данные

`data.rs` — это `SCard`/`SRow` из оригинала: те же подписи, порядок, опции, min/max, суффиксы,
цвета карточек, условия показа (`show_if`). Добавить/убрать карточку или строку — правка
только этого файла; никакого кода отрисовки трогать не нужно.

## Превью

- `game/client-bestclient/preview/menu_exact.html` — самодостаточная копия меню peekgg
  (canvas, числа 1:1 из `peekgg_gui.cpp`, шрифт Actay Wide Bold вшит). Подробности и
  список осознанных расхождений — `preview/PREVIEW_1TO1.md`.
- `preview/menu_exact.template.html` + `preview/build_menu_exact.py` — исходник превью
  и сборщик, который вшивает шрифты.

- `preview/menu_redesign.html` — прототип нового вида по мокапу: панели-окна,
  перетаскивание со снапом по слотам, обмен местами, стек по z, ресайз, пружины.
  Описание сетки и физики — `preview/REDESIGN_PROTO.md`.
