// use ndarray::{
//     Array, ArrayBase, Data, DataMut, DataOwned, Ix1, Ix2, Ix3, Ix4, OwnedRepr, RawData, SliceInfo,
//     ViewRepr,
// };
// use num_traits::{FromPrimitive, Num, One};
// use std::{fmt::Debug, ops::Mul};
// use std::{fmt::Display, ops::Div};
// use uom::si::f64::Ratio;
// use uom::si::Quantity;

// pub type Float = f64;
// pub type UArray<D, Q> = UnitArray<OwnedRepr<Float>, D, Q>;
// pub type UArray1<Q> = UArray<Ix1, Q>;
// pub type UArray2<Q> = UArray<Ix2, Q>;
// pub type UArray3<Q> = UArray<Ix3, Q>;
// pub type UArray4<Q> = UArray<Ix4, Q>;

// pub struct UnitArray<S, D, Q>
// where
//     D: ndarray::Dimension,
//     S: RawData<Elem = f64>,
//     Q: Clone,
// {
//     data: ArrayBase<S, D>,
//     conversion: Q,
// }

// impl<S, D, Q> UnitArray<S, D, Q>
// where
//     D: ndarray::Dimension,
//     S: RawData<Elem = f64>,
//     Q: Clone,
// {
//     pub fn new(data: ArrayBase<S, D>, conversion: Q) -> UnitArray<S, D, Q> {
//         UnitArray { data, conversion }
//     }

//     pub fn shape(&self) -> &[usize] {
//         self.data.shape()
//     }

//     pub fn slice<Do>(
//         &self,
//         info: &SliceInfo<D::SliceArg, Do>,
//     ) -> UnitArray<ViewRepr<&<S as RawData>::Elem>, Do, Q>
//     where
//         Do: ndarray::Dimension,
//         S: Data,
//     {
//         let s = self.data.slice(info);
//         UnitArray::new(s, self.conversion.clone())
//     }
// }

// impl<S, B, D, Dim, U> Display for UnitArray<S, D, Quantity<Dim, U, B>>
// where
//     D: ndarray::Dimension,
//     S: Debug + Data + RawData<Elem = f64>,
//     B: uom::Conversion<B> + Clone + Num + Debug,
//     Dim: uom::si::Dimension + ?Sized,
//     U: uom::si::Units<B> + ?Sized,
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

// // impl<S, D, Q> UnitArray<S, D, Q>
// // where
// //     D: ndarray::Dimension,
// //     S: Clone + Zero + FromPrimitive + Div<Output = S> + RawData + Data,
// //     Q: std::ops::Mul<<S as RawData>::Elem, Output = Q> + Clone,
// //     <S as RawData>::Elem: Zero + Clone + FromPrimitive + Div<Output = <S as RawData>::Elem>,
// // {
// //     pub fn mean(&self) -> Option<Q> {
// //         let x = self.data.mean();
// //         x.map(|val| self.conversion.clone() * val)
// //     }
// // }

// impl<S, D, Q> UnitArray<S, D, Q>
// where
//     S: Data + RawData<Elem = f64>,
//     D: ndarray::Dimension,
//     Q: Clone + Mul<f64, Output = Q>,
// {
//     pub fn mean(&self) -> Option<Q> {
//         let x = self.data.mean();
//         x.map(|val| self.conversion.clone() * val)
//     }
// }

// impl<S, D, Q> UnitArray<S, D, Q>
// where
//     D: ndarray::Dimension,
//     S: Data + RawData<Elem = f64>,
//     Q: Clone,
// {
//     pub fn to_owned(&self) -> UnitArray<OwnedRepr<f64>, D, Q> {
//         return UnitArray::new(self.data.to_owned(), self.conversion.clone());
//     }
// }

// impl<Q> UArray1<Q>
// where
//     Q: Clone,
// {
//     pub fn from_vec(values: Vec<f64>, unit: Q) -> UArray1<Q> {
//         UnitArray::new(Array::from(values), unit)
//     }
// }

// impl<D, S1, S2, Q1, Q2, Q3> std::ops::Mul<UnitArray<S2, D, Q2>> for UnitArray<S1, D, Q1>
// where
//     S1: FromPrimitive
//         + Div<Output = S1>
//         + Clone
//         + One
//         + Data
//         + DataOwned<Elem = f64>
//         + RawData<Elem = f64>
//         + DataMut,
//     S2: Data<Elem = f64>,
//     D: ndarray::Dimension,
//     Q1: std::ops::Mul<Q2, Output = Q3> + Clone,
//     Q2: Clone,
//     Q3: Clone,
// {
//     type Output = UnitArray<S1, D, Q3>;
//     fn mul(self, rhs: UnitArray<S2, D, Q2>) -> Self::Output {
//         UnitArray::new(self.data * rhs.data, self.conversion * rhs.conversion)
//     }
// }

// impl<D, S1, S2, Q> std::ops::Add<UnitArray<S2, D, Q>> for UnitArray<S1, D, Q>
// where
//     S1: FromPrimitive
//         + Div<Output = S1>
//         + Clone
//         + One
//         + Data
//         + DataOwned<Elem = f64>
//         + RawData<Elem = f64>
//         + DataMut,
//     S2: Data<Elem = f64>,
//     D: ndarray::Dimension,
//     Q: std::ops::Div<Q, Output = Ratio> + Clone,
// {
//     type Output = UnitArray<S1, D, Q>;
//     fn add(self, rhs: UnitArray<S2, D, Q>) -> Self::Output {
//         let ratio: Ratio = self.conversion / rhs.conversion;
//         UnitArray::new(self.data * rhs.data * ratio.value, self.conversion)
//     }
// }

// impl<S, D, Q1, Q3, Dim, U, V> std::ops::Mul<Quantity<Dim, U, V>> for UnitArray<S, D, Q1>
// where
//     S: FromPrimitive
//         + Div<Output = S>
//         + Clone
//         + One
//         + Data
//         + DataOwned<Elem = f64>
//         + RawData<Elem = f64>
//         + DataMut,
//     D: ndarray::Dimension,
//     Q1: std::ops::Mul<Quantity<Dim, U, V>, Output = Q3> + Clone,
//     Dim: uom::si::Dimension + ?Sized,
//     U: uom::si::Units<V> + ?Sized,
//     V: uom::Conversion<V> + uom::num::Num,
//     Q3: Clone,
// {
//     type Output = UnitArray<S, D, Q3>;
//     fn mul(self, rhs: Quantity<Dim, U, V>) -> Self::Output {
//         UnitArray::new(self.data, self.conversion * rhs)
//     }
// }
