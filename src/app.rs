use eframe;
use egui::Ui;

use crate::{decode_instr::DecodeInstr, instr_list::InstrList};

/// Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KompusimApp {
    // Example stuff:
    label: String,

    show_settings: bool,
    instr_list: InstrList,
    decode_instr: DecodeInstr,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for KompusimApp {
    fn default() -> Self {
        Self {
            label: "Kompusim".to_owned(),
            show_settings: false,
            instr_list: InstrList::default(),
            decode_instr: DecodeInstr::default(),
            value: 2.7, // TODO: remove
        }
    }
}

impl KompusimApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for KompusimApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            show_settings,
            instr_list,
            decode_instr,
            value,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Settings").clicked() {
                        *show_settings = true;
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                        _frame.close();
                    }
                });
                ui.menu_button("Instructions", |ui| {
                    if ui.button("Instruction list").clicked() {
                        instr_list.open();
                        ui.close_menu();
                    }
                    if ui.button("Decode instruction").clicked() {
                        decode_instr.open();
                        ui.close_menu();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Increase font").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Decrease font").clicked() {
                        ui.close_menu();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Kompusim");
            ui.hyperlink("https://github.com/dvoytik/kompusim-gui");
            egui::warn_if_debug_build(ui);
        });

        instr_list.show(ctx);

        decode_instr.show(ctx);

        egui::Window::new("Settings")
            .open(show_settings)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| ctx.settings_ui(ui));
            });
    }
}

// fn text_styles_ui(ui: &mut Ui, text_styles: &mut BTreeMap<TextStyle, FontId>) -> Response {
//     ui.vertical(|ui| {
//         crate::Grid::new("text_styles").show(ui, |ui| {
//             for (text_style, font_id) in text_styles.iter_mut() {
//                 ui.label(RichText::new(text_style.to_string()).font(font_id.clone()));
//                 crate::introspection::font_id_ui(ui, font_id);
//                 ui.end_row();
//             }
//         });
//         crate::reset_button_with(ui, text_styles, default_text_styles());
//     })
//     .response
// }

fn increase_font(ui: &mut Ui) {
    let mut text_styles = ui.style_mut().text_styles();
    //ui.vertical(|ui| {
    //crate::Grid::new("text_styles").show(ui, |ui| {
    for (text_style, font_id) in text_styles.iter_mut() {
        //ui.label(RichText::new(text_style.to_string()).font(font_id.clone()));
        //crate::introspection::font_id_ui(ui, font_id);
        //ui.end_row();
        font_id.size += 1.0;
    }
    //});
    //crate::reset_button_with(ui, text_styles, default_text_styles());
    //})
    //.response
}

// https://docs.rs/egui/latest/egui/style/struct.Style.html
    /// let mut style = (*ctx.style()).clone();
    ///
    /// // Redefine text_styles
    /// style.text_styles = [
    ///   (Heading, FontId::new(30.0, Proportional)),
    ///   (Name("Heading2".into()), FontId::new(25.0, Proportional)),
    ///   (Name("Context".into()), FontId::new(23.0, Proportional)),
    ///   (Body, FontId::new(18.0, Proportional)),
    ///   (Monospace, FontId::new(14.0, Proportional)),
    ///   (Button, FontId::new(14.0, Proportional)),
    ///   (Small, FontId::new(10.0, Proportional)),
    /// ].into();
    ///
    /// // Mutate global style with above changes
    /// ctx.set_style(style);
