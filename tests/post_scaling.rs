mod setup;
use setup::run_bob_on_setup;
use setup::TestArg::{NormalArg, RelativePath};
pub use setup::*;

#[test]
fn run_post_scaling() {
    let out = run_bob_on_setup(
        "post_scaling",
        &[NormalArg("post"), RelativePath("."), NormalArg("scaling")],
    );
    dbg!(out.stderr);
    assert!(out.success);
}
