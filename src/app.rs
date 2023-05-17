use eframe;

use crate::{decode_instr::DecodeInstr, instr_list::InstrList};

/// Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KompusimApp {
    label: String,

    /// -+ to the default font size
    font_delta: f32,
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
            font_delta: 0.0,
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
        if let Some(storage) = cc.storage {
            let app: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            println!("font_delta: {}", app.font_delta);
            set_all_fonts_size(&cc.egui_ctx, app.font_delta);
            return app;
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
            font_delta,
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
                        println!("\nDBG: inc: font_delta: {}\n", font_delta);
                        increase_all_fonts(ctx, font_delta);
                        ui.close_menu();
                    }
                    if ui.button("Decrease font").clicked() {
                        println!("\nDBG: decr: font_delta: {}\n", font_delta);
                        decrease_all_fonts(ctx, font_delta);
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

fn increase_all_fonts(ctx: &egui::Context, font_delta: &mut f32) {
    println!("\nDBG: incr: font_delta: {}\n", font_delta);
    if *font_delta <= 50.0 {
        *font_delta += 1.0;
        println!("\nDBG: incr: font_delta: {}\n", font_delta);
        set_all_fonts_size(ctx, *font_delta);
    }
}

fn decrease_all_fonts(ctx: &egui::Context, font_delta: &mut f32) {
    println!("\nDBG: decr: font_delta: {}\n", font_delta);
    if *font_delta >= 0.0 {
        *font_delta -= 1.0;
        println!("\nDBG: decr: font_delta: {}\n", font_delta);
        set_all_fonts_size(ctx, *font_delta);
    }
}

fn set_all_fonts_size(ctx: &egui::Context, font_delta: f32) {
    println!("\nDBG: set: font_delta: {}\n", font_delta);
    let mut style: egui::Style = (*ctx.style()).clone();
    for (_, v) in style.text_styles.iter_mut() {
        v.size += font_delta;
    }
    ctx.set_style(style);
}
