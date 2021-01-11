// use super::SimPostFn;
// use super::{get_snapshots, plot::PlotInfo};
// use crate::sim_params::SimParams;

// use anyhow::Result;
// use uom::si::{
//     f64::{Ratio, Time},
//     ratio::ratio,
// };
// use serde::{Deserialize, Serialize};
use clap::Clap;

#[derive(Clap, Debug)]
pub struct ExpansionFn {}

// #[derive(Serialize, Deserialize)]
// pub struct ExpansionResult {
//     times: Vec<Time>,
//     mean_abundance: Vec<Ratio>,
// }

// impl SimPostFn for &ExpansionFn {
//     type Output = ExpansionResult;

//     fn post(&self, sim: &SimParams) -> Result<Vec<Self::Output>> {
//         let mut times: Vec<Time> = vec![];
//         let mut mean_abundance: Vec<Ratio> = vec![];
//         for mb_snap in get_snapshots(sim)? {
//             let snap = mb_snap?;
//             let coords = snap.coordinates()?;
//             let dens = snap.density()?;
//             let h_plus_abundance = snap.h_plus_abundance()?;
//             times.push(snap.time);
//             mean_abundance.push(Ratio::new::<ratio>(h_plus_abundance.mean().unwrap()));
//         }
//         Ok(ExpansionResult {
//             times,
//             mean_abundance,
//         })
//     }

//     fn plot(&self, result: &Vec<Self::Output>, plot_info: &PlotInfo) -> Result<()> {
//         Ok(())
//     }
// }
