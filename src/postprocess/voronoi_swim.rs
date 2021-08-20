use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use regex::Regex;

use crate::{sim_params::SimParams, util::get_shell_command_output};

use super::get_snapshots;

pub fn simulate_run_time(sim: &SimParams) -> Result<f64> {
    let snap = get_last_snapshot(sim)?;
    run_voronoi_swim(&snap)
}

fn get_last_snapshot(sim: &SimParams) -> Result<Utf8PathBuf> {
    let snapshot = get_snapshots(sim)?.last().unwrap()?;
    let grid_file = snapshot.path.with_extension("dat");
    if grid_file.is_file() {
        println!("Reusing existing grid file: {}", grid_file);
        Ok(grid_file)
    }
    else {
        Ok(snapshot.path)
    }
}

fn run_voronoi_swim(snap: &Utf8Path) -> Result<f64> {
    let out = get_shell_command_output("voronoi_swim", &[&snap], None, false);
    if !out.success {
        return Err(anyhow!("voronoiSwim failed with error: {}", &out.stderr));
    }
    get_runtime_from_voronoi_swim_output(&out.stdout)
}

fn get_runtime_from_voronoi_swim_output(stdout: &str) -> Result<f64> {
    let re = Regex::new("[0-9]+ ([.0-9]+) ").unwrap();
    let first_capture = re.captures_iter(stdout).next().unwrap();
    first_capture
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .context("While reading runtime from output")
}
