use egui::{Color32, RichText};
use game_config::config::Config;

use serde::{Deserialize, Serialize};

pub const BESTCLIENT_SETTINGS_STORAGE_KEY: &str = "bestclient.settings";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BestClientTab {
    #[default]
    Interface,
    Discord,
}

impl BestClientTab {
    pub fn by_utf8xbot_3101(self) -> &'static str {
        match self {
            BestClientTab::Interface => "Interface",
            BestClientTab::Discord => "Discord RPC",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BestClientTabState {
    pub tab: BestClientTab,
}

fn by_utf8xbot_3102(ui: &mut egui::Ui, state: &mut BestClientTabState) {
    ui.horizontal(|ui| {
        for tab in [BestClientTab::Interface, BestClientTab::Discord] {
            let selected = state.tab == tab;
            let text = RichText::new(tab.by_utf8xbot_3101());
            if ui.selectable_label(selected, text).clicked() {
                state.tab = tab;
            }
        }
    });
    ui.add_space(4.0);
}

pub fn by_utf8xbot_3103(ui: &mut egui::Ui, config: &mut Config) {
    crate::theme::by_utf8xbot_2102(&config.game.bc.ui);

    let mut state = config.storage::<BestClientTabState>(BESTCLIENT_SETTINGS_STORAGE_KEY);

    ui.vertical(|ui| {
        ui.label(
            RichText::new("BestClient")
                .size(20.0)
                .color(Color32::from_rgb(
                    config.game.bc.ui.accent.r,
                    config.game.bc.ui.accent.g,
                    config.game.bc.ui.accent.b,
                )),
        );
        ui.add_space(2.0);
        by_utf8xbot_3102(ui, &mut state);
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| match state.tab {
            BestClientTab::Interface => {
                super::tabs::interface::by_utf8xbot_3201(ui, &mut config.game.bc.ui);
            }
            BestClientTab::Discord => {
                super::tabs::discord::by_utf8xbot_3301(ui, &mut config.game.bc.discord);
            }
        });
    });

    config.set_storage(BESTCLIENT_SETTINGS_STORAGE_KEY, &state);
}
