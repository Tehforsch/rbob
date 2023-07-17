use anyhow::anyhow;
use anyhow::Result;

use crate::config;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::get_shell_command_output;

pub fn run_sim_set(sim_set: &SimSet, verbose: bool) -> Result<()> {
    let is_cascade = is_cascade(sim_set.iter().last().unwrap());
    if is_cascade {
        println!("Starting job as cascade");
    }
    let mut run_after = None;
    for (i, sim) in sim_set.iter().enumerate() {
        println!("Running sim {}", i);
        let previous_job_id = run_sim(sim, verbose, run_after)?;
        run_after = previous_job_id.filter(|_| is_cascade);
    }
    Ok(())
}

fn is_cascade(sim: &SimParams) -> bool {
    if let Some(val) = sim.get("postprocess/remap_from") {
        !val.is_null()
    } else {
        false
    }
}

fn run_sim(
    sim: &SimParams,
    verbose: bool,
    dependency_job_id: Option<usize>,
) -> Result<Option<usize>> {
    run_job_file(
        sim,
        &sim.folder.join(config::DEFAULT_JOB_FILE_NAME),
        verbose,
        dependency_job_id,
    )
}

fn run_job_file(
    sim: &SimParams,
    job_file_path: &camino::Utf8Path,
    verbose: bool,
    dependency_job_id: Option<usize>,
) -> Result<Option<usize>> {
    let args = get_run_command_args(job_file_path, dependency_job_id);
    let out = get_shell_command_output(
        &config::JOB_FILE_RUN_COMMAND,
        &args,
        Some(&sim.folder),
        verbose,
    );
    match out.success {
        false => {
            if !verbose {
                println!("{}", out.stdout);
                println!("{}", out.stderr);
            }
            Err(anyhow!("Running the job file failed."))
        }
        true => get_job_id(&out.stdout),
    }
}

fn get_run_command_args(
    job_file_path: &camino::Utf8Path,
    dependency_job_id: Option<usize>,
) -> Vec<String> {
    let job_file_name = job_file_path.file_name().unwrap().into();
    match dependency_job_id {
        Some(id) => vec![
            format!("--dependency=afterok:{id}", id = id),
            job_file_name,
        ],
        None => vec![job_file_name],
    }
}

fn get_job_id(output: &str) -> Result<Option<usize>> {
    if !config::SYSTEM_CONFIG.dependencies_allowed() {
        Ok(None)
    } else {
        use anyhow::Context;
        use regex::Regex;
        let re = Regex::new("Submitted batch job ([0-9]*)").unwrap();
        let capture = re.captures_iter(output).next();
        match capture {
            None => panic!("Could not parse id from job submission output."),
            Some(capture) => Ok(Some(
                capture
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .context("Failed to parse job id as int")?,
            )),
        }
    }
}
