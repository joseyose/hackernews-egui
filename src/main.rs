mod app;
use app::HackerNewsEgui;
mod storycard_widget;
use std::error::Error;

use hackernewsapi::HackerNewsAPI;

const RESOLUTION: (f32, f32) = (540., 500.);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting HackerNews-egui application...");

    // disable for now
    // let mut api = HackerNewsAPI::new();
    // let response = api.collect_all_stories().await?;
    // response.debug_print_stories(5).await?;

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size =
        Some(eframe::epaint::Vec2::new(RESOLUTION.0, RESOLUTION.1));

    eframe::run_native(
        "HackerNews-egui",
        native_options,
        Box::new(|cc| Box::new(HackerNewsEgui::new(cc))),
    );

    Ok(())
}
