// use std::thread;
// use std::time::Duration;

use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;

use crate::config;
use crate::job_params::JobParams;
use crate::run::run_job_file;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::write_file;

pub fn simulate_run_time(_sim: &SimParams, _voronoi_swim_param_file: &Utf8Path) -> Result<f64> {
    // let grid_file = get_grid_file(sim)?;
    // let result = voronoi_swim::run::simulate_grid(&voronoi_swim_param_file, &vec![grid_file]);
    // Ok(result?[0].time)
    todo!() // Fix this for building on supermuc with cross ...
}

fn get_grid_file_path(sim: &SimParams) -> Utf8PathBuf {
    sim.folder.join(config::DEFAULT_GRID_FILE_NAME)
}

pub fn generate_all_grid_files(sim_set: &SimSet) -> Result<()> {
    for sim in sim_set.iter() {
        let grid_file = get_grid_file_path(sim);
        if !grid_file.is_file() {
            println!("Generating grid file for sim: {}", sim.folder);
            get_grid_file_from_arepo(sim)?;
        }
    }
    Ok(())
}

// fn get_grid_file(sim: &SimParams) -> Result<Utf8PathBuf> {
//     let grid_file = get_grid_file_path(sim);
//     let grid_file = if grid_file.is_file() {
//         println!("Reusing existing grid file: {}", grid_file);
//         grid_file
//     } else {
//         wait_for_grid_file(sim)
//     };
//     Ok(grid_file)
// }

fn get_grid_file_from_arepo(sim: &SimParams) -> Result<Utf8PathBuf> {
    let grid_file = get_grid_file_path(sim);
    let job_file = write_grid_job_file(sim)?;
    run_job_file(sim, &job_file, false, None)?;
    Ok(grid_file)
}

// fn wait_for_grid_file(sim: &SimParams) -> Utf8PathBuf {
//     let grid_file = get_grid_file_path(sim);
//     if !grid_file.is_file() {
//         println!("Waiting for grid job to finish.");
//     }
//     while !grid_file.is_file() {
//         thread::sleep(Duration::from_millis(100));
//     }
//     grid_file
// }

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
