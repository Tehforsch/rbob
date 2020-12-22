use crate::sim_params::SimParams;
use crate::unit_array::{UArray, UArray1, UArray2, UArray3};
use anyhow::Result;
use ndarray::s;
use std::path::Path;
use uom::si::ratio::ratio;
use uom::si::{f64::Length, f64::MassDensity, f64::Ratio};

use super::read_hdf5::{read_1d_unit_array, read_2d_unit_array};

#[derive(Debug)]
pub struct Snapshot<'a> {
    file: hdf5::File,
    sim: &'a SimParams,
}

impl<'a> Snapshot<'a> {
    pub fn coordinates(&self) -> Result<UArray2<Length>> {
        self.read_2d_dataset("Coordinates", self.sim.units.length)
    }

    pub fn density(&self) -> Result<UArray1<MassDensity>> {
        self.read_1d_dataset("Density", self.sim.units.mass_density)
    }

    pub fn H_plus_abundance(&self) -> Result<UArray1<Ratio>> {
        let full_data = self.read_2d_dataset("ChemicalAbundances", Ratio::new::<ratio>(1.0))?;
        full_data.slice(s![1, ..]);
        todo!()
    }

    pub fn chemical_abundances(&self) -> Result<UArray2<Ratio>> {
        let arr = self.read_2d_dataset("ChemicalAbundances", Ratio::new::<ratio>(1.0))?;
        assert_eq!(arr.shape()[1], 6);
        Ok(arr)
    }

    pub fn read_2d_dataset<Q>(&self, dataset: &str, unit: Q) -> Result<UArray2<Q>> {
        read_2d_unit_array(&self.file, &("PartType0/".to_owned() + dataset), unit)
    }

    pub fn read_1d_dataset<Q>(&self, dataset: &str, unit: Q) -> Result<UArray1<Q>> {
        read_1d_unit_array(&self.file, &("PartType0/".to_owned() + dataset), unit)
    }

    pub fn from_file(sim: &'a SimParams, file: &Path) -> Result<Snapshot<'a>> {
        Ok(Snapshot {
            file: hdf5::File::open(file)?,
            sim,
        })
    }
}
