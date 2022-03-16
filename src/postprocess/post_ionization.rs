use anyhow::Result;
use clap::Clap;

use super::named::Named;
use super::post_fn::PostResult;
use crate::array_utils::FArray2;
use crate::postprocess::data_plot_info::DataPlotInfo;
use crate::postprocess::plot_params::PlotParams;
use crate::set_function;
use crate::sim_set::SimSet;

#[derive(Clap, Debug, Clone)]
pub struct IonizationFn;

impl IonizationFn {
    set_function!(shadowing, {
        move |sim_set| get_average_ionization_over_time(sim_set)
    });
}

impl Named for IonizationFn {
    fn name(&self) -> &'static str {
        "ionization"
    }

    fn qualified_name(&self) -> String {
        format!("{}", self.name())
    }
}

fn get_average_ionization_over_time(sim_set: &SimSet) -> Result<PostResult> {
    let times_and_ionizations: Vec<(f64, f64, f64)> = sim_set
        .iter()
        .flat_map(|sim| {
            let simplex_file = sim.get_simplex_file();
            simplex_file.get_average_ionization_over_time().unwrap_or(vec![])
        })
        .collect();
    let num_points = times_and_ionizations.len();
    let mut data = FArray2::zeros((num_points, 3));
    for (i, (time, volume_ionization, mass_ionization)) in times_and_ionizations.iter().enumerate()
    {
        data[[i, 0]] = *time;
        data[[i, 1]] = *volume_ionization;
        data[[i, 2]] = *mass_ionization;
    }
    Ok(PostResult::new(PlotParams::default(), vec![data]))
}
