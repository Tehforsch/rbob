use super::{plot::PlotInfo, SnapPostFn};
use crate::{
    array_utils::{FArray1, FArray2},
    config::{NX_SLICE, NY_SLICE},
    sim_params::SimParams,
};

use super::axis::Axis;
use super::snapshot::Snapshot;
use crate::array_utils::meshgrid2;
use anyhow::Result;
use clap::Clap;
use ndarray::{array, s};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SliceResult {
    data: FArray2,
}

#[derive(Clap, Debug)]
pub struct SliceFn {
    pub axis: Axis,
}

impl SnapPostFn for &SliceFn {
    type Output = SliceResult;

    fn post(&self, sim: &SimParams, snap: &Snapshot) -> Result<Vec<Self::Output>> {
        let coords = snap.coordinates()?;
        // let dens = snap.density()?;
        let h_plus_abundance = snap.h_plus_abundance()?;
        let min_extent = snap.min_extent();
        let max_extent = snap.max_extent();
        let center = snap.center();
        let mut data = FArray2::zeros((NX_SLICE, NY_SLICE));
        for (i0, i1, pos) in self.get_slice_grid(center, min_extent, max_extent, NX_SLICE, NY_SLICE)
        {
            let (index, _) = coords
                .outer_iter()
                .enumerate()
                .min_by_key(|(_, coord)| {
                    let dist = coord - &pos;
                    OrderedFloat(dist.dot(&dist))
                })
                .unwrap();
            data[[i0, i1]] = h_plus_abundance[index];
        }
        Ok(vec![SliceResult { data }])
    }

    fn plot(&self, result: &Vec<Self::Output>, plot_info: &PlotInfo) -> Result<()> {
        Ok(())
    }
}

impl SliceFn {
    fn get_slice_grid(
        &self,
        center: FArray1,
        min_extent: FArray1,
        max_extent: FArray1,
        n0: usize,
        n1: usize,
    ) -> Box<dyn Iterator<Item = (usize, usize, FArray1)>> {
        let (orth1, orth2) = self.axis.get_orthogonal_vectors();
        let min_extent_2d = array![orth1.dot(&min_extent), orth2.dot(&min_extent)];
        let max_extent_2d = array![orth1.dot(&max_extent), orth2.dot(&max_extent)];
        let grid = meshgrid2(&min_extent_2d, &max_extent_2d, n0, n1);
        let axis = self.axis.get_axis_vector();
        let center_along_axis = (center.dot(&axis)) * axis;
        Box::new(Self::get_usize_grid(n0, n1).map(move |(i0, i1)| {
            let pos2d = grid.slice(s![i0, i1, ..]);
            (
                i0,
                i1,
                pos2d[0] * &orth1 + pos2d[1] * &orth2 + &center_along_axis,
            )
        }))
    }

    fn get_usize_grid(n0: usize, n1: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
        Box::new(
            (0..n0)
                .into_iter()
                .flat_map(move |i0| (0..n1).into_iter().map(move |i1| (i0, i1))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_slice_grid() -> Result<()> {
        let center = array![2., 2., 2.];
        let min_extent = array![1., 1., 1.];
        let max_extent = array![3., 3., 3.];
        let slice_fn = SliceFn { axis: Axis::X };
        let grid: Vec<(usize, usize, FArray1)> = slice_fn
            .get_slice_grid(center, min_extent, max_extent, 3, 3)
            .collect();
        assert_eq!(
            grid,
            vec![
                (0, 0, array![2., 1., 1.]),
                (0, 1, array![2., 1., 2.]),
                (0, 2, array![2., 1., 3.]),
                (1, 0, array![2., 2., 1.]),
                (1, 1, array![2., 2., 2.]),
                (1, 2, array![2., 2., 3.]),
                (2, 0, array![2., 3., 1.]),
                (2, 1, array![2., 3., 2.]),
                (2, 2, array![2., 3., 3.]),
            ]
        );
        Ok(())
    }
}