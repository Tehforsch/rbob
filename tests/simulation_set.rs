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
            NormalArg("MultipleDomains"),
            NormalArg("SX_SWEEP"),
            NormalArg("SX_NUM_ROT"),
        ],
    );
    assert!(out.success);
    compare_output_lines(
        out.output,
        &[
            "0:",
            "\tMultipleDomains: 1",
            "\tSX_SWEEP: true",
            "\tSX_NUM_ROT: 8",
            "1:",
            "\tMultipleDomains: 2",
            "\tSX_SWEEP: false",
            "\tSX_NUM_ROT: 8",
            "2:",
            "\tMultipleDomains: 3",
            "\tSX_SWEEP: false",
            "\tSX_NUM_ROT: 8",
        ],
    );
}
