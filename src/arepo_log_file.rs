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
        let re = Regex::new("Code run for ([0-9.]+) seconds!").unwrap();
        let num_cores_string = self.get_first_capture_string(&re)?;
        num_cores_string
            .parse()
            .context("Failed to parse run time string in log file")
    }

    pub fn get_first_capture_string(&self, re: &Regex) -> Result<String> {
        let contents = self.get_contents()?;
        let mut captures = re.captures_iter(&contents);
        let capture = captures.next().ok_or(anyhow!(
            "Cannot determine number of cores from log file: {:?}",
            &self.file
        ))?;
        capture
            .get(1)
            .ok_or(anyhow!(
                "No line referencing number of MPI ranks in log file. Simulation not run properly?"
            ))
            .map(|cap| cap.as_str().to_owned())
    }
}
