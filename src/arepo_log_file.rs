use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use regex::Regex;

use crate::util::read_file_contents;

pub struct ArepoLogFile {
    pub file: Utf8PathBuf,
}

impl ArepoLogFile {
    pub fn new(file: &Utf8Path) -> ArepoLogFile {
        ArepoLogFile {
            file: file.to_owned(),
        }
    }

    pub fn get_contents(&self) -> Result<String> {
        read_file_contents(&self.file).context("While reading log file")
    }

    pub fn get_num_cores(&self) -> Result<i64> {
        let re = Regex::new("Running with ([0-9]+) MPI tasks").unwrap();
        let num_cores_string = self.get_first_capture_string(&re)?;
        num_cores_string
            .parse()
            .context("Failed to parse number of cores in log file")
    }

    pub fn get_run_time(&self) -> Result<f64> {
        let re = Regex::new("Finished sweep in ([0-9.]+)s").unwrap();
        let contents = self.get_contents()?;
        let mut total_run_time = 0.0;
        for cap in re.captures_iter(&contents) {
            let run_time_string = cap.get(1).unwrap().as_str();
            let run_time: f64 = run_time_string.parse()?;
            total_run_time += dbg!(run_time);
        }
        Ok(total_run_time)
    }

    pub fn get_convergence_errors(&self) -> Result<Vec<Vec<f64>>> {
        let re = Regex::new("Sweep PBC iteration (\\d+),\\s*Mean error: ([0-9]+\\.[0-9]+e-[0-9]+)")
            .unwrap();
        let contents = self.get_contents()?;
        let captures = re.captures_iter(&contents);
        let mut result = vec![];
        for cap in captures {
            let iteration_num: i64 = cap.get(1).unwrap().as_str().parse()?;
            let error: f64 = cap.get(2).unwrap().as_str().parse()?;
            if iteration_num == 1 {
                result.push(vec![]);
            }
            result.last_mut().unwrap().push(error);
        }
        Ok(result)
    }

    pub fn get_first_capture_string(&self, re: &Regex) -> Result<String> {
        let contents = self.get_contents()?;
        let mut captures = re.captures_iter(&contents);
        let capture = captures.next().ok_or_else(|| {
            anyhow!(
                "Cannot determine number of cores from log file: {:?}",
                &self.file
            )
        })?;
        capture
            .get(1)
            .ok_or_else(|| {
                anyhow!(
                "No line referencing number of MPI ranks in log file. Simulation not run properly?"
            )
            })
            .map(|cap| cap.as_str().to_owned())
    }
}
