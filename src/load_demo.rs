pub struct LoadDemo {
    /// Is window open or not
    window_open: bool,
}

impl Default for LoadDemo {
    fn default() -> LoadDemo {
        LoadDemo { window_open: true }
    }
}

impl LoadDemo {
    pub fn open(&mut self) {
        self.window_open = true;
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if self.window_open {
            let mut window_opened = self.window_open;
            egui::Window::new("Load demo")
                .open(&mut window_opened)
                .resizable(true)
                .default_width(400.0)
                .show(ctx, |ui| {
                    self.show_window_content(ui);
                });
            if self.window_open {
                self.window_open = window_opened;
            }
        }
    }

    fn show_window_content(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("load_demo_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Bare metal hello world with UART");
                if ui.button("Load").clicked() {
                    self.window_open = false;
                }
                ui.end_row();
                ui.label("Linux kernel (unimplemented)");
                if ui.button("Load").clicked() {
                    self.window_open = false;
                }
                ui.end_row();
            });
    }
}
