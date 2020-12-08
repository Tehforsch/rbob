mod setup;
use setup::compare_output_lines;
use setup::run_bob_on_setup;
use setup::TestArg;
use setup::TestArg::{AbsolutePath, NormalArg, RelativePath};

#[test]
fn check_simulation_set() {
    let out = run_bob_on_setup(
        "simulationSet",
        &[NormalArg("show"), RelativePath("in"), NormalArg("SX_SWEEP")],
    );
    assert!(out.success);
    compare_output_lines(out.output, &["0"]);
}
