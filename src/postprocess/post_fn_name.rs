use super::{
    post_compare::CompareFn,
    post_convergence::ConvergenceFn,
    post_expansion::{DTypeExpansionFn, RTypeExpansionFn},
    post_fn::PostFn,
    post_scaling::ScalingFn,
    post_shadowing::ShadowingFn,
    post_slice::SliceFn,
};
use clap::Clap;

#[derive(Clap, Debug)]
pub enum PostFnName {
    RType(RTypeExpansionFn),
    DType(DTypeExpansionFn),
    Slice(SliceFn),
    Scaling(ScalingFn),
    Compare(CompareFn),
    Shadowing(ShadowingFn),
    Convergence(ConvergenceFn),
}

impl PostFnName {
    pub fn get_function<'a>(&'a self) -> Box<dyn PostFn + 'a> {
        match self {
            Self::Scaling(s) => Box::new(s),
            Self::Slice(s) => Box::new(s),
            Self::Compare(s) => Box::new(s),
            Self::RType(s) => Box::new(s),
            Self::DType(s) => Box::new(s),
            Self::Shadowing(s) => Box::new(s),
            Self::Convergence(s) => Box::new(s),
        }
    }
}

// impl std::fmt::Display for PostFnName {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let name = match self {
//             // Self::Expansion(_) => "expansion".to_owned(),
//             // Self::Slice(s) => format!("slice_{}", s.axis),
//             Self::Scaling(_s) => "scaling".to_owned(),
//         };
//         write!(f, "{}", name)
//     }
// }
