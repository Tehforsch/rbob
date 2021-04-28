use super::{
    calculations::{get_recombination_time, get_stroemgren_radius},
    get_snapshots,
    plot_params::PlotParams,
    post_fn::{PostFn, PostResult},
};
use super::{post_fn::PostFnKind, snapshot::Snapshot};
use crate::{
    array_utils::{FArray1, FArray2},
    sim_params::SimParams,
    sim_set::SimSet,
};
use anyhow::Result;
use clap::Clap;
use uom::si::f64::{Frequency, Length};
use uom::si::frequency::hertz;

#[derive(Clap, Debug)]
pub struct ExpansionFn {
    #[clap(short, long)]
    pub log: bool,
}

impl PostFn for &ExpansionFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "expansion"
    }

    fn qualified_name(&self) -> String {
        format!("{}", self.name())
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        let num_snaps = get_snapshots(sim_set.iter().next().unwrap())?.count();
        let mut result = vec![];
        let mut max_t = 0.0;
        for sim in sim_set.iter() {
            let mut data = FArray2::zeros((num_snaps, 2));
            for (j, snap) in get_snapshots(sim)?.enumerate() {
                let snap = snap?;
                let photon_rate = Frequency::new::<hertz>(1.0e49);
                let recombination_time = get_recombination_time(&snap)?;
                let stroemgren_radius = get_stroemgren_radius(&snap, photon_rate)?;
                let time = (snap.time / recombination_time).value;
                data[[j, 0]] = time;
                data[[j, 1]] = (get_radius(&snap)? / stroemgren_radius).value;
                if time > max_t {
                    max_t = time;
                }
            }
            result.push(data);
        }
        let mut params = PlotParams::new();
        params.add("minX", 0.0);
        params.add("maxX", max_t);
        params.add("minY", 0.0);
        params.add("maxY", 1.0);
        // replacements.insert("minC".to_owned(), h_plus_abundance.min().unwrap().to_string());
        // Ok((vec![SliceFn::convert_heatmap_to_gnuplot_format(data)], replacements));
        Ok(PostResult::new(params, result))
    }
}

fn get_radius(snap: &Snapshot) -> Result<Length> {
    get_radius_code_units(snap).map(|radius| radius * snap.sim.units.length)
}

fn get_radius_code_units(snap: &Snapshot) -> Result<f64> {
    let coords = snap.coordinates()?;
    let h_plus_abundance = snap.h_plus_abundance()?;
    let center = snap.center();
    let min_extent = snap.min_extent();
    let max_extent = snap.max_extent();
    let max_radius = (max_extent[0] - min_extent[0]).max(max_extent[1] - min_extent[1]);
    for i in 1..10 {
        get_mean_abundance_at_radius(&coords, &h_plus_abundance, &center, 0.1 * (i as f64));
    }
    Ok(bisect(
        |radius| {
            1.0 - get_mean_abundance_at_radius(&coords, &h_plus_abundance, &center, radius).unwrap()
        },
        0.5,
        0.001,
        0.0,
        max_radius,
    ))
}

fn bisect(f: impl Fn(f64) -> f64, y_target: f64, treshold: f64, x_min: f64, x_max: f64) -> f64 {
    bisect_to_value(f, y_target, treshold, x_min, x_max, 0, 15)
}

fn bisect_to_value(
    f: impl Fn(f64) -> f64,
    y_target: f64,
    treshold: f64,
    x_min: f64,
    x_max: f64,
    depth: usize,
    max_depth: usize,
) -> f64 {
    let x_try = (x_max + x_min) / 2.0;
    let y = f(x_try);
    if (y - y_target).abs() < treshold {
        x_try
    } else if depth > max_depth {
        x_try
    } else if y > y_target {
        bisect_to_value(f, y_target, treshold, x_min, x_try, depth + 1, max_depth)
    } else {
        bisect_to_value(f, y_target, treshold, x_try, x_max, depth + 1, max_depth)
    }
}

fn get_mean_abundance_at_radius(
    coordinates: &FArray2,
    h_plus_abundance: &FArray1,
    center: &FArray1,
    radius: f64,
) -> Option<f64> {
    let thickness = 0.05;
    let mut mean_abundance = 0.0;
    let mut num_points = 0;
    let coords_iter = coordinates.outer_iter().map(|x| [x[0], x[1], x[2]]);
    for (i, coord) in coords_iter.enumerate() {
        if (distance_to_center(&coord, center) - radius).abs() < thickness {
            mean_abundance += h_plus_abundance[i];
            num_points += 1;
        }
    }
    match num_points {
        0 => None,
        _ => Some(mean_abundance / num_points as f64),
    }
}

fn distance_to_center(coord: &[f64; 3], center: &FArray1) -> f64 {
    ((coord[0] - center[0]).powi(2)
        + (coord[1] - center[1]).powi(2)
        + (coord[2] - center[2]).powi(2))
    .sqrt()
}
