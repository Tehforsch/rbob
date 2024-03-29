use anyhow::Result;
use clap::Clap;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use ndarray::s;
use ndarray_stats::QuantileExt;
use uom::si::f64::Length;
use uom::si::length::parsec;

use super::axis::Axis;
use super::data_plot_info::DataPlotInfo;
use super::field_identifier::FieldIdentifier;
use super::named::Named;
use super::plot_params::PlotParams;
use super::post_fn::PostResult;
use super::snapshot::Snapshot;
use crate::array_utils::convert_heatmap_to_gnuplot_format;
use crate::array_utils::get_slice_grid;
use crate::array_utils::FArray2;
use crate::config::NX_SLICE;
use crate::config::NY_SLICE;
use crate::sim_set::SimSet;
use crate::snap_function;

#[derive(Clap, Debug, Clone)]
pub struct SliceFn {
    pub field: FieldIdentifier,
    pub axis: Axis,
}

impl SliceFn {
    snap_function!(slice_fn, {
        move |slice: SliceFn, snap| get_slice_result(&snap, &slice.axis, &slice.field)
    });
}

impl Named for SliceFn {
    fn name(&self) -> &'static str {
        "slice"
    }

    fn qualified_name(&self) -> String {
        format!("{}_{}_{}", self.name(), self.axis, self.field)
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
        FieldIdentifier::PhotonRates => {
            let data = snap.photon_rates()?;
            let index_136_freq = 2;
            data.slice(s![.., index_136_freq]).to_owned()
        }
        FieldIdentifier::PhotonFlux => {
            let data = snap.photon_flux()?;
            let densities = snap.density()?;
            let masses = snap.masses()?;
            let volumes = masses / densities;
            let index_136_freq = 2;
            let fluxes = data.slice(s![.., index_136_freq]).to_owned();
            (fluxes / volumes).to_owned()
        }
    };
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_millis(1000));
    let min_extent = snap.min_extent();
    let max_extent = snap.max_extent();
    let center = snap.center();
    let mut result = FArray2::zeros((NX_SLICE, NY_SLICE));
    let grid = get_slice_grid(axis, &center, &min_extent, &max_extent, NX_SLICE, NY_SLICE);
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
    params.add("minX", (min_extent[0] - center[0]) * length_factor);
    params.add("maxX", (max_extent[0] - center[0]) * length_factor);
    params.add("minY", (min_extent[1] - center[1]) * length_factor);
    params.add("maxY", (max_extent[1] - center[1]) * length_factor);
    params.add("minC", *data.min().unwrap());
    params.add("maxC", *data.max().unwrap());
    Ok(PostResult::new(
        params,
        vec![convert_heatmap_to_gnuplot_format(result, length_factor)],
    ))
}
