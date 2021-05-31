use super::{
    axis::Axis,
    field_identifier::FieldIdentifier,
    post_fn::{PostFn, PostResult},
    post_slice::get_slice_result,
};
use super::{post_fn::PostFnKind, snapshot::Snapshot};
use crate::{sim_params::SimParams, sim_set::SimSet};
use anyhow::Result;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct ShadowingFn {}

impl PostFn for &ShadowingFn {
    fn kind(&self) -> PostFnKind {
        PostFnKind::Snap
    }

    fn name(&self) -> &'static str {
        "shadowing"
    }

    fn qualified_name(&self) -> String {
        format!("{}", self.name())
    }

    fn post(
        &self,
        _sim_set: &SimSet,
        _sim: Option<&SimParams>,
        snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        get_slice_result(snap.unwrap(), &Axis::Z, &FieldIdentifier::HpAbundance)
    }
}
