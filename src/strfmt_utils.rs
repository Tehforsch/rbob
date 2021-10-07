use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use strfmt::strfmt;
use strfmt::FmtError;

pub fn strfmt_anyhow(contents: &str, replacements: HashMap<String, String>) -> Result<String> {
    strfmt(
        contents,
        &replacements.into_iter().map(|(s1, s2)| (s1, s2)).collect(),
    )
    .map_err(|e| match e {
        FmtError::Invalid(s) => anyhow!("Invalid format string: {}", s),
        FmtError::KeyError(s) => anyhow!("Required key not in parameter list: {}", s),
        FmtError::TypeError(s) => anyhow!("Wrong type in parameter for template: {}", s),
    })
}
