use super::{
    axis::Axis,
    field_identifier::FieldIdentifier,
    plot_params::PlotParams,
    post_fn::{PostFn, PostResult},
};
use super::{post_fn::PostFnKind, snapshot::Snapshot};
use crate::{
    array_utils::{convert_heatmap_to_gnuplot_format, get_slice_grid, FArray2},
    config::{NX_SLICE, NY_SLICE},
    sim_params::SimParams,
    sim_set::SimSet,
};
use anyhow::Result;
use clap::Clap;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use ndarray_stats::QuantileExt;
use uom::si::f64::Length;
use uom::si::length::parsec;

#[derive(Clap, Debug)]
pub struct SliceFn {
    pub field: FieldIdentifier,
    pub axis: Axis,
}

impl PostFn for &SliceFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Snap
    }

    fn name(&self) -> &'static str {
        "slice"
    }

    fn qualified_name(&self) -> String {
        format!("{}_{}_{}", self.name(), self.axis, self.field)
    }

    fn post(
        &self,
        _sim_set: &SimSet,
        _sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        get_slice_result(snap.unwrap(), &self.axis, &self.field)
    }
}

pub fn get_slice_result(
    snap: &Snapshot,
    axis: &Axis,
    field: &FieldIdentifier,
) -> Result<PostResult> {
    let coords = snap.coordinates()?;
    let data = match field {
        FieldIdentifier::HpAbundance => snap.h_plus_abundance()?,
        FieldIdentifier::Density => snap.density()?,
    };
    let min_extent = snap.min_extent();
    let max_extent = snap.max_extent();
    let center = snap.center();
    let mut result = FArray2::zeros((NX_SLICE, NY_SLICE));
    let grid = get_slice_grid(&axis, &center, &min_extent, &max_extent, NX_SLICE, NY_SLICE);
    let mut tree = KdTree::new(3);
    let coords_iter = coords.outer_iter().map(|x| [x[0], x[1], x[2]]);
    for (i, coord) in coords_iter.enumerate() {
        tree.add(coord, i)?;
    }
    for (i0, i1, pos) in grid {
        let (_, index) = tree
            .nearest(&[pos[0], pos[1], pos[2]], 1, &squared_euclidean)
            .unwrap()[0];
        result[[i0, i1]] = data[*index];
    }
    let mut params = PlotParams::default();
    let default_length_unit = Length::new::<parsec>(1.0);
    let length_factor = (snap.sim.units.length / default_length_unit).value;
    params.add("minX", min_extent[0] * length_factor);
    params.add("maxX", max_extent[0] * length_factor);
    params.add("minY", min_extent[1] * length_factor);
    params.add("maxY", max_extent[1] * length_factor);
    params.add("minC", *data.min().unwrap());
    params.add("maxC", *data.max().unwrap());
    Ok(PostResult::new(
        params,
        vec![convert_heatmap_to_gnuplot_format(result, center, length_factor)],
    ))
}
