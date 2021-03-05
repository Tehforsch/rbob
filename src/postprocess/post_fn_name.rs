use super::{post_fn::PostFn, post_scaling::ScalingFn};
use clap::Clap;

#[derive(Clap, Debug)]
pub enum PostFnName {
    // Expansion(ExpansionFn),
    // Slice(SliceFn),
    Scaling(ScalingFn),
}

impl PostFnName {
    pub fn get_function<'a>(&'a self) -> impl PostFn + 'a {
        match self {
            Self::Scaling(s) => s,
        }
    }
}

impl std::fmt::Display for PostFnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            // Self::Expansion(_) => "expansion".to_owned(),
            // Self::Slice(s) => format!("slice_{}", s.axis),
            Self::Scaling(_s) => "scaling".to_owned(),
        };
        write!(f, "{}", name)
    }
}
