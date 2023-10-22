use eframe::{
    egui::{
        global_dark_light_mode_switch, menu::bar, CentralPanel, FontData, FontDefinitions,
        ScrollArea, SidePanel, TextStyle, TopBottomPanel, Widget,
    },
    epaint::{FontFamily, FontId, Vec2},
    run_native, App, NativeOptions,
};
use tracing::instrument::WithSubscriber;

#[derive(Default)]
pub struct HackerNewsEgui {
    open: bool,
}

impl HackerNewsEgui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

    // dont mess with the styles for now
    // style.text_styles = [
    //     (
    //         TextStyle::Heading,
    //         FontId::new(35.0, FontFamily::Proportional),
    //     ),
    //     (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional)),
    //     (
    //         TextStyle::Monospace,
    //         FontId::new(12.0, FontFamily::Monospace),
    //     ),
    //     (
    //         TextStyle::Button,
    //         FontId::new(10., FontFamily::Proportional),
    //     ),
    // ]
    // .into();

    cc.egui_ctx.set_style(style);
    cc.egui_ctx.set_fonts(font_def);
}

impl App for HackerNewsEgui {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // The order in which I create these panels matters!
        // little test based off the egui.rs demo
        // Menu bar
        TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;

                // add bar contents here
                global_dark_light_mode_switch(ui);

                // ui.separator();

                ui.toggle_value(&mut self.open, "💻 Backend");
            })
        });

        // Left side panel
        let mut is_open = self.open || ctx.memory(|mem| mem.everything_is_visible());

        SidePanel::left("backend_panel")
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.vertical_centered(|ui| {
                    // Weird that I need to add this extra spacing so that the
                    // separator bar aligns with the one in the central panel
                    // to the right
                    ui.add_space(6.);
                    ui.heading("💻 Backend");
                });

                ui.separator();
                // add the actual contents here
            });

        // Right Panel
        SidePanel::right("right_panel")
            .resizable(true)
            // .default_width(350.)
            // .width_range(80.0..=200.)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(6.);
                    ui.heading("Right Panel");
                });
                ui.separator();
            });

        // Section for list of articles
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("💻 Hacker News");
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        let iter = (0..20).map(|a| StoryCardData {
                            title: format!("HN Story #: {a}"),
                            author: format!("Author: {a}"),
                            desc: format!("This is a short description..."),
                            url: format!("https://helloworld.org/v{a}"),
                        });

                        let stories = Vec::from_iter(iter);

                        for i in stories {
                            ui.heading(i.title);
                            ui.label(i.author);
                            ui.label(i.desc);
                            ui.monospace(i.url);
                            ui.separator();
                        }
                    })
                });
            });
        });
    }
}

#[derive(Debug)]
struct StoryCardData {
    title: String,
    author: String,
    desc: String,
    url: String,
}
