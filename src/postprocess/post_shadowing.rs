use anyhow::anyhow;
use anyhow::Result;
use clap::Clap;
use uom::si::f64::Time;
use uom::si::time::year;

use super::data_plot_info::DataPlotInfo;
use super::get_snapshots;
use super::named::Named;
use super::post_fn::PostResult;
use super::snapshot::Snapshot;
use crate::postprocess::axis::Axis;
use crate::postprocess::field_identifier::FieldIdentifier;
use crate::postprocess::post_slice::get_slice_result;
use crate::set_function;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::unit_utils::nice_time;

#[derive(Clap, Debug)]
pub struct ShadowingFn {
    times: Vec<f64>,
}

impl Named for ShadowingFn {
    fn name(&self) -> &'static str {
        "shadowing"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }
}

impl ShadowingFn {
    set_function!(shadowing, { move |sim_set| run(&shadowing, sim_set) });
}

fn run(shadowing: &ShadowingFn, sim_set: &SimSet) -> Result<PostResult> {
    let mut results = vec![];
    let kiloyear = Time::new::<year>(1e3);
    let times = if shadowing.times.is_empty() {
        vec![6.4, 32.0, 48.0]
    } else {
        shadowing.times.clone()
    };
    let times: Vec<_> = times.into_iter().map(|time| time * kiloyear).collect();
    for sim in sim_set.iter() {
        let snaps = find_snaps_at_times(sim, &times)?;
        for snap in snaps {
            results.push(get_slice_result(
                &snap,
                &Axis::Z,
                &FieldIdentifier::HpAbundance,
            )?);
        }
    }
    Ok(PostResult::join(results))
}

fn find_snaps_at_times<'a>(sim: &'a SimParams, times: &[Time]) -> Result<Vec<Snapshot<'a>>> {
    let mut snaps: Vec<Snapshot<'a>> = get_snapshots(sim)?.collect::<Result<_>>()?;
    let num_snaps = snaps.len();
    let mut current_time_index = 0;
    let mut result_indices = vec![];
    let mut result = vec![];
    for i in 0..num_snaps - 1 {
        let snap1 = &snaps[i];
        let snap2 = &snaps[i + 1];
        let time = times[current_time_index];
        if snap1.time < time && snap2.time > time || snap1.time > time && i == 0 {
            let snap_index = if (time - snap1.time).abs() < (snap2.time - time).abs() {
                i
            } else {
                i + 1
            };
            println!(
                "Found snap: {} at time {} for time {}",
                snap_index,
                nice_time(snaps[snap_index].time),
                nice_time(time)
            );
            result_indices.push(snap_index);
            current_time_index += 1;
        }
        if current_time_index == times.len() {
            // Traverse in reverse order - therefore the indices will always be valid
            for index in result_indices.iter().rev() {
                result.insert(0, snaps.remove(*index));
            }
            return Ok(result);
        }
    }
    Err(anyhow!(
        "No snaps found for the desired time: {}",
        nice_time(times[current_time_index])
    ))
}
