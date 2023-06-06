#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Console {
    /// Is the window open or not
    open: bool,
    //font_size: usize,
}

impl Console {
    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        egui::Window::new("Console")
            .open(&mut open)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut out_bytes = String::new();
                    ui.add(
                        egui::TextEdit::multiline(&mut out_bytes)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY),
                    );
                });
            });
        self.open = open;
    }
}
