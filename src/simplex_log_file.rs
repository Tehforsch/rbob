use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use regex::Regex;

use crate::util::read_file_contents;

pub struct SimplexLogFile {
    file: Utf8PathBuf,
}

impl SimplexLogFile {
    pub fn new(file: &Utf8Path) -> Self {
        Self { file: file.into() }
    }

    pub fn get_contents(&self) -> Result<String> {
        read_file_contents(&self.file).context("While reading log file")
    }

    pub fn get_average_ionization_over_time(&self) -> Result<Vec<(f64, f64, f64)>> {
        let re = Regex::new("Time ([0-9.+]+): Volume Av. H ionization: ([0-9.+]+), Mass Av. H ionization: ([0-9.+]+)").unwrap();
        re.captures_iter(&self.get_contents()?)
            .map(|cap| {
                let time = cap.get(1).unwrap().as_str().parse::<f64>()?;
                let volume_ionization = cap.get(2).unwrap().as_str().parse::<f64>()?;
                let mass_ionization = cap.get(2).unwrap().as_str().parse::<f64>()?;
                Ok((time, volume_ionization, mass_ionization))
            })
            .collect()
    }
}
