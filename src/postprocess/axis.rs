use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use clap::Clap;
use ndarray::array;
use ndarray::Array1;

#[derive(Clap, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn get_axis_vector(&self) -> Array1<f64> {
        match self {
            Axis::X => array![1., 0., 0.],
            Axis::Y => array![0., 1., 0.],
            Axis::Z => array![0., 0., 1.],
        }
    }

    pub fn get_orthogonal_vectors(&self) -> (Array1<f64>, Array1<f64>) {
        match self {
            Axis::X => (array![0., 1., 0.], array![0., 0., 1.]),
            Axis::Y => (array![1., 0., 0.], array![0., 0., 1.]),
            Axis::Z => (array![1., 0., 0.], array![0., 1., 0.]),
        }
    }
}

impl FromStr for Axis {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Axis> {
        match s {
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(anyhow!("Invalid axis specification {}", s)),
        }
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::Y => write!(f, "y"),
            Self::Z => write!(f, "z"),
        }
    }
}
