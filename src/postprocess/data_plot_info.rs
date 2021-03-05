

use crate::array_utils::FArray2;

use super::plot::PlotInfo;

pub struct DataPlotInfo {
    pub info: PlotInfo,
    pub data: Vec<FArray2>,
}
