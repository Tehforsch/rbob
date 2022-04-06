use std::iter;
use std::iter::once;

use bob::sim_params::SimParams;
use bob::util::get_folders;
use camino::Utf8Path;
use eframe::egui::Button;
use eframe::egui::TextStyle;
use eframe::egui::Ui;
use eframe::egui::{self};
use eframe::epi;

use self::config::SELECTED_COLOR;
use self::gui_sim_set::GuiSimSet;

mod config;
mod gui_sim_set;

pub struct BobGui {
    sim_sets: Vec<GuiSimSet>,
    selected: Vec<usize>,
    sims: Vec<SimParams>,
}

fn discover_sims(path: &Utf8Path) -> Vec<GuiSimSet> {
    discover_sims_iter(path).collect()
}

fn discover_sims_iter(path: &Utf8Path) -> Box<dyn Iterator<Item = GuiSimSet>> {
    let folders = get_folders(path).unwrap();
    Box::new(
        folders
            .into_iter()
            .flat_map(move |path| {
                if get_folders(&path)
                    .unwrap()
                    .iter()
                    .find(|path| path.file_name() == Some("0"))
                    .is_some()
                {
                    Box::new(iter::once(GuiSimSet { path: path.into() }))
                } else {
                    discover_sims_iter(&path)
                }
            })
            .into_iter(),
    )
}

impl BobGui {
    pub fn new(path: &Utf8Path) -> Self {
        Self {
            sim_sets: discover_sims(path),
            selected: vec![],
            sims: vec![],
        }
    }

    fn get_selected(&self) -> impl Iterator<Item = &GuiSimSet> {
        self.selected.iter().map(|index| &self.sim_sets[*index])
    }

    fn add_or_remove_from_selection(&mut self, sim_index: usize) {
        let index = self
            .selected
            .iter()
            .enumerate()
            .find(|(_, sim_i)| *sim_i == &sim_index)
            .map(|(index, _)| index);
        if let Some(index) = index {
            self.selected.remove(index);
        } else {
            self.selected.push(sim_index);
        }
    }

    fn select(&mut self, sim_index: usize) {
        self.add_or_remove_from_selection(sim_index);
        self.sims = self
            .get_selected()
            .flat_map(|sim_set| sim_set.get_sims())
            .collect();
    }

    fn show_sim_set_buttons_and_handle_selection(&mut self, ui: &mut Ui) {
        let mut selected = None;
        for (i, sim) in self.sim_sets.iter().enumerate() {
            let mut button = Button::new(sim.name()).text_style(TextStyle::Heading);
            if self.selected.contains(&i) {
                button = button.fill(SELECTED_COLOR);
            }
            let response = ui.add(button);
            if response.clicked() {
                selected = Some(i);
            }
        }
        if let Some(selected) = selected {
            self.select(selected);
        }
    }

    fn add_sim_set_selection_panel(&mut self, ctx: &egui::CtxRef) {
        egui::SidePanel::left("sim_set_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |mut ui| {
                self.show_sim_set_buttons_and_handle_selection(&mut ui);
            });
    }

    fn add_sim_selection_panel(&mut self, ctx: &egui::CtxRef) {
        egui::SidePanel::left("sim_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |ui| {
                for sim in self.sims.iter() {
                    ui.label(sim.get_name());
                }
            });
    }

    fn add_central_panel(&mut self, ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}

impl epi::App for BobGui {
    fn name(&self) -> &str {
        "Bob"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _: &epi::Frame) {
        self.add_sim_set_selection_panel(ctx);
        self.add_sim_selection_panel(ctx);
        self.add_central_panel(ctx);
    }
}
