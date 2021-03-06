use crate::array_utils::FArray2;

use super::plot_info::PlotInfo;

pub struct DataPlotInfo {
    pub info: PlotInfo,
    pub data: Vec<FArray2>,
}
