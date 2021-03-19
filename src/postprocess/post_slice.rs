use std::collections::HashMap;

use crate::{
    array_utils::{FArray1, FArray2},
    config::{NX_SLICE, NY_SLICE},
    sim_params::SimParams,
    sim_set::SimSet,
};
use ndarray_stats::QuantileExt;
use super::{axis::Axis, post_fn::PostFn};
use super::{post_fn::PostFnKind, snapshot::Snapshot};
use crate::array_utils::meshgrid2;
use anyhow::Result;
use clap::Clap;
use ndarray::{array, s};
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

#[derive(Clap, Debug)]
pub struct SliceFn {
    pub axis: Axis,
    #[clap(short, long)]
    pub log: bool,
}

impl PostFn for &SliceFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Snap
    }

    fn name(&self) -> &'static str {
        "slice"
    }

    fn qualified_name(&self) -> String {
        format!("{}_{}", self.name(), self.axis)
    }

    fn post(
        &self,
        _sim_set: &SimSet,
        _sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
    ) -> Result<(Vec<FArray2>, HashMap<String, String>)> {
        let snap = snap.unwrap();
        let coords = snap.coordinates()?;
        // let dens = snap.density()?;
        let h_plus_abundance = snap.h_plus_abundance()?;
        let min_extent = snap.min_extent();
        let max_extent = snap.max_extent();
        let center = snap.center();
        let mut data = FArray2::zeros((NX_SLICE, NY_SLICE));
        let grid = self.get_slice_grid(&center, &min_extent, &max_extent, NX_SLICE, NY_SLICE);
        let mut tree = KdTree::new(3);
        let coords_iter = coords.outer_iter().map(|x| [x[0], x[1], x[2]]);
        for (i, coord) in coords_iter.enumerate() {
            tree.add(coord, i)?;
        }
        for (i0, i1, pos) in grid {
            let (_, index) = tree.nearest(&[pos[0], pos[1], pos[2]], 1, &squared_euclidean).unwrap()[0];
            data[[i0, i1]] = h_plus_abundance[*index];
        }
        let mut replacements = HashMap::new();
        replacements.insert("minX".to_owned(), format!("{}", min_extent[0]));
        replacements.insert("maxX".to_owned(), format!("{}", max_extent[0]));
        replacements.insert("minY".to_owned(), format!("{}", min_extent[1]));
        replacements.insert("maxY".to_owned(), format!("{}", max_extent[1]));
        replacements.insert("logPlot".to_owned(), format!("{}", self.log as i32));
        replacements.insert("minC".to_owned(), h_plus_abundance.min().unwrap().to_string());
        Ok((vec![SliceFn::convert_heatmap_to_gnuplot_format(data)], replacements))
    }
}

impl SliceFn {
    fn convert_heatmap_to_gnuplot_format(heatmap: FArray2) -> FArray2 {
        let shape = heatmap.shape();
        let mut result = FArray2::zeros((shape[0] * shape[1], 3));
        for ((i0, i1), v) in heatmap.indexed_iter() {
            result[[i0 * shape[0] + i1, 0]] = i0 as f64 / NX_SLICE as f64;
            result[[i0 * shape[0] + i1, 1]] = i1 as f64 / NY_SLICE as f64;
            result[[i0 * shape[0] + i1, 2]] = *v as f64;
        }
        result
    }

    fn get_slice_grid(
        &self,
        center: &FArray1,
        min_extent: &FArray1,
        max_extent: &FArray1,
        n0: usize,
        n1: usize,
    ) -> Box<dyn Iterator<Item = (usize, usize, FArray1)>> {
        let (orth1, orth2) = self.axis.get_orthogonal_vectors();
        let min_extent_2d = array![orth1.dot(min_extent), orth2.dot(min_extent)];
        let max_extent_2d = array![orth1.dot(max_extent), orth2.dot(max_extent)];
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
            .get_slice_grid(&center, &min_extent, &max_extent, 3, 3)
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
