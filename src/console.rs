#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Console {
    /// Is window open or not
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
                egui::ScrollArea::vertical().show();
            });
        self.open = open;
    }
}
