use std::collections::HashMap;

use super::plot_info::PlotInfo;
use super::post_fn::PostResult;
use crate::array_utils::FArray2;

pub struct DataPlotInfo {
    pub info: PlotInfo,
    pub data: Vec<FArray2>,
    pub replacements: HashMap<String, String>,
}

impl DataPlotInfo {
    pub fn new(info: PlotInfo, result: PostResult) -> Self {
        Self {
            info,
            data: result.data,
            replacements: result.params.0,
        }
    }
}
