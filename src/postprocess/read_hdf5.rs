use anyhow::Result;

use crate::unit_array::{UArray1, UArray2, UArray3};

pub fn read_1d_unit_array<Q>(h5_file: &hdf5::File, dataset: &str, unit: Q) -> Result<UArray1<Q>> {
    Ok(UArray1::new(h5_file.dataset(dataset)?.read()?, unit))
}

pub fn read_2d_unit_array<Q>(h5_file: &hdf5::File, dataset: &str, unit: Q) -> Result<UArray2<Q>> {
    Ok(UArray2::new(h5_file.dataset(dataset)?.read()?, unit))
}

pub fn read_3d_unit_array<Q>(h5_file: &hdf5::File, dataset: &str, unit: Q) -> Result<UArray3<Q>> {
    Ok(UArray3::new(h5_file.dataset(dataset)?.read()?, unit))
}
