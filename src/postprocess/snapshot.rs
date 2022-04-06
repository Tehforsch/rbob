use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use ndarray::array;
use ndarray::s;
use uom::si::f64::Time;

use super::read_hdf5::get_header_attribute;
use crate::array_utils::FArray1;
use crate::array_utils::FArray2;
use crate::config::SX_NFREQ;
use crate::sim_params::SimParams;

#[derive(Debug)]
pub struct Snapshot {
    file: hdf5::File,
    pub path: Utf8PathBuf,
    pub sim: SimParams,
    pub time: Time,
    size: f64,
    name: String,
}

impl std::fmt::Display for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file.filename())
    }
}

impl Snapshot {
    pub fn min_extent(&self) -> FArray1 {
        array![0., 0., 0.]
    }

    pub fn max_extent(&self) -> FArray1 {
        array![self.size, self.size, self.size]
    }

    pub fn center(&self) -> FArray1 {
        (self.min_extent() + self.max_extent()) * 0.5
    }

    pub fn coordinates(&self) -> Result<FArray2> {
        self.read_2d_dataset("PartType0/Coordinates")
    }

    pub fn density(&self) -> Result<FArray1> {
        self.read_1d_dataset("PartType0/Density")
    }

    pub fn h_plus_abundance(&self) -> Result<FArray1> {
        let full_data = self.read_2d_dataset("PartType0/ChemicalAbundances")?;
        Ok(full_data.slice(s![.., 1]).to_owned())
    }

    pub fn chemical_abundances(&self) -> Result<FArray2> {
        let arr = self.read_2d_dataset("PartType0/ChemicalAbundances")?;
        assert_eq!(arr.shape()[1], SX_NFREQ);
        Ok(arr)
    }

    pub fn energies(&self) -> Result<FArray1> {
        let arr = self.read_1d_dataset("PartType0/InternalEnergy")?;
        Ok(arr)
    }

    pub fn read_2d_dataset(&self, dataset: &str) -> Result<FArray2> {
        Ok(self.file.dataset(dataset)?.read()?)
    }

    pub fn read_1d_dataset(&self, dataset: &str) -> Result<FArray1> {
        Ok(self.file.dataset(dataset)?.read()?)
    }

    pub fn get_header_attribute<Q>(&self, name: &str, unit: Q) -> Result<Q>
    where
        Q: Clone + std::ops::Mul<f64, Output = Q>,
    {
        get_header_attribute(&self.file, name, unit)
    }

    pub fn from_file(sim: &SimParams, file: &Utf8Path) -> Result<Snapshot> {
        let h5file = hdf5::File::open(file)?;
        let time = get_header_attribute(&h5file, "Time", sim.units.time)?;
        let snap_shot_base = sim["SnapshotFileBase"].unwrap_string();
        let name = Utf8Path::new(&h5file.filename())
            .file_name()
            .and_then(|x| x.strip_suffix(".hdf5"))
            .and_then(|x| x.strip_prefix(&snap_shot_base))
            .and_then(|x| x.strip_prefix('_'))
            .unwrap()
            .to_owned();
        Ok(Snapshot {
            file: h5file,
            path: file.to_owned(),
            size: sim["BoxSize"].unwrap_f64(),
            time,
            sim: sim.clone(),
            name,
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
