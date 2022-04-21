use anyhow::anyhow;
use anyhow::Result;

use crate::config;
use crate::param_value::ParamValue;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::get_shell_command_output;

pub fn run_sim_set(sim_set: &SimSet, verbose: bool) -> Result<()> {
    let mut run_after = None;
    for (i, sim) in sim_set.iter().enumerate() {
        println!("Running sim {}", i);
        let job_id = run_sim(sim, verbose, run_after)?;
        if matches!(
            sim.get(config::CASCADE_IDENTIFIER),
            Some(ParamValue::Bool(true))
        ) {
            match config::JOB_FILE_RUN_COMMAND.as_str() {
                "bash" => {}
                _ => {
                    run_after = job_id;
                    assert!(
                        run_after.is_some(),
                        "Failed to get job id from previous job! Run without -v ? "
                    );
                }
            }
        }
    }
    Ok(())
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

pub fn run_job_file(
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
            format!("--dependency=afterany:{id}", id = id),
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
            None => Ok(None),
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
