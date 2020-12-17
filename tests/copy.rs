mod setup;
use setup::run_bob_on_setup;
use setup::TestArg;
use setup::TestArg::{AbsolutePath, NormalArg, RelativePath};
use setup::{compare_output_lines, different_output_folders};

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
