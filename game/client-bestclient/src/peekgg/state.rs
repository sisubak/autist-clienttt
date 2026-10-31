use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::Duration;

use super::bind::ColorBind;
use super::data::Page;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PageState(pub Page);

impl Default for PageState {
    fn default() -> Self {
        Self(Page::Visual)
    }
}

impl PageState {
    pub fn by_utf8xbot_4401(self) -> usize {
        match self.0 {
            Page::Visual => 0,
            Page::Accessories => 1,
            Page::Settings => 2,
        }
    }

    pub fn by_utf8xbot_4402(index: usize) -> Self {
        Self(match index {
            1 => Page::Accessories,
            2 => Page::Settings,
            _ => Page::Visual,
        })
    }
}

pub struct State {
    pub open: bool,
    pub prev_path: String,
    pub page: PageState,
    pub search: String,
    pub scroll: f32,
    pub page_scroll: [f32; 3],
    pub collapsed: HashMap<(usize, usize), bool>,
    pub anims: HashMap<u64, f32>,
    pub awaiting_key: bool,
    pub open_anim: f32,
    pub last_time: Option<Duration>,
    pub dt: f32,
    pub color_popup: Option<ColorBind>,
    pub dropdown: Option<(usize, usize)>,
    pub last_page: usize,
    pub page_anim: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open: false,
            prev_path: String::new(),
            page: PageState::default(),
            search: String::new(),
            scroll: 0.0,
            page_scroll: [0.0; 3],
            collapsed: HashMap::new(),
            anims: HashMap::new(),
            awaiting_key: false,
            open_anim: 0.0,
            last_time: None,
            dt: 1.0 / 60.0,
            color_popup: None,
            dropdown: None,
            last_page: 0,
            page_anim: 1.0,
        }
    }
}

fn by_utf8xbot_4403() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(State::default()))
}

pub fn by_utf8xbot_4404() -> MutexGuard<'static, State> {
    by_utf8xbot_4403()
        .lock()
        .unwrap_or_else(|err| err.into_inner())
}

pub fn by_utf8xbot_4405(prev_path: &str) {
    let mut state = by_utf8xbot_4404();
    state.open = true;
    state.prev_path = prev_path.to_string();
    state.open_anim = 0.0;
    state.awaiting_key = false;
    state.color_popup = None;
    state.dropdown = None;
}

pub fn by_utf8xbot_4406() -> String {
    let mut state = by_utf8xbot_4404();
    state.open = false;
    state.awaiting_key = false;
    state.color_popup = None;
    state.dropdown = None;
    state.prev_path.clone()
}

pub fn by_utf8xbot_4407() -> bool {
    by_utf8xbot_4404().open
}

pub fn by_utf8xbot_4408(state: &mut State, now: &Duration) {
    let dt = match state.last_time {
        Some(last) => now.saturating_sub(last).as_secs_f32(),
        None => 1.0 / 60.0,
    };
    state.dt = dt.clamp(0.0, 0.1);
    state.last_time = Some(*now);

    let target = if state.open { 1.0 } else { 0.0 };
    let step = state.dt * if state.open { 1.9 } else { 2.2 };
    if state.open_anim < target {
        state.open_anim = (state.open_anim + step).min(target);
    } else if state.open_anim > target {
        state.open_anim = (state.open_anim - step).max(target);
    }
}

pub fn by_utf8xbot_4409(state: &mut State, id: &str, target: f32, speed: f32) -> f32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    id.hash(&mut hasher);
    let key = hasher.finish();
    let step = state.dt * speed;
    let cur = state.anims.entry(key).or_insert(target);
    if *cur < target {
        *cur = (*cur + step).min(target);
    } else if *cur > target {
        *cur = (*cur - step).max(target);
    }
    by_utf8xbot_4410(*cur)
}

pub fn by_utf8xbot_4410(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

pub fn by_utf8xbot_4411(value: f32) -> f32 {
    1.0 - (1.0 - value) * (1.0 - value)
}
