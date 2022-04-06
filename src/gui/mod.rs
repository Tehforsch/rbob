use std::iter;

use bob::postprocess::get_snapshots;
use bob::postprocess::snapshot::Snapshot;
use bob::sim_params::SimParams;
use bob::util::get_folders;
use camino::Utf8Path;
use eframe::egui::Button;
use eframe::egui::RichText;
use eframe::egui::TextStyle;
use eframe::egui::Ui;
use eframe::egui::{self};
use eframe::epi;

use self::config::SELECTED_COLOR;
use self::gui_sim_set::GuiSimSet;
use self::named::Named;
use self::selection::Selection;

mod config;
mod gui_sim_set;
mod named;
mod selection;

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

fn show_buttons_and_handle_selection<T: Named>(
    ui: &mut Ui,
    selection: &mut Selection<T>,
) -> Option<usize> {
    let mut selected = None;
    for (i, sim_set) in selection.iter().enumerate() {
        let mut button = Button::new(RichText::new(sim_set.name()).text_style(TextStyle::Heading));
        if selection.contains(i) {
            button = button.fill(SELECTED_COLOR);
        }
        let response = ui.add(button);
        if response.clicked() {
            selected = Some(i);
        }
    }
    if let Some(selected) = selected {
        selection.add_or_remove_from_selection(selected);
    }
    selected
}

pub struct BobGui {
    sim_sets: Selection<GuiSimSet>,
    sims: Selection<SimParams>,
    snaps: Selection<Snapshot>,
}

impl BobGui {
    pub fn new(path: &Utf8Path) -> Self {
        Self {
            sim_sets: Selection::new(discover_sims(path)),
            sims: Selection::new(vec![]),
            snaps: Selection::new(vec![]),
        }
    }

    fn update_sims_from_sim_set_selection(&mut self) {
        self.sims = self
            .sim_sets
            .get_selected()
            .flat_map(|sim_set| sim_set.get_sims())
            .collect();
    }

    fn update_snaps_from_sim_selection(&mut self) {
        self.snaps = self
            .sims
            .get_selected()
            .flat_map(|sim| {
                let snaps = match get_snapshots(sim) {
                    Ok(snaps) => snaps.map(|snap| snap.unwrap()).collect(),
                    Err(_) => vec![],
                };
                snaps.into_iter()
            })
            .collect();
    }

    fn add_sim_set_selection_panel(&mut self, ctx: &egui::CtxRef) {
        egui::SidePanel::left("sim_set_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |mut ui| {
                let selected_sim_set_index =
                    show_buttons_and_handle_selection(&mut ui, &mut self.sim_sets);
                if selected_sim_set_index.is_some() {
                    self.update_sims_from_sim_set_selection();
                }
            });
    }

    fn add_sim_selection_panel(&mut self, ctx: &egui::CtxRef) {
        if self.sim_sets.num_selected() != 1 {
            return;
        }
        egui::SidePanel::left("sim_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |mut ui| {
                let selected_sim_index = show_buttons_and_handle_selection(&mut ui, &mut self.sims);
                if selected_sim_index.is_some() {
                    self.update_snaps_from_sim_selection();
                }
            });
    }

    fn add_snap_selection_panel(&mut self, ctx: &egui::CtxRef) {
        if self.sims.num_selected() != 1 {
            return;
        }
        egui::SidePanel::left("snap_bar")
            .resizable(false)
            .min_width(config::MIN_SIDE_BAR_WIDTH)
            .show(ctx, |mut ui| {
                let selected_snap_index =
                    show_buttons_and_handle_selection(&mut ui, &mut self.snaps);
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
        self.add_snap_selection_panel(ctx);
        self.add_central_panel(ctx);
    }
}
