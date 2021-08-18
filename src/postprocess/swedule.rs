use anyhow::{anyhow, Context, Result};
use regex::Regex;

use crate::{sim_params::SimParams, util::get_shell_command_output};

use super::{get_snapshots, snapshot::Snapshot};

pub fn simulate_run_time(sim: &SimParams) -> Result<f64> {
    let snap = get_last_snapshot(sim)?;
    run_swedule(&snap)
}

fn get_last_snapshot(sim: &SimParams) -> Result<Snapshot<'_>> {
    get_snapshots(sim)?.last().unwrap()
}

fn run_swedule(snap: &Snapshot<'_>) -> Result<f64> {
    let out = get_shell_command_output("swedule", &[&snap.path], None, false);
    if !out.success {
        return Err(anyhow!("Swedule failed with error: {}", &out.stderr));
    }
    get_runtime_from_swedule_output(&out.stdout)
}

fn get_runtime_from_swedule_output(stdout: &str) -> Result<f64> {
    let re = Regex::new("[0-9]+ ([.0-9]+) ").unwrap();
    let first_capture = re.captures_iter(stdout).next().unwrap();
    first_capture
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .context("While reading runtime from output")
}
