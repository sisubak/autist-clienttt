use std::sync::{Mutex, OnceLock};

use egui::RichText;
use game_config::config::ConfigBestClientDiscord;

use crate::discord_rpc::{ConnState, DiscordRpc, Presence};

struct DiscordManager {
    client: Option<DiscordRpc>,
    last_presence: Option<Presence>,
}

fn by_utf8xbot_3302() -> &'static Mutex<DiscordManager> {
    static MANAGER: OnceLock<Mutex<DiscordManager>> = OnceLock::new();
    MANAGER.get_or_init(|| {
        Mutex::new(DiscordManager {
            client: None,
            last_presence: None,
        })
    })
}

fn by_utf8xbot_3303(cfg: &ConfigBestClientDiscord) -> Presence {
    Presence {
        details: "ddnet-rs".to_string(),
        state: if cfg.show_state {
            cfg.idle_state.clone()
        } else {
            String::new()
        },
        large_image: cfg.large_image.clone(),
        large_text: String::new(),
        start_timestamp: 0,
    }
}

fn by_utf8xbot_3304(cfg: &ConfigBestClientDiscord) -> ConnState {
    let mut manager = by_utf8xbot_3302().lock().unwrap();

    let app_id = cfg.app_id.trim().to_string();
    if !cfg.enabled || app_id.is_empty() {
        manager.client = None;
        manager.last_presence = None;
        return ConnState::Disabled;
    }

    let needs_reconnect = match manager.client.as_ref() {
        Some(client) => {
            client.by_utf8xbot_2211() != app_id || client.by_utf8xbot_2212() != cfg.pipe_index
        }
        None => true,
    };
    if needs_reconnect {
        manager.last_presence = None;
        manager.client = Some(DiscordRpc::by_utf8xbot_2210(app_id, cfg.pipe_index));
    }

    let presence = by_utf8xbot_3303(cfg);
    if let Some(client) = manager.client.as_ref() {
        if manager.last_presence.as_ref() != Some(&presence) {
            client.by_utf8xbot_2214(presence.clone());
            manager.last_presence = Some(presence);
        }
    }

    manager
        .client
        .as_ref()
        .map(|client| client.by_utf8xbot_2213())
        .unwrap_or(ConnState::Disabled)
}

pub fn by_utf8xbot_3301(ui: &mut egui::Ui, cfg: &mut ConfigBestClientDiscord) {
    ui.checkbox(&mut cfg.enabled, "Discord rich presence");
    ui.label(
        RichText::new(
            "Talks to the local discord client over its IPC pipe. No network access, no tokens.",
        )
        .small()
        .weak(),
    );
    ui.add_space(8.0);

    ui.add_enabled_ui(cfg.enabled, |ui| {
        ui.horizontal(|ui| {
            ui.label("Application id");
            ui.add(
                egui::TextEdit::singleline(&mut cfg.app_id)
                    .hint_text("123456789012345678")
                    .char_limit(32)
                    .desired_width(220.0),
            );
        });
        ui.add(
            egui::Slider::new(&mut cfg.pipe_index, 0..=9)
                .text("Pipe index")
                .custom_formatter(|v, _| {
                    if v < 0.5 {
                        "auto".to_string()
                    } else {
                        format!("{}", v as u64 - 1)
                    }
                }),
        );
        ui.checkbox(&mut cfg.show_state, "Show state text");
        ui.horizontal(|ui| {
            ui.label("Idle state");
            ui.add(
                egui::TextEdit::singleline(&mut cfg.idle_state)
                    .char_limit(128)
                    .desired_width(220.0),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Large image key");
            ui.add(
                egui::TextEdit::singleline(&mut cfg.large_image)
                    .char_limit(128)
                    .desired_width(220.0),
            );
        });
    });

    ui.add_space(12.0);
    ui.separator();

    let state = by_utf8xbot_3304(cfg);
    let (color, label) = match state {
        ConnState::Connected => (
            egui::Color32::from_rgb(120, 220, 140),
            state.by_utf8xbot_2201(),
        ),
        ConnState::Connecting => (
            egui::Color32::from_rgb(230, 200, 120),
            state.by_utf8xbot_2201(),
        ),
        ConnState::Disabled => (egui::Color32::GRAY, state.by_utf8xbot_2201()),
        ConnState::Unavailable | ConnState::Failed => (
            egui::Color32::from_rgb(230, 130, 130),
            state.by_utf8xbot_2201(),
        ),
    };
    ui.horizontal(|ui| {
        ui.label("Status:");
        ui.label(RichText::new(label).color(color));
    });

    if cfg.enabled && cfg.app_id.trim().is_empty() {
        ui.label(
            RichText::new("An application id is required before a presence can be published.")
                .small()
                .color(egui::Color32::from_rgb(230, 200, 120)),
        );
    }
}
