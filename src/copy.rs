use std::fs;

use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;

use crate::config;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

pub fn copy_sim_set<U: AsRef<Utf8Path>>(
    sim_set: &SimSet,
    input_folder: U,
    output_folder: U,
    delete: bool,
    symlink_ics: bool,
) -> Result<SimSet> {
    let output_folder = output_folder.as_ref();
    if delete && output_folder.is_dir() {
        fs::remove_dir_all(output_folder)
            .with_context(|| "When deleting the previous output folder")?;
    }
    fs::create_dir(output_folder).with_context(|| "When creating the output folder")?;
    let output_sim_set: Result<SimSet> = sim_set
        .enumerate()
        .map(|(i, sim)| -> Result<(usize, SimParams)> {
            println!("Copying files for sim {}:", i);
            let sim_output_folder = output_folder.join(i.to_string());
            Ok((
                *i,
                copy_sim(sim, input_folder.as_ref(), &sim_output_folder, symlink_ics)
                    .with_context(|| format!("When copying simulation {}", i))?,
            ))
        })
        .collect();
    symlink_to_plot_folder(input_folder, output_folder);
    output_sim_set
}

fn copy_sim(
    sim: &SimParams,
    input_folder: &Utf8Path,
    sim_output_folder: &Utf8Path,
    symlink_ics: bool,
) -> Result<SimParams> {
    assert_eq!(input_folder, sim.folder);
    fs::create_dir_all(sim_output_folder)?;
    sim.write_param_file(&sim_output_folder.join(config::DEFAULT_PARAM_FILE_NAME))?;
    sim.write_job_file(&sim_output_folder.join(config::DEFAULT_JOB_FILE_NAME))?;
    sim.copy_ics(sim_output_folder, symlink_ics)?;
    // This is not the most efficient thing ever but it should be completely fine since this is not done very often, nor is the hashmap very large.
    let mut new_sim_params = sim.clone();
    new_sim_params.folder = sim_output_folder.to_owned();
    Ok(new_sim_params)
}

fn symlink_to_plot_folder(source_folder: impl AsRef<Utf8Path>, sim_output_folder: &Utf8Path) {
    let source = source_folder.as_ref().join("plots");
    if source.exists() {
        let target = sim_output_folder.join("plots");
        let source = &source.canonicalize();
        match source {
            Err(e) => {
                println!(
                    "While trying to obtain absolute path to plots folder at {:?}: {}",
                    source, e
                );
            }
            Ok(source) => std::os::unix::fs::symlink(&source, &target).unwrap_or_else(|e| {
                println!(
                    "While trying to create symlink to plots folder at {:?}: {}",
                    target, e
                );
            }),
        };
    }
}
