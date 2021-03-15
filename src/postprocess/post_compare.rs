use crate::{
    array_utils::{FArray1, FArray2},
    config::{NX_SLICE, NY_SLICE},
    sim_params::SimParams,
    sim_set::SimSet,
};

use super::{axis::Axis, get_snapshots, post_fn::PostFn};
use super::{post_fn::PostFnKind, snapshot::Snapshot};
use crate::array_utils::meshgrid2;
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Clap;
use ndarray::{array, s, Array};
use ordered_float::OrderedFloat;

static EPSILON: f64 = 1e-2;
static MIN_VAL: f64 = 1e-24;

#[derive(Clap, Debug)]
pub struct CompareFn {
    pub reference: Utf8PathBuf,
}

impl PostFn for &CompareFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::NoPlotSet
    }

    fn name(&self) -> &'static str {
        "compare"
    }

    fn qualified_name(&self) -> String {
        format!("{}", self.name())
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<Vec<FArray2>> {
        let reference_sim_set = SimSet::from_output_folder(&self.reference)?;
        for (sim, sim_reference) in sim_set.iter().zip(reference_sim_set.iter()) {
            for (key, value) in sim.iter() {
                assert_eq!(sim[key], sim_reference[key]);
                if (key == "SX_SWEEP") {
                    dbg!(&sim[key], &sim_reference[key]);
                    assert!(false);
                }
            }
            for (snap, snap_reference) in get_snapshots(sim)?.zip(get_snapshots(sim_reference)?) {
                let snap = snap?;
                let snap_reference = snap_reference?;
                CompareFn::compare_snaps(&snap, &snap_reference)?;
            }
        }
        Ok(vec![])
    }
}

impl CompareFn {
    fn compare_snaps(snap1: &Snapshot, snap2: &Snapshot) -> Result<()> {
        assert!(CompareFn::is_close(
            snap1.coordinates()?,
            snap2.coordinates()?
        ));
        assert!(CompareFn::is_close(
            snap1.h_plus_abundance()?,
            snap2.h_plus_abundance()?
        ));
        Ok(())
    }

    fn is_close<D>(arr1: Array<f64, D>, arr2: Array<f64, D>) -> bool
    where
        D: ndarray::Dimension,
    {
        arr1.indexed_iter().zip(arr2.indexed_iter()).all(
            |((indices1, value1), (indices2, value2))| {
                dbg!(&indices1, &indices2, &value1, &value2);
                indices1 == indices2 && CompareFn::same_within_epsilon(*value1, *value2)
            },
        )
    }

    pub fn same_within_epsilon(val1: f64, val2: f64) -> bool {
        dbg!((val1 - val2) / (val1.abs() + val2.abs() + MIN_VAL)) < EPSILON
    }
}
