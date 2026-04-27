use eframe::egui::{self, FontId, RichText, Visuals};

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "JetBrainsMono".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/JetBrainsMono-Regular.ttf")),
    );

    fonts.font_data.insert(
        "JetBrainsMonoBold".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/JetBrainsMono-Bold.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "JetBrainsMono".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "JetBrainsMono".to_owned());

    ctx.set_fonts(fonts);
}

pub fn apply_dark_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    visuals.window_fill = egui::Color32::from_rgb(18, 18, 18);
    visuals.panel_fill = egui::Color32::from_rgb(24, 24, 24);
    visuals.faint_bg_color = egui::Color32::from_rgb(30, 30, 30);

    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(35, 35, 35);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 40, 40);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 50, 50);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 60, 60);

    visuals.selection.bg_fill = egui::Color32::from_rgb(70, 130, 180);
    visuals.selection.stroke.color = egui::Color32::from_rgb(100, 150, 200);

    visuals.hyperlink_color = egui::Color32::from_rgb(100, 150, 250);

    visuals.window_shadow.color = egui::Color32::from_black_alpha(100);
    visuals.popup_shadow.color = egui::Color32::from_black_alpha(100);

    ctx.set_visuals(visuals);
}

pub fn heading(text: &str) -> RichText {
    RichText::new(text)
        .size(24.0)
        .color(egui::Color32::WHITE)
        .strong()
}

pub fn subheading(text: &str) -> RichText {
    RichText::new(text)
        .size(18.0)
        .color(egui::Color32::from_rgb(200, 200, 200))
}

pub fn body(text: &str) -> RichText {
    RichText::new(text)
        .size(14.0)
        .color(egui::Color32::WHITE)
}

pub fn monospace(text: &str) -> RichText {
    RichText::new(text)
        .size(13.0)
        .color(egui::Color32::from_rgb(180, 220, 180))
        .monospace()
}

pub fn error_text(text: &str) -> RichText {
    RichText::new(text)
        .size(14.0)
        .color(egui::Color32::from_rgb(255, 100, 100))
}

pub fn success_text(text: &str) -> RichText {
    RichText::new(text)
        .size(14.0)
        .color(egui::Color32::from_rgb(100, 255, 100))
}

pub fn button_primary() -> egui::Style {
    let mut style = egui::Style::default();
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(70, 130, 180);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(90, 150, 200);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 120, 170);
    style
}
