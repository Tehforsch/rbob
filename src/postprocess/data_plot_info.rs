use std::collections::HashMap;

use crate::array_utils::FArray2;

use super::{plot_info::PlotInfo, post_fn::PostResult};

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
            replacements: result.replacements.0,
        }
    }
}
