use ndarray::{
    arr1, Array, ArrayBase, ArrayView, Data, Ix1, Ix2, Ix3, Ix4, OwnedRepr, RawData, SliceInfo,
};
use num_traits::{FromPrimitive, Num, One, Zero};
use std::{fmt::Display, ops::Div};
use uom::si::Quantity;

// pub type Float = f64;
// pub type UArray<D, Q> = UnitArray<OwnedRepr<Float>, D, Q>;
// pub type UArray1<Q> = UArray<Ix1, Q>;
// pub type UArray2<Q> = UArray<Ix2, Q>;
// pub type UArray3<Q> = UArray<Ix3, Q>;
// pub type UArray4<Q> = UArray<Ix4, Q>;

// #[derive(Debug)]
pub struct UnitArray<S, D, Q>
where
    D: ndarray::Dimension,
    S: Data,
{
    data: ArrayBase<S, D>,
    conversion: Q,
}

impl<S, D, Q> UnitArray<S, D, Q>
where
    D: ndarray::Dimension,
    S: Data,
{
    pub fn new(data: ArrayBase<S, D>, conversion: Q) -> UnitArray<S, D, Q> {
        let d = arr1(&[1.0, 2.0, 3.0]);
        let x = d * (1.0 as f64);
        UnitArray { data, conversion }
    }

    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    pub fn slice<Do>(&self, info: &SliceInfo<D::SliceArg, Do>) -> UnitArray<S, Do, Q>
    where
        Do: ndarray::Dimension,
    {
        UnitArray::new(self.data.slice(info), self.conversion)
    }
}

// impl<A, D, Dim, U> Display for UnitArray<A, D, Quantity<Dim, U, A>>
// where
//     D: ndarray::Dimension,
//     A: std::ops::Mul<Quantity<Dim, U, A>, Output = Quantity<Dim, U, A>>
//         + Copy
//         + Num
//         + uom::Conversion<A>
//         + Display
//         + std::fmt::Debug,
//     Dim: uom::si::Dimension + ?Sized,
//     U: uom::si::Units<A> + ?Sized,
//     // V: uom::Conversion<V> + uom::num::Num,
// {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             fmt,
//             "{} * {:?}",
//             self.data,
//             self.conversion // &self.conversion
//         )
//     }
// }

// impl<A, D, Q> UnitArray<A, D, Q>
// where
//     D: ndarray::Dimension,
//     A: Clone + Zero + FromPrimitive + Div<Output = A>,
//     Q: std::ops::Mul<A, Output = Q> + Clone,
// {
//     pub fn mean(&self) -> Option<Q> {
//         let x = self.data.mean();
//         x.map(|val| self.conversion.clone() * val)
//     }
// }

// impl<A, D, Q1, Q2, Q3> std::ops::Mul<UnitArray<A, D, Q2>> for UnitArray<A, D, Q1>
// where
//     A: FromPrimitive + Div<Output = A> + Clone + One,
//     D: ndarray::Dimension,
//     Q1: std::ops::Mul<Q2, Output = Q3>,
// {
//     type Output = UnitArray<A, D, Q3>;
//     fn mul(self, rhs: UnitArray<A, D, Q2>) -> Self::Output {
//         UnitArray::new(self.data * rhs.data, self.conversion * rhs.conversion)
//     }
// }

// impl<A, D, Q1, Q3, Dim, U, V> std::ops::Mul<Quantity<Dim, U, V>> for UnitArray<A, D, Q1>
// where
//     A: FromPrimitive + Div<Output = A> + Clone + One,
//     D: ndarray::Dimension,
//     Q1: std::ops::Mul<Quantity<Dim, U, V>, Output = Q3>,
//     Dim: uom::si::Dimension + ?Sized,
//     U: uom::si::Units<V> + ?Sized,
//     V: uom::Conversion<V> + uom::num::Num,
// {
//     type Output = UnitArray<A, D, Q3>;
//     fn mul(self, rhs: Quantity<Dim, U, V>) -> Self::Output {
//         UnitArray::new(self.data, self.conversion * rhs)
//     }
// }
