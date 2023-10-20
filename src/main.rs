use std::error::Error;

use eframe::{
    egui::{CentralPanel, FontData, FontDefinitions, TextStyle},
    epaint::{FontFamily, FontId, Vec2},
    run_native, App, NativeOptions,
};
use hackernewsapi::HackerNewsAPI;

const RESOLUTION: (f32, f32) = (540., 500.);

#[derive(Default)]
struct HackerNewsEgui {}

impl HackerNewsEgui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts_and_style(cc);
        Self::default()
    }
}

fn configure_fonts_and_style(cc: &eframe::CreationContext<'_>) {
    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        "Inter".to_owned(),
        FontData::from_static(include_bytes!("../Inter-Regular.ttf")),
    );

    // Place my font first (highest priority)
    font_def
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "Inter".to_owned());

    // Place my font as last fallback for monospace
    font_def
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("Inter".to_owned());

    let mut style = (*cc.egui_ctx.style()).clone();

    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(35.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional)),
        (
            TextStyle::Monospace,
            FontId::new(12.0, FontFamily::Monospace),
        ),
    ]
    .into();

    cc.egui_ctx.set_style(style);
    cc.egui_ctx.set_fonts(font_def);
}

impl App for HackerNewsEgui {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting HackerNews-egui application...");

    // disable for now
    // let mut api = HackerNewsAPI::new();
    // let response = api.collect_all_stories().await?;
    // response.debug_print_stories(5).await?;

    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(RESOLUTION.0, RESOLUTION.1));

    run_native(
        "HackerNews-egui",
        native_options,
        Box::new(|cc| Box::new(HackerNewsEgui::new(cc))),
    );

    Ok(())
}
