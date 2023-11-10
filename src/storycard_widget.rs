use eframe::{
    egui::{Response, Sense, TextFormat, TextStyle, Ui, WidgetText},
    emath::NumExt,
    epaint::{text::LayoutJob, vec2},
};

// Custom widget paint
pub fn toggle_ui(ui: &mut Ui, on: &mut bool) -> Response {
    // Data I will pass on to the widget --- this needs to come from the api...hardcoding for now
    let heading = "Show HN: Phind Model beats GPT-4 at coding, with GPT-3.5 speed and 16k context";
    let url = "(phind.com)";
    let subheading = "269 points by rushingcreek 2 hours ago | hide | 129 comments";

    let style = ui.style();
    let heading_style = TextStyle::Heading.resolve(style);
    let url_style = TextStyle::Button.resolve(style);

    // Layout a custom job for text
    let mut text_job = LayoutJob::default();
    text_job.append(
        heading,
        0.0,
        TextFormat {
            // font_id: FontId::new(14.0, FontFamily::Proportional),
            font_id: heading_style,
            color: style.visuals.text_color(),
            ..Default::default()
        },
    );

    text_job.append(
        url,
        2.5,
        TextFormat {
            // font_id: FontId::new(14.0, FontFamily::Monospace),
            font_id: url_style,
            color: style.visuals.weak_text_color(),
            ..Default::default()
        },
    );

    // Decide size of widget
    let wrap_width = ui.available_width(); //- total_extra.x;
    let button_padding = ui.spacing().button_padding;
    let total_extra = button_padding + button_padding + vec2(0.0, 20.0);

    let heading = WidgetText::from(text_job);
    let heading_galley = heading.into_galley(ui, None, wrap_width, TextStyle::Heading);

    let subtitle = WidgetText::from(subheading);
    let text_subtitle = subtitle.into_galley(ui, None, wrap_width, TextStyle::Button);

    let value_y = heading_galley.size().y + text_subtitle.size().y + total_extra.y;

    let mut desired_size = vec2(ui.available_width(), value_y);
    desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);

    let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

    // TODO: comeback and add the accessibilty stuff

    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().interact_selectable(&response, *on);

        let rect_expanded =
            if *on || response.hovered() || response.highlighted() || response.has_focus() {
                let expanded_rect = response.rect.expand(visuals.expansion); // Use 'response.rect' directly to
                ui.painter().rect(
                    expanded_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
                expanded_rect // We're now working with an expanded rect
            } else {
                response.rect // No expansion needed, use the original rect
            };

        // Calculate title and subtitle positions once instead of computing them separately.
        // This avoids duplicate computations and improves readability.
        let padding = rect_expanded.shrink2(button_padding);

        // make title and subtitle sit one after the other vertically and left aligned
        let title_pos = rect_expanded.min + button_padding + vec2(0.0, 5.0);
        let subtitle_pos = title_pos + vec2(0.0, heading_galley.size().y + 5.0);

        heading_galley.paint_with_visuals(ui.painter(), title_pos, &visuals);
        text_subtitle.paint_with_visuals(ui.painter(), subtitle_pos, &visuals);
    }
    // All done! Return the interaction response so the user can what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}
