use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use super::plot_info::PlotInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlotInfoFileContents {
    pub info: PlotInfo,
    pub replacements: HashMap<String, String>,
}
