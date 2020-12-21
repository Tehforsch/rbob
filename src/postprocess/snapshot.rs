use crate::unit_array::{UArray, UArray1, UArray2, UArray3};
use anyhow::Result;
use std::path::Path;
use uom::si::{f64::Length, length::meter, time::second};

use super::read_hdf5::read_2d_unit_array;

#[derive(Debug)]
pub struct Snapshot {
    file: hdf5::File,
}

impl Snapshot {
    pub fn coordinates(&self) -> Result<UArray2<Length>> {
        self.read_2d_dataset("Coordinates", Length::new::<meter>(3.0))
    }

    pub fn densities(&self) -> Result<UArray2<Length>> {
        self.read_2d_dataset("Density", Length::new::<meter>(3.0))
    }

    pub fn read_2d_dataset<Q>(&self, dataset: &str, unit: Q) -> Result<UArray2<Q>> {
        read_2d_unit_array(&self.file, &("PartType0/".to_owned() + dataset), unit)
    }

    pub fn from_file(file: &Path) -> Result<Snapshot> {
        Ok(Snapshot {
            file: hdf5::File::open(file)?,
        })
    }
}
