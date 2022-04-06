use bob::util::get_folders;
use camino::Utf8Path;
use eframe::egui::Button;
use eframe::egui::TextStyle;
use eframe::egui::{self};
use eframe::epi;

use self::config::SELECTED_COLOR;
use self::gui_sim_set::GuiSimSet;

mod config;
mod gui_sim_set;

pub struct BobGui {
    sim_sets: Vec<GuiSimSet>,
    selected: usize,
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
            selected: 0,
        }
    }

    fn find_index(&self, sim: &GuiSimSet) -> usize {
        self.sim_sets
            .iter()
            .enumerate()
            .find(|(i, s)| *s == sim)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn add_side_panel(&mut self, ctx: &egui::CtxRef) {
        egui::SidePanel::left("side_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |ui| {
                for (i, sim) in self.sim_sets.iter().enumerate() {
                    let mut button = Button::new(sim.name()).text_style(TextStyle::Heading);
                    if i == self.selected {
                        button = button.fill(SELECTED_COLOR);
                    }
                    let response = ui.add(button);
                    if response.clicked() {
                        self.selected = self.find_index(sim);
                    }
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
