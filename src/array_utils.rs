use crate::{
    config::{NX_SLICE, NY_SLICE},
    postprocess::axis::Axis,
};

use ndarray::{array, s, Array1, Array2, Array3, Array4, ArrayView};

pub type Float = f64;
pub type FArray1 = Array1<Float>;
pub type FArray2 = Array2<Float>;
pub type FArray3 = Array3<Float>;
pub type FArray4 = Array4<Float>;

pub fn convert_heatmap_to_gnuplot_format(heatmap: FArray2, center: FArray1, length_factor: f64) -> FArray2 {
    let shape = heatmap.shape();
    let mut result = FArray2::zeros((shape[0] * shape[1], 3));
    for ((i0, i1), v) in heatmap.indexed_iter() {
        result[[i0 * shape[0] + i1, 0]] = ((i0 as f64 / NX_SLICE as f64) - center[[0]]) * length_factor;
        result[[i0 * shape[0] + i1, 1]] = ((i1 as f64 / NY_SLICE as f64) - center[[1]]) * length_factor;
        result[[i0 * shape[0] + i1, 2]] = *v as f64;
    }
    result
}

pub fn get_slice_grid(
    axis: &Axis,
    center: &FArray1,
    min_extent: &FArray1,
    max_extent: &FArray1,
    n0: usize,
    n1: usize,
) -> Box<dyn Iterator<Item = (usize, usize, FArray1)>> {
    let (orth1, orth2) = axis.get_orthogonal_vectors();
    let min_extent_2d = array![orth1.dot(min_extent), orth2.dot(min_extent)];
    let max_extent_2d = array![orth1.dot(max_extent), orth2.dot(max_extent)];
    let grid = meshgrid2(&min_extent_2d, &max_extent_2d, n0, n1);
    let axis = axis.get_axis_vector();
    let center_along_axis = (center.dot(&axis)) * axis;
    Box::new(get_usize_grid(n0, n1).map(move |(i0, i1)| {
        let pos2d = grid.slice(s![i0, i1, ..]);
        (
            i0,
            i1,
            pos2d[0] * &orth1 + pos2d[1] * &orth2 + &center_along_axis,
        )
    }))
}

pub fn get_usize_grid(n0: usize, n1: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
    Box::new(
        (0..n0)
            .into_iter()
            .flat_map(move |i0| (0..n1).into_iter().map(move |i1| (i0, i1))),
    )
}

pub fn meshgrid2(min: &FArray1, max: &FArray1, n0: usize, n1: usize) -> FArray3 {
    let step = (max - min) / array![(n0 - 1) as f64, (n1 - 1) as f64];
    let mut output = Array3::zeros((n0, n1, 2));
    for i0 in 0..n0 {
        for i1 in 0..n1 {
            // output[[i0, i1]] = array![min[0] + i0 as f64 * step[0], min[1] + i1 as f64 * step[1]];
            let mut slice = output.slice_mut(s![i0, i1, ..]);
            slice +=
                &ArrayView::from(&[min[0] + i0 as f64 * step[0], min[1] + i1 as f64 * step[1]]);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    #[test]
    fn test_slice_grid() -> Result<()> {
        let center = array![2., 2., 2.];
        let min_extent = array![1., 1., 1.];
        let max_extent = array![3., 3., 3.];
        let axis = Axis::X;
        let grid: Vec<(usize, usize, FArray1)> =
            get_slice_grid(&axis, &center, &min_extent, &max_extent, 3, 3).collect();
        assert_eq!(
            grid,
            vec![
                (0, 0, array![2., 1., 1.]),
                (0, 1, array![2., 1., 2.]),
                (0, 2, array![2., 1., 3.]),
                (1, 0, array![2., 2., 1.]),
                (1, 1, array![2., 2., 2.]),
                (1, 2, array![2., 2., 3.]),
                (2, 0, array![2., 3., 1.]),
                (2, 1, array![2., 3., 2.]),
                (2, 2, array![2., 3., 3.]),
            ]
        );
        Ok(())
    }
}
