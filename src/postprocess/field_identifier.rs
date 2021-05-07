use anyhow::{anyhow, Result};
use clap::Clap;
use std::str::FromStr;

#[derive(Clap, Debug)]
pub enum FieldIdentifier {
    HpAbundance,
    Density,
}

impl FromStr for FieldIdentifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "abundance" => Ok(Self::HpAbundance),
            "density" => Ok(Self::Density),
            _ => Err(anyhow!("Invalid field specification {}", s)),
        }
    }
}

// impl std::fmt::Display for Axis {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::X => write!(f, "x"),
//             Self::Y => write!(f, "y"),
//             Self::Z => write!(f, "z"),
//         }
//     }
// }
