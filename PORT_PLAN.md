# Порт so.loud → ddnet-rs (BestClient на Rust)

Источник: `so.loud` (C++ форк DDNet, 3082 файла, ~66k строк в `src/game/client/components/bestclient` + ~10k в `tclient` + ~3k в `peekgg`).
Цель: `ddnet-rs` (форк `sisubak/autist-clienttt`, egui-based UI, 225k строк Rust workspace).

## Почему это не «один коммит»

`so.loud` — это C++ клиент с собственным immediate-mode UI (`CUi`), прямым доступом к
`CGameClient`, `IGraphics`, тикам игры и памятью рендера. `ddnet-rs` — полностью другая
архитектура: egui-страницы (`UiPageInterface`), конфиг через proc-macro (`ConfigInterface`),
игровая логика в отдельных крейтах, UI общается с игрой через `UserData`-структуры.

Прямого переноса «файл в файл» не существует. Каждый модуль проходит три шага:

1. **Конфиг** — переменные из `config_variables_bestclient.h` (504 шт.) и `config_variables_tclient.h` → структуры с `#[config_default]` + `ConfigInterface` в `game-config`.
2. **Состояние/логика** — C++ синглтоны-компоненты → Rust-структуры в крейтах `game/*` с явным владением.
3. **Рендер** — вызовы `CUi::DoLabel/DoButton/DoRect` → egui-страницы в `client-ui`/`ingame-ui` (меню, настройки) или `UiState::custom_paints` (оверлеи поверх игры).

## Что уже в ветке (stage 1)

- `lib/ui-base/src/theme.rs` — глобальная тема UI: акцентный цвет, скругления, скорость анимаций, скроллбар, масштаб шрифта. Применяется в `default_style()`, т.е. ко всем страницам сразу.
- `game/game-config`: секция `ConfigBestClient` (`bc.ui`, `bc.discord`) — 15 живых настроек.
- `game/client-bestclient` — новый крейт:
  - `theme.rs` — маппинг конфига в `UiTheme` с дедупликацией записи.
  - `discord_rpc.rs` — полноценный Discord IPC клиент: unix-сокет `$XDG_RUNTIME_DIR/discord-ipc-N` / windows named pipe, handshake `op 0`, `SET_ACTIVITY`, ping→pong, авто-реконнект, статус соединения. Без SDK, только `std` + `serde_json`.
  - `ui/page.rs` + `ui/tabs/{interface,discord}.rs` — страница настроек с табами, состояние таба в `config.storage`.
- `game/client-ui`: кнопка `BestClient` в навигации настроек + маршрут.
- `src/client/client.rs`: синк темы каждый кадр из `config.game.bc.ui`.
- `.github/workflows/build-debian.yml`: сборка на `debian:bookworm-slim`, кэш cargo, артефакт.

## Инвентарь порта (110 модулей исходника)

### Батч 1 — UI/тема/меню (основа, без неё остальное некуда вешать)
| Модуль so.loud | Строк | Куда в Rust |
|---|---|---|
| `menus_bestclient.cpp` | 4909 | `client-ui/main_menu/settings/bestclient/*` — дерево страниц |
| `ui_theme/style.h`, `settings_nav.cpp` | 1224 | `ui-base/theme.rs` (готово) + навигация |
| `widgets.cpp`, `scroll_bar.cpp`, `ui_animations.cpp`, `gradient.cpp`, `smoother.cpp` | ~1800 | `ui-base/components/*` |
| `settings_search.cpp` | — | уже есть в ddnet-rs (`settings/search_settings`) — только расширить под новые ключи |
| `config_presets.cpp` | — | `game-config` + страница пресетов (сериализация `Config`) |
| `translate.cpp` | 861 | `game/localization` + словарь |
| `menus_games.cpp` | 3158 | лобби/кастомные игры — отдельный крейт `game/client-games` |
| `menus_assets_editor.cpp`, `profile_assets.cpp`, `demo_tee_editor.cpp` | ~2500 | редакторы ассетов на egui |
| `browser_utils.cpp`, `browser_cache.cpp`, `server_map_preview.cpp` | ~1500 | `client-ui/main_menu/content/browser/*` |

### Батч 2 — HUD/инфо-оверлеи
`keystrokes` (1022), `hud_editor` (1366), `hud_extras`, `statusbar` (368), `client_indicator` (1219),
`corner_indicator`, `player_indicator` (156), `flying_name_plates`, `show_points`, `spec_pause_radio`,
`inputs.cpp`, `quick_binds`.
Цель: `game/ingame-ui/src/hud/*` + новые egui-оверлеи, состояние берётся из
`GameState`/`ClientInfo` (аналог `CGameClient::m_aLocalIds`).

### Батч 3 — Чат и социальное
`chat_qol` (494), `chat_bubbles` (471), `chat_media` (2496), `chat_scroll` (506),
`chat_message_actions`, `gif_bubbles`, `twitch_chat` (544), `mumble`, `bindchat` (355),
`bindwheel` (351), `custom_communities` (125), `warlist` (511), `skinprofiles` (164).
Цель: `game/ingame-ui/src/chat/*`, HTTP-часть — через `base-http`; медиа-декод — `image-utils` + `gif`/`webp`.

### Батч 4 — Игровой рендер/FX
`players/jelly_tee`, `laser_glow`, `rocket_glow`, `weapon_vfx` (687), `nuke_vfx` (648),
`3d_particles` (531), `hook*` (rope/magic/fire/wind/rainbow/lightning/tentacle/troll/grip/blackhole ~3400),
`trails` (376), `outlines` (285), `moving_tiles` (273), `cursor_trail`, `aspect_ratio`, `automargin`,
`better_preview`, `edgehelper`, `optimizer`, `rollback_demo`.
Цель: `game/client-render-game/*`. Часть (aspect ratio, automargin, preview) — чистая математика камеры, переносится быстро.

### Батч 5 — Практика/игровые ассистенты
`fast_practice` (3022), `fast_actions`, `finish_prediction` (1059), `ego_finished_maps`,
`ego_tiles_prediction`, `camera_demos`, `camera_extras`, `spec_pause_radio`, `presence_cache`,
`fgf_*` (actions/glow/iceward/ki/weather), `physicball` (871), `pet` (103), `process_priority`.

### Батч 6 — Медиа
`music_player` (4504) + `music_player_lyrics` (935) + `media_decoder` (935) + `analyzer` (555) +
`visualizer/source_{wasapi,pulse,passive}` (2372) + `menu_media_background` (809).
Цель: декод через `ffmpeg`-фичу ddnet-rs (`client-demo/ffmpeg`), вывод — `sound-backend`,
визуализатор — `symphonia`/`ffmpeg` + FFT (`rustfft`), loopback — PulseAudio monitor source на Linux, WASAPI loopback на Windows.

### Батч 7 — Голос
`voice/voice.cpp` (4125). Цель: `lib/microphone` (в ddnet-rs уже есть opus/cpal) + Opus-энкодер,
jitter-буфер, свой net-канал поверх `game-network`.

### Батч 8 — Сервисное
`updater` (569) + `updater_linux`, `discord_rpc` (готово в stage 1), `admin_panel` (1204),
`peekgg_gui` (2110) + `peekgg_glass.rs` (512, уже Rust в исходнике), `antibot` интерфейс,
`serverbrowser` патчи, `text.cpp` патчи (шрифты), `backend_opengl` патчи (glass/blur).

## Оценка

| Батч | Объём | Реалистичный срок |
|---|---|---|
| 1 UI/меню | ~15k строк C++ | 3–4 недели |
| 2 HUD | ~5k | 1.5 недели |
| 3 Чат | ~6k | 2 недели |
| 4 Рендер/FX | ~7k | 3 недели |
| 5 Практика | ~6k | 2 недели |
| 6 Медиа | ~10k | 3–4 недели |
| 7 Голос | ~4k | 2 недели |
| 8 Сервис | ~5k | 2 недели |

Порядок: 1 → 2 → 5 → 3 → 4 → 6 → 7 → 8. Внутри батча модули независимы, можно резать на PR.

## Сборка

Локально:
```bash
cargo check -p client-bestclient
cargo run --release -p ddnet-rs
```
CI: пуш в `main` или `bestclient/**` → `.github/workflows/build-debian.yml` → артефакт
`ddnet-rs-debian-bookworm-<sha>.tar.gz` (бинарь + `data/`).
