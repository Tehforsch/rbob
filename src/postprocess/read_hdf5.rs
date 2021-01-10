use std::path::Path;

use crate::util::get_shell_command_output;
use anyhow::Result;

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
