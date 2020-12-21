use anyhow::Result;

use crate::args::PostFn;
use crate::sim_set::SimSet;

pub mod post_expansion;
pub mod snapshot;

pub fn postprocess_sim_set(sim_set: &SimSet, function: PostFn) -> Result<()> {
    for sim in sim_set.iter() {
        let (post_function, plot_function) = match function {
            PostFn::Expansion => (post_expansion::post, post_expansion::plot),
        };
        post_function(sim);
        plot_function(sim);
    }
    Ok(())
}
