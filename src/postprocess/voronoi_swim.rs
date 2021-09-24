use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use regex::Regex;

use crate::{config, job_params::JobParams, run::run_job_file, sim_params::SimParams, util::{get_shell_command_output, write_file}};

pub fn simulate_run_time(sim: &SimParams) -> Result<f64> {
    let snap = get_grid_file(sim)?;
    run_voronoi_swim(&snap)
}

fn get_grid_file(sim: &SimParams) -> Result<Utf8PathBuf> {
    let grid_file = sim.folder.join("grid.dat");
    let grid_file = if grid_file.is_file() {
        println!("Reusing existing grid file: {}", grid_file);
        grid_file
    }
    else {
        get_grid_file_from_arepo(sim)?
    };
    Ok(grid_file)
}

fn get_grid_file_from_arepo(sim: &SimParams) -> Result<Utf8PathBuf> {
    let job_file = write_grid_job_file(sim)?;
    run_job_file(
        sim,
        &job_file,
        false,
    )?;
    Ok(sim.folder.join(config::DEFAULT_GRID_FILE_NAME))
}

fn write_grid_job_file(sim: &SimParams) -> Result<Utf8PathBuf> {
    let job_file = sim.folder.join(config::DEFAULT_GRID_JOB_FILE_NAME);
    let mut job_params = sim.get_job_params()?;
    let num_cores = sim.get_num_cores()?;
    let (num_nodes, num_cores_per_node, partition) =
        JobParams::get_core_distribution(num_cores, config::SYSTEM_CONFIG);
    job_params.num_cores = num_cores;
    job_params.num_nodes = num_nodes;
    job_params.num_cores_per_node = num_cores_per_node;
    job_params.partition = partition;
    job_params.run_params = "23".to_owned();
    job_params.executable_name = "arepo_grid".to_owned();
    job_params.log_file = "grid.stdout".to_owned();
    let contents = sim.get_job_file_contents_from_job_params(&job_params)?;
    write_file(&job_file, &contents)?;
    Ok(job_file)
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
