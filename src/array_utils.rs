use ndarray::{array, s, Array1, Array2, Array3, Array4, ArrayView};
pub type Float = f64;
pub type FArray1 = Array1<Float>;
pub type FArray2 = Array2<Float>;
pub type FArray3 = Array3<Float>;
pub type FArray4 = Array4<Float>;

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
