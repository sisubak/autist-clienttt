use egui::RichText;
use game_config::config::ConfigBestClientUi;

pub fn by_utf8xbot_3201(ui: &mut egui::Ui, cfg: &mut ConfigBestClientUi) {
    ui.checkbox(&mut cfg.better_ui, "Better UI");
    ui.label(
        RichText::new(
            "Applies the accent color, widget rounding and animation speed to every UI page.",
        )
        .small()
        .weak(),
    );
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Accent color");
        let mut rgb = [cfg.accent.r, cfg.accent.g, cfg.accent.b];
        if ui.color_edit_button_srgb(&mut rgb).changed() {
            cfg.accent.r = rgb[0];
            cfg.accent.g = rgb[1];
            cfg.accent.b = rgb[2];
        }
        ui.label(
            RichText::new(format!(
                "#{:02X}{:02X}{:02X}",
                cfg.accent.r, cfg.accent.g, cfg.accent.b
            ))
            .monospace()
            .weak(),
        );
    });

    ui.add(
        egui::Slider::new(&mut cfg.corner_rounding, 0.0..=20.0)
            .text("Corner rounding")
            .suffix(" px"),
    );
    ui.add(
        egui::Slider::new(&mut cfg.animation_time, 0.0..=1.0)
            .text("Animation time")
            .suffix(" s"),
    );
    ui.add(
        egui::Slider::new(&mut cfg.font_scale, 0.5..=2.0)
            .text("Font scale")
            .suffix("x"),
    );

    ui.add_space(8.0);
    ui.checkbox(&mut cfg.better_scrollbar, "Better scrollbar");
    ui.add_enabled(
        cfg.better_scrollbar,
        egui::Slider::new(&mut cfg.scrollbar_width, 2.0..=32.0)
            .text("Scrollbar width")
            .suffix(" px"),
    );

    ui.add_space(12.0);
    ui.separator();
    ui.label(RichText::new("Preview").strong());
    ui.horizontal(|ui| {
        let _ = ui.button("Button");
        let mut checked = true;
        let _ = ui.checkbox(&mut checked, "Checkbox");
        let _ = ui.add(egui::Slider::new(&mut 0.5f32, 0.0..=1.0));
    });
}
