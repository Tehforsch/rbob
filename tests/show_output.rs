mod setup;
use setup::run_bob_on_setup;
use setup::TestArg::{NormalArg, RelativePath};
pub use setup::*;

#[test]
fn show_output() {
    let out = run_bob_on_setup(
        "showOutput",
        &[
            NormalArg("show-output"),
            RelativePath("."),
            NormalArg("MultipleDomains"),
        ],
    );
    assert!(out.success);
    compare_output_lines(
        out.output,
        &[
            "0:",
            "\tMultipleDomains: Int(1)",
            "1:",
            "\tMultipleDomains: Int(2)",
            "4:",
            "\tMultipleDomains: Int(3)",
        ],
    );
}
