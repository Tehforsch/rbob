use std::path::Path;

use crate::util::get_shell_command_output;
use anyhow::Result;

use crate::unit_array::{UArray1, UArray2, UArray3};

pub fn read_1d_unit_array<Q>(h5_file: &hdf5::File, dataset: &str, unit: Q) -> Result<UArray1<Q>>
where
    Q: Clone,
{
    Ok(UArray1::new(h5_file.dataset(dataset)?.read()?, unit))
}

pub fn read_2d_unit_array<Q>(h5_file: &hdf5::File, dataset: &str, unit: Q) -> Result<UArray2<Q>>
where
    Q: Clone,
{
    Ok(UArray2::new(h5_file.dataset(dataset)?.read()?, unit))
}

pub fn read_3d_unit_array<Q>(h5_file: &hdf5::File, dataset: &str, unit: Q) -> Result<UArray3<Q>>
where
    Q: Clone,
{
    Ok(UArray3::new(h5_file.dataset(dataset)?.read()?, unit))
}

pub fn get_attribute<Q>(path: &Path, name: &str, unit: Q) -> Result<Q>
where
    Q: Clone + std::ops::Mul<f64, Output = Q>,
{
    // let d = h5_file.dataset("Header/Time")?;
    let s = path.to_str().unwrap();
    let output = get_shell_command_output("bash", &["showAttribute.sh", &name, s], None);
    let stripped = output.stdout.strip_suffix("\n").unwrap();
    let parsed = stripped.parse::<f64>()?;
    Ok(unit * parsed)
}
