use anyhow::Result;
use clap::Clap;
use uom::si::f64::Length;
use uom::si::f64::Time;
use uom::si::length::parsec;
use uom::si::time::year;

use super::calculations::get_recombination_time;
use super::calculations::get_stroemgren_radius;
use super::get_snapshots;
use super::get_source_file;
use super::plot_params::PlotParams;
use super::post_fn::PostFn;
use super::post_fn::PostFnKind;
use super::post_fn::PostResult;
use super::snapshot::Snapshot;
use crate::array_utils::FArray1;
use crate::array_utils::FArray2;
use crate::config;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

#[derive(Clap, Debug)]
pub struct RTypeExpansionFn {}

impl PostFn for &RTypeExpansionFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "rtype"
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
        get_expansion_data(sim_set)
    }
}

#[derive(Clap, Debug)]
pub struct DTypeExpansionFn {}

impl PostFn for &DTypeExpansionFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "dtype"
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
        get_expansion_data(sim_set).map(|mut result| {
            result.params.add("startTimeAnalytical", 1.0);
            result
        })
    }
}

fn get_expansion_data(sim_set: &SimSet) -> Result<PostResult> {
    let first_sim = sim_set.iter().next().expect("No sim found.");
    let num_snaps = get_snapshots(first_sim)?.count();
    let mut result = vec![];
    let mut max_t = 0.0;
    let megayear = Time::new::<year>(1e6);
    let kpc = Length::new::<parsec>(1e3);
    let first_snap = get_snapshots(first_sim)?.next().unwrap()?;
    let recombination_time = get_recombination_time(&first_snap)?;
    let photon_rate = get_source_file(first_sim)?.get_rate(0, config::H_IONIZATION_RATE_INDEX);
    let stroemgren_radius = get_stroemgren_radius(&first_snap, photon_rate)?;
    println!(
        "Stroemgren radius: {:?} kpc, Recombination time: {:?} Myr",
        stroemgren_radius / kpc,
        recombination_time / megayear
    );
    for sim in sim_set.iter() {
        let mut data = FArray2::zeros((num_snaps, 2));
        for (j, snap) in get_snapshots(sim)?.enumerate() {
            let snap = snap?;
            let time = (snap.time / recombination_time).value;
            data[[j, 0]] = time;
            data[[j, 1]] = (get_radius(&snap)? / stroemgren_radius).value;
            if time > max_t {
                max_t = time;
            }
        }
        result.push(data);
    }
    let mut params = PlotParams::default();
    params.add("minX", 0.0);
    params.add("maxX", max_t);
    params.add("minY", 0.0);
    params.add("maxY", 1.0);
    params.add("maxY", result[0][[num_snaps - 1, 1]].max(1.0));
    params.add("stroemgrenRadius", (stroemgren_radius / kpc).value);
    params.add("recombinationTime", (recombination_time / megayear).value);
    Ok(PostResult::new(params, result))
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
    Ok(bisect(
        |radius| {
            1.0 - get_mean_abundance_at_radius(&coords, &h_plus_abundance, &center, radius).unwrap()
        },
        0.5,
        0.00001,
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
    if (y - y_target).abs() < treshold || depth > max_depth {
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
        if (distance_to_center(&coord, center) - radius).abs() < thickness / 2.0 {
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
