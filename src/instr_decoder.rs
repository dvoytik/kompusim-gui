#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct InstrDecoder {
    /// Is window open or not
    window_open: bool,
    font_size: usize,

    instr_hex: String,
}

impl Default for InstrDecoder {
    fn default() -> InstrDecoder {
        InstrDecoder {
            window_open: true,
            font_size: 0,
            instr_hex: String::with_capacity(8),
        }
    }
}

impl InstrDecoder {
    pub fn open(&mut self) {
        self.window_open = true;
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.window_open;
        egui::Window::new("Instruction decoder")
            .open(&mut open)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                self.show_window_content(ui);
            });
        self.window_open = open;
    }

    fn show_window_content(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("decode_instr_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Hexadecimal");
                ui.add(egui::TextEdit::singleline(&mut self.instr_hex).hint_text("in hex"));
                ui.end_row();
                ui.label("Binary");
                ui.label("0000 0000 0000 0000 0000 0000 0000 0000");
                ui.end_row();
                ui.label("Assembly");
                ui.label("bne x0, x0, 0x8000004");
                ui.end_row();
            });
    }
}
