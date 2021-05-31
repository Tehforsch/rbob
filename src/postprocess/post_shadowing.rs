use super::{
    axis::Axis,
    field_identifier::FieldIdentifier,
    get_snapshots,
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
        PostFnKind::Set
    }

    fn name(&self) -> &'static str {
        "shadowing"
    }

    fn qualified_name(&self) -> String {
        self.name().to_string()
    }

    fn post(
        &self,
        sim_set: &SimSet,
        _sim: Option<&SimParams>,
        _snap: Option<&Snapshot>,
    ) -> Result<PostResult> {
        for sim in sim_set.iter() {
            let snap = get_snapshots(sim)?.next().unwrap()?;
            return get_slice_result(&snap, &Axis::Z, &FieldIdentifier::HpAbundance);
        }
        panic!()
    }
}
