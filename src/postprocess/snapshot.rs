use anyhow::Result;
use hdf5::Dataset;
use ndarray::Array2;
use std::path::Path;

#[derive(Debug)]
pub struct Snapshot {}

impl Snapshot {
    pub fn from_file(file: &Path) -> Result<Snapshot> {
        let hdf5_file = hdf5::File::open(file)?;
        let c: Dataset = hdf5_file.group("PartType0")?.dataset("Coordinates")?;
        println!("hi");
        let array: Array2<f64> = c.read()?;
        dbg!(&array);
        todo!()
    }
}
