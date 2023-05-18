use eframe;
use egui::Modifiers;

use crate::{decode_instr::DecodeInstr, instr_list::InstrList};

/// Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KompusimApp {
    /// delta (font_delta * 0.5) for the default font size for all text styles
    font_delta: i32,
    show_settings: bool,
    instr_list: InstrList,
    decode_instr: DecodeInstr,
    // this how you opt-out of serialization of a member
    // #[serde(skip)]
}

impl Default for KompusimApp {
    fn default() -> Self {
        Self {
            show_settings: false,
            font_delta: 0,
            instr_list: InstrList::default(),
            decode_instr: DecodeInstr::default(),
        }
    }
}

impl KompusimApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            let app: KompusimApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            set_all_fonts_size(&cc.egui_ctx, app.font_delta as f32 * 0.5);
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
            show_settings,
            font_delta,
            instr_list,
            decode_instr,
        } = self;

        // The top panel is for the menu bar:
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Shortcuts
            let organize_windows_shortcut =
                egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
            if ui.input_mut(|i| i.consume_shortcut(&organize_windows_shortcut)) {
                ctx.memory_mut(|mem| mem.reset_areas());
            }

            let inc_fonts_shortcut =
                egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::PlusEquals);
            if ui.input_mut(|i| i.consume_shortcut(&inc_fonts_shortcut)) {
                increase_all_fonts(ctx, font_delta);
            }
            let dec_fonts_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::Minus);
            if ui.input_mut(|i| i.consume_shortcut(&dec_fonts_shortcut)) {
                decrease_all_fonts(ctx, font_delta);
            }

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
                    // hack to make menus oneliners
                    ui.set_min_width(*font_delta as f32 * 10.0 + 150.0);
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
                    // hack to make menus oneliners
                    ui.set_min_width(*font_delta as f32 * 10.0 + 150.0);
                    if ui
                        .add(
                            egui::Button::new("Increase font")
                                .shortcut_text(ui.ctx().format_shortcut(&inc_fonts_shortcut)),
                        )
                        .clicked()
                    {
                        increase_all_fonts(ctx, font_delta);
                        ui.close_menu();
                    }
                    if ui
                        .add(
                            egui::Button::new("Decrease font")
                                .shortcut_text(ui.ctx().format_shortcut(&dec_fonts_shortcut)),
                        )
                        .clicked()
                    {
                        decrease_all_fonts(ctx, font_delta);
                        ui.close_menu();
                    }
                    if ui
                        .add(
                            egui::Button::new("Organize windows").shortcut_text(
                                ui.ctx().format_shortcut(&organize_windows_shortcut),
                            ),
                        )
                        .clicked()
                    {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
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

fn increase_all_fonts(ctx: &egui::Context, font_delta: &mut i32) {
    if *font_delta <= 50 {
        *font_delta += 1;
        set_all_fonts_size(ctx, 0.5);
    }
}

fn decrease_all_fonts(ctx: &egui::Context, font_delta: &mut i32) {
    if *font_delta >= -5 {
        *font_delta -= 1;
        set_all_fonts_size(ctx, -0.5);
    }
}

fn set_all_fonts_size(ctx: &egui::Context, font_delta: f32) {
    let mut style: egui::Style = (*ctx.style()).clone();
    for (_, v) in style.text_styles.iter_mut() {
        v.size += font_delta;
    }
    ctx.set_style(style);
}
