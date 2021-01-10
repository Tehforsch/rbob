use uom::{
    si::{Quantity, SI},
    typenum::{int::Z0, N3, P1},
    Kind,
};
pub type Quant<T> = Quantity<T, SI<f64>, f64>;


pub type Length = dyn uom::si::Dimension<
    L = P1,
    I = Z0,
    J = Z0,
    M = Z0,
    Th = Z0,
    N = Z0,
    Kind = dyn Kind,
    T = Z0,
>;

pub type MassDensity = uom::si::Dimension<
    L = N3,
    I = Z0,
    J = Z0,
    M = P1,
    Th = Z0,
    N = Z0,
    T = Z0,
    Kind = (dyn Kind + 'static),
>;

pub type Time = uom::si::Dimension<
    L = Z0,
    I = Z0,
    J = Z0,
    M = Z0,
    Th = Z0,
    N = Z0,
    Kind = (dyn Kind + 'static),
    T = P1,
>;

pub type Ratio = uom::si::Dimension<
    L = Z0,
    I = Z0,
    J = Z0,
    M = Z0,
    Th = Z0,
    N = Z0,
    Kind = (dyn Kind + 'static),
    T = Z0,
>;
