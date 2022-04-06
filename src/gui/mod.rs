use camino::Utf8Path;
use camino::Utf8PathBuf;
use eframe::egui::Button;
use eframe::egui::{self};
use eframe::epi;

mod config;

pub struct BobGui {
    path: Utf8PathBuf,
}

impl BobGui {
    pub fn new(path: &Utf8Path) -> Self {
        Self { path: path.into() }
    }

    fn add_side_panel(&mut self, ctx: &egui::CtxRef) {
        egui::SidePanel::left("side_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |ui| {
                let cut_button = ui.add(Button::new("hi"));
                if cut_button.clicked() {
                    dbg!("ho");
                }
            });
    }

    fn add_central_panel(&mut self, ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("ho");
        });
    }
}

impl epi::App for BobGui {
    fn name(&self) -> &str {
        "Bob"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _: &epi::Frame) {
        self.add_side_panel(ctx);
        self.add_central_panel(ctx);
    }
}
