use super::{post_expansion::ExpansionFn, post_slice::SliceFn};
use clap::Clap;

#[derive(Clap, Debug)]
pub enum PostFnName {
    Expansion(ExpansionFn),
    Slice(SliceFn),
    // Shadowing(ShadowingType),
}

impl std::fmt::Display for PostFnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Expansion(_) => "expansion".to_owned(),
            Self::Slice(s) => format!("slice_{}", s.axis),
        };
        write!(f, "{}", name)
    }
}
