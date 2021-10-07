mod setup;
use setup::run_bob_on_setup;
use setup::TestArg::NormalArg;
use setup::TestArg::RelativePath;
pub use setup::*;

#[test]
fn show_diff() {
    let out = run_bob_on_setup(
        "diff",
        &[NormalArg("diff"), RelativePath("0"), RelativePath("1")],
    );
    assert!(out.success);
    compare_output_lines(
        out.output,
        &[
            "< MultipleDomains: 1",
            "> MultipleDomains: 2",
            "< SX_SWEEP: true",
            "> SX_SWEEP: false",
        ],
    );
}
