use std::error::Error;

use eframe::{egui::CentralPanel, epaint::Vec2, run_native, App, NativeOptions};
use hackernewsapi::HackerNewsAPI;

const RESOLUTION: (f32, f32) = (540., 500.);

#[derive(Default)]
struct HackerNewsEgui {}

impl HackerNewsEgui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
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
