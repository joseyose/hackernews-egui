use eframe::{
    egui::{
        global_dark_light_mode_switch, menu::bar, CentralPanel, FontData, FontDefinitions,
        SidePanel, TextStyle, TopBottomPanel, Widget,
    },
    epaint::{FontFamily, FontId, Vec2},
    run_native, App, NativeOptions,
};

#[derive(Default)]
struct HackerNewsEgui {
    open: bool,
}

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
        // SidePanel::left("left_side_panel").show(ctx, |ui| {
        //     ui.label("Hello world!");
        // });
        // CentralPanel::default().show(ctx, |ui| {
        //     bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             ui.label("test");
        //         })
        //     });
        //     ui.heading("Hello World!");
        // });

        // little test based off the egui.rs demo

        TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;

                // add bar contents here
                global_dark_light_mode_switch(ui);

                ui.separator();

                // ui.menu_button("💻 Backend", |ui| {
                //     ui.set_style(ui.ctx().style()); // ignore the "menu" style set by 'menu_button'
                // })
                ui.toggle_value(&mut self.open, "💻 Backend");
            })
        });

        let mut is_open = self.open || ctx.memory(|mem| mem.everything_is_visible());

        SidePanel::left("backend_panel")
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("💻 Backend");
                });

                ui.separator();
                // add the actual contents here
            });
    }
}
