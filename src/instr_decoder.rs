use kompusim::rv64i_disasm::disasm;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct InstrDecoder {
    /// Is window open or not
    window_open: bool,
    font_size: usize,

    instr_hex: String,
    #[serde(skip)]
    instr_disasm: String,
}

impl Default for InstrDecoder {
    fn default() -> InstrDecoder {
        InstrDecoder {
            window_open: true,
            font_size: 0,
            instr_hex: String::with_capacity(8),
            instr_disasm: String::with_capacity(32),
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
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.instr_hex).hint_text("instruction in hex"),
                );
                if response.changed() {
                    self.instr_disasm = disasm(hex_to_u64(&self.instr_hex), 0x0);
                }
                ui.end_row();
                ui.label("Binary");
                ui.label("0000 0000 0000 0000 0000 0000 0000 0000");
                ui.end_row();
                ui.label("Assembly");
                ui.label(&self.instr_disasm);
                ui.end_row();
            });
    }
}

/// Convert hex str (e.g, "0x9393") to u32
fn hex_to_u64(hex_str: &str) -> u32 {
    u32::from_str_radix(hex_str.trim_start_matches("0x"), 16).unwrap_or_default()
}
