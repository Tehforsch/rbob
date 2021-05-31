use super::{
    axis::Axis,
    field_identifier::FieldIdentifier,
    get_snapshots,
    post_fn::{PostFn, PostResult},
    post_slice::get_slice_result,
};
use super::{post_fn::PostFnKind, snapshot::Snapshot};
use crate::{sim_params::SimParams, sim_set::SimSet, unit_utils::nice_time};
use anyhow::{anyhow, Result};
use clap::Clap;
use uom::si::f64::Time;
use uom::si::time::year;

#[derive(Clap, Debug)]
pub struct ShadowingFn {}

impl PostFn for &ShadowingFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "shadowing"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        let mut results = vec![];
        let kiloyear = Time::new::<year>(1e3);
        let times = [
            3.2 * kiloyear,
            6.5 * kiloyear,
            32.0 * kiloyear,
            64.0 * kiloyear,
        ];
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
        if snap1.time < time && snap2.time > time {
            let snap_index = if (time - snap1.time) < (snap2.time - time) {
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
                result.push(snaps.remove(*index));
            }
            return Ok(result);
        }
    }
    Err(anyhow!(
        "No snaps found for the desired time: {}",
        nice_time(times[current_time_index])
    ))
}
