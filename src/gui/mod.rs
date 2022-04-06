use bob::util::get_folders;
use camino::Utf8Path;
use eframe::egui::{self};
use eframe::epi;

use self::gui_sim_set::GuiSimSet;

mod config;
mod gui_sim_set;

pub struct BobGui {
    sim_sets: Vec<GuiSimSet>,
}

fn discover_sims(path: &Utf8Path) -> Vec<GuiSimSet> {
    get_folders(path)
        .unwrap()
        .into_iter()
        .map(|path| GuiSimSet { path })
        .collect()
}

impl BobGui {
    pub fn new(path: &Utf8Path) -> Self {
        Self {
            sim_sets: discover_sims(path),
        }
    }

    fn add_side_panel(&mut self, ctx: &egui::CtxRef) {
        egui::SidePanel::left("side_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |ui| {
                for sim in self.sim_sets.iter() {
                    ui.label(sim.name());
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
