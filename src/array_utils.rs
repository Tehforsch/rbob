use crate::unit_array::{UArray1, UArray3};
use ndarray::{array, s, Array1, Array3, ArrayView};

pub fn meshgrid2<Q>(min: &UArray1<Q>, max: &UArray1<Q>, n0: usize, n1: usize) -> UArray3<Q>
where
    Q: Clone,
{
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
