mod setup;
use setup::TestArg::{NormalArg, RelativePath};
pub use setup::*;

#[test]
fn check_copy() {
    let out = run_bob_on_setup(
        "copy",
        &[
            NormalArg("copy"),
            RelativePath("in"),
            RelativePath("produced_out"),
        ],
    );
    assert!(out.success);
    assert!(!different_output_folders(&out.env, "produced_out", "out"));
}
