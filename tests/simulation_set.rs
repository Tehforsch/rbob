mod setup;
use setup::compare_output_lines;
use setup::run_bob_on_setup;
use setup::TestArg;
use setup::TestArg::{AbsolutePath, NormalArg, RelativePath};

#[test]
fn check_sim_set() {
    let out = run_bob_on_setup(
        "simSet",
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
            "\tMultipleDomains: Int(1)",
            "\tSX_SWEEP: Bool(true)",
            "\tSX_NUM_ROT: Int(8)",
            "1:",
            "\tMultipleDomains: Int(2)",
            "\tSX_SWEEP: Bool(false)",
            "\tSX_NUM_ROT: Int(8)",
            "2:",
            "\tMultipleDomains: Int(3)",
            "\tSX_SWEEP: Bool(false)",
            "\tSX_NUM_ROT: Int(8)",
        ],
    );
}
