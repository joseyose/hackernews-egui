use eframe::{
    egui::{
        global_dark_light_mode_switch, menu::bar, CentralPanel, FontData, FontDefinitions,
        Response, ScrollArea, Sense, SidePanel, TextStyle, TopBottomPanel, Ui, Widget,
    },
    emath::{lerp, Align2},
    epaint::{pos2, vec2, Color32, FontFamily, FontId, Pos2, Rect, Stroke, Vec2},
    run_native, App, NativeOptions,
};
use tracing::instrument::WithSubscriber;

#[derive(Default)]
pub struct HackerNewsEgui {
    open: bool,
    story: String,
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

                ui.vertical(|ui| {
                    ui.label(&self.story);
                });
            });

        // Section for list of articles
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("💻 Hacker News");
                ui.separator();

                let mut on = false;

                // toggle_ui(ui, &mut on);

                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        let iter = (0..20).map(|a| StoryCardData {
                            title: format!("HN Story #: {a}"),
                            author: format!("Author: {a}"),
                            desc: format!("This is a short description..."),
                            url: format!("https://helloworld.org/v{a}"),
                        });

                        let stories = Vec::from_iter(iter);

                        for (index, i) in stories.iter().enumerate() {
                            // ui.heading(i.title);
                            // ui.label(i.author);
                            // ui.label(i.desc);
                            // ui.monospace(i.url);
                            // if ui.button("Click me!").clicked() {
                            //     println!("Hello world!");
                            //     self.story = String::from("Here's my little story man!");
                            // };
                            if toggle_ui(ui, &mut on).clicked() {
                                self.story = format!("This custom thing works {index}");
                                on = false;
                            };
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

// Here's a test of a custom button/painting
fn toggle_ui(ui: &mut Ui, on: &mut bool) -> Response {
    // Widget code can be broken up in four steps:
    // 1. Decide a size for the widget
    // 2. Allocate space for it
    // 3. Handle interactions with the widget (if any)
    // 4. Paint the widget

    // 1. Deciding the widget size:
    // You can query the "ui" to figure how much space is available
    // but in this example we have a fixed size widget based on the height of a standard button:
    // let desired_size = ui.available_width() * vec2(1.0, 1.0);
    let desired_size = vec2(ui.available_width(), 85.0);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    // 3. Interact:
    // Time to check for clicks in the allocated region.
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    // attach some metadata for screen readers
    // TODO: implement this

    // 4. Paint!
    // Make sure we need to paint
    if ui.is_rect_visible(rect) {
        // lets ask for a simple animation from egui
        // egui keeps track of changes in the boolean associated with the id and
        // returns an animated value in the 0-1 range for how much "on" we are.
        let how_on = ui.ctx().animate_bool(response.id, *on);

        // We will follow the current style by asking:
        // "How should something that is being interacted with be painted?"
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().interact_selectable(&response, *on);

        // All coordinates are in absolute screen coordinates so we use 'rect'
        // to place the elements
        // let rect = rect.expand(visuals.expansion);
        let radius = 0.1; //* rect.height();

        // ui.painter()
        //     .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);

        let pos = rect.min;

        let anchor = Align2::CENTER_CENTER;
        let text_color = Color32::from_rgb(0, 0, 0);
        let text = String::from("helloooooo");
        let font_id = FontId::proportional(50.0);
        let width = ui.available_width();

        // ui.painter().text(pos, anchor, text, font_id, text_color);
        // ui.painter().debug_text(pos, anchor, color, text);

        let galley = ui.painter().layout(text, font_id, text_color, width);
        // let galley = ui.painter().layout_no_wrap(text, font_id, text_color);

        ui.painter()
            .rect(rect, 0.0, visuals.bg_fill, visuals.bg_stroke);
        ui.painter_at(rect).galley(pos, galley);

        // // Paint the circle, animating it from to right with 'how_on':
        // let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        // let center = pos2(circle_x, rect.center().y);
        //
        // ui.painter()
        //     .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    // All done! Return the interaction response so the user can what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}
