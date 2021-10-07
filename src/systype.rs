use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use clap::Clap;

#[derive(Clap, Debug)]
pub enum Systype {
    Asan,
    Gprof,
}

impl FromStr for Systype {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Systype> {
        match s {
            "asan" => Ok(Systype::Asan),
            "gprof" => Ok(Systype::Gprof),
            _ => Err(anyhow!("Invalid compile option {}", s)),
        }
    }
}
