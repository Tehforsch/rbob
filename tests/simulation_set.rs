mod setup;
use setup::compare_output_lines;
use setup::run_bob_on_setup;
use setup::TestArg;
use setup::TestArg::{AbsolutePath, NormalArg, RelativePath};

#[test]
fn check_simulation_set() {
    let out = run_bob_on_setup(
        "simulationSet",
        &[
            NormalArg("show"),
            RelativePath("in"),
            NormalArg("a"),
            NormalArg("b"),
            NormalArg("c"),
        ],
    );
    assert!(out.success);
    compare_output_lines(
        out.output,
        &[
            "0:", "\ta: 1", "\tb: 4", "\tc: 0", "1:", "\ta: 2", "\tb: 5", "\tc: 0", "2:", "\ta: 3",
            "\tb: 6", "\tc: 0",
        ],
    );
}

#[test]
fn check_simulation_set_wrong_list_lengths() {
    let out = run_bob_on_setup(
        "simulationSetWrongListLengths",
        &[NormalArg("show"), RelativePath("in"), NormalArg("SX_SWEEP")],
    );
    assert!(!out.success);
    compare_output_lines(
        out.stderr,
        &["Error: Found different lengths of parameter lists!"],
    );
}

#[test]
fn check_simulation_set_wrong_list_lengths_cartesian() {
    let out = run_bob_on_setup(
        "simulationSetWrongListLengthsCartesian",
        &[NormalArg("show"), RelativePath("in"), NormalArg("SX_SWEEP")],
    );
    assert!(!out.success);
    compare_output_lines(
        out.stderr,
        &["Error: Found different lengths of parameter lists!"],
    );
}
