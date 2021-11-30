use anyhow::Result;

pub fn get_header_attribute<Q>(file: &hdf5::File, name: &str, unit: Q) -> Result<Q>
where
    Q: Clone + std::ops::Mul<f64, Output = Q>,
{
    Ok(unit * read_attr_f64(file, name)?)
}

pub fn read_attr_f64(file: &hdf5::File, name: &str) -> Result<f64> {
    let attr = file.group("Header")?.attr(name)?;
    let value: Vec<f64> = attr.read_raw()?;
    assert_eq!(value.len(), 1);
    Ok(value[0])
}
