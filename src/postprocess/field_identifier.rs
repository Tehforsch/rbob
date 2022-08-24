use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use clap::Clap;

#[derive(Clap, Debug, Clone)]
pub enum FieldIdentifier {
    HpAbundance,
    PhotonRates,
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

impl std::fmt::Display for FieldIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HpAbundance => write!(f, "hpabundance"),
            Self::PhotonRates => write!(f, "photonrates"),
            Self::Density => write!(f, "density"),
        }
    }
}
