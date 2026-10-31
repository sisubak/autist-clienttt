use egui::{Color32, CornerRadius, Stroke};

use super::bind::{Bind, ColorBind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowKind {
    Toggle,
    Slider,
    Key,
    Label,
    Dropdown,
    Step,
    Color,
    Pick,
    Ratio,
}

pub struct Row {
    pub label: &'static str,
    pub kind: RowKind,
    pub bind: Option<Bind>,
    pub color: Option<ColorBind>,
    pub min: i64,
    pub max: i64,
    pub suffix: &'static str,
    pub options: &'static [&'static str],
    pub show_if: Option<(Bind, i64)>,
}

pub struct Card {
    pub title: &'static str,
    pub color: Color32,
    pub rows: &'static [Row],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Visual,
    Accessories,
    Settings,
}

impl Page {
    pub fn by_utf8xbot_4201(self) -> &'static str {
        match self {
            Page::Visual => "ВИЗУАЛ",
            Page::Accessories => "АКСЕССУАРЫ",
            Page::Settings => "НАСТРОЙКИ",
        }
    }

    pub fn by_utf8xbot_4202(self) -> &'static [Card] {
        match self {
            Page::Visual => VISUAL_CARDS,
            Page::Accessories => ACCESSORY_CARDS,
            Page::Settings => SETTINGS_CARDS,
        }
    }
}

pub const PAGES: [Page; 3] = [Page::Visual, Page::Accessories, Page::Settings];

pub const HOOK_STYLES: &[&str] = &[
    "None",
    "Whirlwind",
    "Black Hole",
    "Magic",
    "Fire",
    "Troll",
    "Rainbow",
    "Lightning",
    "Tentacle",
];

pub const RATIO_PRESETS: &[&str] = &[
    "4:3", "16:10", "16:9", "21:9", "32:9", "5:4", "3:2", "1:1", "64:27",
];

pub const GRADIENT_MODES: &[&str] = &["Skin Colors", "Custom", "Rainbow"];
pub const ASPECT_MODES: &[&str] = &["Выкл", "Пресет", "Свой"];
pub const WEATHER_TYPES: &[&str] = &["Снег", "Дождь", "Светлячки", "Пыль"];

macro_rules! row {
    ($label:expr, $kind:ident, $bind:expr, $min:expr, $max:expr) => {
        Row {
            label: $label,
            kind: RowKind::$kind,
            bind: Some($bind),
            color: None,
            min: $min,
            max: $max,
            suffix: "",
            options: &[],
            show_if: None,
        }
    };
    ($label:expr, $kind:ident, $bind:expr, $min:expr, $max:expr, $suffix:expr) => {
        Row {
            label: $label,
            kind: RowKind::$kind,
            bind: Some($bind),
            color: None,
            min: $min,
            max: $max,
            suffix: $suffix,
            options: &[],
            show_if: None,
        }
    };
    ($label:expr, $kind:ident, $bind:expr, $min:expr, $max:expr, $suffix:expr, $options:expr) => {
        Row {
            label: $label,
            kind: RowKind::$kind,
            bind: Some($bind),
            color: None,
            min: $min,
            max: $max,
            suffix: $suffix,
            options: $options,
            show_if: None,
        }
    };
}

macro_rules! color_row {
    ($label:expr, $color:expr) => {
        Row {
            label: $label,
            kind: RowKind::Color,
            bind: None,
            color: Some($color),
            min: 0,
            max: 0,
            suffix: "",
            options: &[],
            show_if: None,
        }
    };
}

macro_rules! gated_row {
    ($label:expr, $kind:ident, $bind:expr, $min:expr, $max:expr, $if_bind:expr, $if_val:expr) => {
        Row {
            label: $label,
            kind: RowKind::$kind,
            bind: Some($bind),
            color: None,
            min: $min,
            max: $max,
            suffix: "",
            options: &[],
            show_if: Some(($if_bind, $if_val)),
        }
    };
}

pub const KEY_ROWS: &[Row] = &[Row {
    label: "Открыть меню",
    kind: RowKind::Key,
    bind: None,
    color: None,
    min: 0,
    max: 0,
    suffix: "",
    options: &[],
    show_if: None,
}];

pub const INTERFACE_ROWS: &[Row] = &[
    row!("Прозрачность фона", Slider, Bind::Opacity, 0, 100, "%"),
    row!("Масштаб меню", Slider, Bind::Scale, 70, 130, "%"),
    row!("Показывать FPS", Toggle, Bind::ShowFps, 0, 1),
    row!("Анимации UI", Toggle, Bind::BcAnimations, 0, 1),
    row!("Подтверждение выхода", Toggle, Bind::BcConfirmQuit, 0, 1),
    row!("Масштаб колеса", Slider, Bind::BcWheelScale, 50, 100, "%"),
];

pub const SOUND_ROWS: &[Row] = &[
    row!("Тихие хуки чужих", Toggle, Bind::BcMuteOthersHook, 0, 1),
    row!("Тихий хаммер чужих", Toggle, Bind::BcMuteOthersHammer, 0, 1),
    row!(
        "Тихий двойной прыжок",
        Toggle,
        Bind::BcMuteOthersAirJump,
        0,
        1
    ),
];

pub const HUD_ROWS: &[Row] = &[
    row!("Включить HUD", Toggle, Bind::ClShowHud, 0, 1),
    row!("Полоски здоровья", Toggle, Bind::ClShowHudHealthAmmo, 0, 1),
    row!("DDRace HUD", Toggle, Bind::ClShowHudDdrace, 0, 1),
    row!("Прыжки", Toggle, Bind::ClShowHudJumpsIndicator, 0, 1),
    row!("Реальный хитбокс", Toggle, Bind::BcShowRealHitbox, 0, 1),
];

pub const TIMER_ROWS: &[Row] = &[
    row!("Таймер", Toggle, Bind::ClShowHudTimer, 0, 1),
    row!("Разница КП", Toggle, Bind::ClShowHudTimeCpDiff, 0, 1),
    row!(
        "Позиция игрока",
        Toggle,
        Bind::ClShowHudPlayerPosition,
        0,
        1
    ),
    row!("Скорость", Toggle, Bind::ClShowHudPlayerSpeed, 0, 1),
    row!("Правильный КП", Toggle, Bind::BcShowCorrectCheckpoint, 0, 1),
];

pub const SCOREBOARD_ROWS: &[Row] = &[
    row!(
        "Скорборд при смерти",
        Toggle,
        Bind::ClScoreboardOnDeath,
        0,
        1
    ),
    row!("Показывать ID", Toggle, Bind::ClShowIds, 0, 1),
    row!("Очки в табе", Toggle, Bind::BcShowPointsInTab, 0, 1),
    row!("Имена зрителей", Toggle, Bind::BcShowSpectatorNames, 0, 1),
    row!("Проценты голосов", Toggle, Bind::BcShowVotePercentage, 0, 1),
    row!("Счёт зрителей", Toggle, Bind::ClShowHudSpectatorCount, 0, 1),
    row!("Масштаб", Slider, Bind::BcScoreboardScale, 50, 100, "%"),
];

pub const GRADIENT_ROWS: &[Row] = &[
    row!("Ники", Toggle, Bind::BcNameplateGradient, 0, 1),
    row!(
        "Режим",
        Dropdown,
        Bind::BcNameplateGradientMode,
        0,
        2,
        "",
        GRADIENT_MODES
    ),
    row!("Клан", Toggle, Bind::BcNameplateGradientClan, 0, 1),
    row!(
        "Всё подряд",
        Toggle,
        Bind::BcNameplateGradientEverything,
        0,
        1
    ),
    row!(
        "Скорость",
        Slider,
        Bind::BcNameplateGradientAnimateSpeed,
        1,
        100
    ),
];

pub const FREEZE_ROWS: &[Row] = &[
    row!("Включить", Toggle, Bind::BcFreezePull, 0, 1),
    row!("Щит", Toggle, Bind::BcFreezePullShield, 0, 1),
    row!("Дальность", Slider, Bind::BcFreezePullRange, 60, 260),
    row!("Количество", Slider, Bind::BcFreezePullStrands, 1, 8),
    row!("Мороз", Slider, Bind::BcFreezePullSparks, 0, 300),
    row!("Цвет как у хука", Toggle, Bind::BcFreezePullHookColor, 0, 1),
    color_row!("Холодный цвет", ColorBind::FreezePull),
];

pub const HOOK_ROWS: &[Row] = &[
    row!("Хук", Dropdown, Bind::BcHookStyle, 0, 8, "", HOOK_STYLES),
    row!("Стилизовать хук", Toggle, Bind::BcHookTip, 0, 1),
    row!("Время роста", Slider, Bind::BcHookChargeTime, 200, 4000),
    row!("Размер", Slider, Bind::BcHookSize, 50, 200),
];

pub const GLOW_ROWS: &[Row] = &[
    row!("Свечение лазера", Toggle, Bind::BcLaserGlow, 0, 1),
    row!("Сила лазера", Slider, Bind::BcLaserGlowPower, 0, 100, "%"),
    row!("Свечение дробовика", Toggle, Bind::BcShotgunGlow, 0, 1),
    row!(
        "Сила дробовика",
        Slider,
        Bind::BcShotgunGlowPower,
        0,
        100,
        "%"
    ),
];

pub const FX_ROWS: &[Row] = &[
    row!("Jelly tee", Toggle, Bind::BcJellyTee, 0, 1),
    row!("Jelly у других", Toggle, Bind::BcJellyTeeOthers, 0, 1),
    row!("Сила Jelly", Slider, Bind::BcJellyTeeStrength, 1, 500),
    row!("Gores режим", Toggle, Bind::BcGoresMode, 0, 1),
];

pub const PARTICLE_ROWS: &[Row] = &[
    row!("3D частицы", Toggle, Bind::BcParticles3d, 0, 1),
    row!("Плотность", Slider, Bind::BcParticles3dDensity, 1, 600),
    row!("Размер", Slider, Bind::BcParticles3dSizeMax, 2, 200),
    row!("Свечение частиц", Toggle, Bind::BcParticles3dGlow, 0, 1),
];

pub const CAMERA_ROWS: &[Row] = &[
    row!("Кинематогр. камера", Toggle, Bind::BcCinematicCamera, 0, 1),
    row!(
        "Сила камеры",
        Slider,
        Bind::BcCinematicCameraStrength,
        0,
        100,
        "%"
    ),
    row!("Dynamic FOV", Toggle, Bind::BcDynamicFov, 0, 1),
    row!("Лучший спектатор", Toggle, Bind::BcBetterSpectate, 0, 1),
    row!("Уведомление moved", Toggle, Bind::BcSpecMovedNotify, 0, 1),
];

pub const CURSOR_ROWS: &[Row] = &[
    row!("Трейл курсора", Toggle, Bind::BcCursorTrail, 0, 1),
    row!(
        "Размер трейла",
        Slider,
        Bind::BcCursorTrailTrailSize,
        10,
        100
    ),
    row!(
        "Прозрачность",
        Slider,
        Bind::BcCursorTrailOpacity,
        0,
        100,
        "%"
    ),
];

pub const WEATHER_ROWS: &[Row] = &[
    row!("Включить погоду", Toggle, Bind::BcAtmosphere, 0, 1),
    row!("Тип", Dropdown, Bind::BcWeather, 0, 3, "", WEATHER_TYPES),
    row!("Количество", Slider, Bind::BcWeatherAmount, 25, 300),
    row!("Размер капель", Slider, Bind::BcWeatherRainSize, 25, 300),
    row!("Скорость", Slider, Bind::BcWeatherRainSpeed, 25, 300),
];

pub const ASPECT_ROWS: &[Row] = &[
    row!(
        "Режим",
        Dropdown,
        Bind::BcCustomAspectRatioMode,
        0,
        2,
        "",
        ASPECT_MODES
    ),
    gated_row!(
        "Пресет",
        Ratio,
        Bind::BcCustomAspectRatio,
        0,
        0,
        Bind::BcCustomAspectRatioMode,
        1
    ),
    gated_row!(
        "Ширина",
        Step,
        Bind::BcCustomAspectRatioNum,
        1,
        100,
        Bind::BcCustomAspectRatioMode,
        2
    ),
    gated_row!(
        "Высота",
        Step,
        Bind::BcCustomAspectRatioDen,
        1,
        100,
        Bind::BcCustomAspectRatioMode,
        2
    ),
];

pub const WORLD_ROWS: &[Row] = &[
    row!("Показ других", Slider, Bind::ClShowOthers, 0, 2),
    row!(
        "Прозрачность других",
        Slider,
        Bind::ClShowOthersAlpha,
        0,
        100,
        "%"
    ),
    row!(
        "Прозрачность тайлов",
        Slider,
        Bind::ClOverlayEntities,
        0,
        100,
        "%"
    ),
    row!("Фриз-бары", Toggle, Bind::ClShowFreezeBars, 0, 1),
    row!("Затемнение фриза", Slider, Bind::BcFrozenSkinDarken, 0, 70),
    row!("Хук-коллизия своя", Slider, Bind::ClShowHookCollOwn, 0, 2),
    row!(
        "Хук-коллизия чужая",
        Slider,
        Bind::ClShowHookCollOther,
        0,
        2
    ),
    row!(
        "Индикатор дамми",
        Toggle,
        Bind::BcShowhudDummyCoordIndicator,
        0,
        1
    ),
];

pub const AURA_ROWS: &[Row] = &[
    row!("Включить ауру", Toggle, Bind::Aura, 0, 1),
    row!("Размер", Slider, Bind::AuraRadius, 100, 300, "%"),
    row!("Прозрачность", Slider, Bind::AuraAlpha, 0, 100, "%"),
    color_row!("Цвет", ColorBind::Aura),
    row!("Перелив ауры", Toggle, Bind::AuraRainbow, 0, 1),
];

pub const OUTLINE_ROWS: &[Row] = &[
    row!("Обводка ти", Toggle, Bind::Outline, 0, 1),
    color_row!("Цвет обводки", ColorBind::Outline),
    row!("Перелив обводки", Toggle, Bind::OutlineRainbow, 0, 1),
];

pub const JUMP_ROWS: &[Row] = &[
    row!("Кольцо прыжка", Toggle, Bind::JumpRing, 0, 1),
    color_row!("Цвет кольца", ColorBind::Ring),
    row!("Перелив кольца", Toggle, Bind::RingRainbow, 0, 1),
];

pub const ACCESSORY_ROWS: &[Row] = &[
    row!("Нет", Pick, Bind::Accessory, 0, 0),
    row!("Китайская шляпка", Pick, Bind::Accessory, 1, 0),
    row!("Крылья ангела", Pick, Bind::Accessory, 2, 0),
    row!("Крылья демона", Pick, Bind::Accessory, 3, 0),
    color_row!("Цвет шляпки", ColorBind::Hat),
    row!("Перелив шляпы", Toggle, Bind::HatRainbow, 0, 1),
];

pub const VISUAL_CARDS: &[Card] = &[
    Card {
        title: "ГРАДИЕНТ",
        color: Color32::from_rgb(184, 153, 255),
        rows: GRADIENT_ROWS,
    },
    Card {
        title: "FREEZE LIFESTEAL",
        color: Color32::from_rgb(140, 217, 255),
        rows: FREEZE_ROWS,
    },
    Card {
        title: "CUSTOM HOOKS",
        color: Color32::from_rgb(204, 140, 255),
        rows: HOOK_ROWS,
    },
    Card {
        title: "ASPECT RATIO",
        color: Color32::from_rgb(115, 191, 255),
        rows: ASPECT_ROWS,
    },
    Card {
        title: "WEAPON GLOW",
        color: Color32::from_rgb(255, 140, 115),
        rows: GLOW_ROWS,
    },
    Card {
        title: "МИР",
        color: Color32::from_rgb(255, 128, 168),
        rows: WORLD_ROWS,
    },
    Card {
        title: "ЭФФЕКТЫ",
        color: Color32::from_rgb(255, 107, 107),
        rows: FX_ROWS,
    },
    Card {
        title: "ЧАСТИЦЫ",
        color: Color32::from_rgb(140, 217, 255),
        rows: PARTICLE_ROWS,
    },
    Card {
        title: "КАМЕРА",
        color: Color32::from_rgb(191, 140, 255),
        rows: CAMERA_ROWS,
    },
    Card {
        title: "КУРСОР",
        color: Color32::from_rgb(153, 242, 179),
        rows: CURSOR_ROWS,
    },
    Card {
        title: "АУРА ЦЕЛИ",
        color: Color32::from_rgb(255, 140, 51),
        rows: AURA_ROWS,
    },
    Card {
        title: "ОБВОДКА",
        color: Color32::from_rgb(140, 242, 140),
        rows: OUTLINE_ROWS,
    },
    Card {
        title: "ПРЫЖОК",
        color: Color32::from_rgb(115, 217, 255),
        rows: JUMP_ROWS,
    },
    Card {
        title: "ПОГОДА",
        color: Color32::from_rgb(140, 204, 255),
        rows: WEATHER_ROWS,
    },
];

pub const ACCESSORY_CARDS: &[Card] = &[Card {
    title: "АКСЕССУАРЫ",
    color: Color32::from_rgb(255, 199, 102),
    rows: ACCESSORY_ROWS,
}];

pub const SETTINGS_CARDS: &[Card] = &[
    Card {
        title: "УПРАВЛЕНИЕ",
        color: Color32::from_rgb(158, 122, 255),
        rows: KEY_ROWS,
    },
    Card {
        title: "ИНТЕРФЕЙС",
        color: Color32::from_rgb(102, 179, 255),
        rows: INTERFACE_ROWS,
    },
    Card {
        title: "ЗВУКИ",
        color: Color32::from_rgb(255, 179, 89),
        rows: SOUND_ROWS,
    },
];

pub fn by_utf8xbot_4203(labels: &[Row], needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let needle = needle.to_lowercase();
    labels
        .iter()
        .any(|row| row.label.to_lowercase().contains(&needle))
}

pub fn by_utf8xbot_4204(rect_h: f32) -> CornerRadius {
    CornerRadius::same(rect_h.clamp(0.0, 255.0) as u8)
}

pub fn by_utf8xbot_4205(w: f32, alpha: u8) -> Stroke {
    Stroke::new(w, Color32::from_white_alpha(alpha))
}
