use anyhow::Result;

pub fn get_header_attribute<Q>(file: &hdf5::File, name: &str, unit: Q) -> Result<Q>
where
    Q: Clone + std::ops::Mul<f64, Output = Q>,
{
    let attr = file.group("Header")?.attr(name)?;
    let value: Vec<f64> = attr.read_raw()?;
    assert_eq!(value.len(), 1);
    Ok(unit * value[0])
}
