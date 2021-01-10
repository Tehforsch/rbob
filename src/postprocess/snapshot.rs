use crate::sim_params::SimParams;
use crate::unit_array::{UArray1, UArray2};
use anyhow::Result;
use ndarray::{array, s, Array1};
use std::path::Path;
use uom::si::ratio::ratio;
use uom::si::{f64::Length, f64::MassDensity, f64::Ratio, f64::Time};

use super::read_hdf5::{get_attribute, read_1d_unit_array, read_2d_unit_array};

#[derive(Debug)]
pub struct Snapshot<'a> {
    file: hdf5::File,
    sim: &'a SimParams,
    pub time: Time,
}

impl<'a> std::fmt::Display for Snapshot<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file.filename())
    }
}

impl<'a> Snapshot<'a> {
    pub fn min_extent(&self) -> UArray1<Length> {
        UArray1::new(array![0., 0.], self.sim.units.length)
    }

    pub fn max_extent(&self) -> UArray1<Length> {
        UArray1::new(array![1., 1.], self.sim.units.length)
    }

    pub fn center(&self) -> UArray1<Length> {
        (self.min_extent() + self.max_extent()) / 2
    }

    pub fn coordinates(&self) -> Result<UArray2<Length>> {
        self.read_2d_dataset("Coordinates", self.sim.units.length)
    }

    pub fn density(&self) -> Result<UArray1<MassDensity>> {
        self.read_1d_dataset("Density", self.sim.units.mass_density)
    }

    pub fn h_plus_abundance(&self) -> Result<UArray1<Ratio>> {
        let full_data = self.read_2d_dataset("ChemicalAbundances", Ratio::new::<ratio>(1.0))?;
        Ok(full_data.slice(s![1, ..]).to_owned())
    }

    pub fn chemical_abundances(&self) -> Result<UArray2<Ratio>> {
        let arr = self.read_2d_dataset("ChemicalAbundances", Ratio::new::<ratio>(1.0))?;
        assert_eq!(arr.shape()[1], 6);
        Ok(arr)
    }

    pub fn read_2d_dataset<Q>(&self, dataset: &str, unit: Q) -> Result<UArray2<Q>>
    where
        Q: Clone,
    {
        read_2d_unit_array(&self.file, &("PartType0/".to_owned() + dataset), unit)
    }

    pub fn read_1d_dataset<Q>(&self, dataset: &str, unit: Q) -> Result<UArray1<Q>>
    where
        Q: Clone,
    {
        read_1d_unit_array(&self.file, &("PartType0/".to_owned() + dataset), unit)
    }

    pub fn from_file(sim: &'a SimParams, file: &Path) -> Result<Snapshot<'a>> {
        Ok(Snapshot {
            file: hdf5::File::open(file)?,
            time: get_attribute(file, "Header/Time", sim.units.time)?,
            sim,
        })
    }
}
