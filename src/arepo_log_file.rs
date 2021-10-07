use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
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

    pub fn get_run_time(&self, sweep: bool) -> Result<f64> {
        let re = match sweep {
            true => Regex::new("Finished sweep in ([0-9.+]+)s").unwrap(),
            false => Regex::new("SX: RUN [0-9]+ took ([0-9.+-e]+) s").unwrap(),
        };
        let contents = self.get_contents()?;
        let run_times: Result<Vec<f64>> = re
            .captures_iter(&contents)
            .map(|cap| {
                let run_time_string = cap.get(1).unwrap().as_str();
                let run_time: f64 = run_time_string.parse()?;
                Ok(run_time)
            })
            .collect();
        let run_times = run_times?;
        assert!(
            run_times.len() > 0,
            "Could not read run time from log file for sim {}!",
            self.file
        );
        let total_run_time = run_times.iter().sum();
        Ok(total_run_time)
    }

    pub fn get_num_sweep_runs(&self) -> Result<usize> {
        let re = Regex::new("Finished sweep in ([0-9.]+)s").unwrap();
        let contents = self.get_contents()?;
        Ok(re.captures_iter(&contents).map(|_| 1).sum())
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
