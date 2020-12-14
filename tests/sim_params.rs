mod setup;

use anyhow::Result;
use setup::compare_output_lines;
use setup::run_bob_on_setup;
use setup::TestArg;
use setup::TestArg::{AbsolutePath, NormalArg, RelativePath};
use setup::TEST_SETUPS_PATH;
use std::{fs, path::Path};

#[test]
fn check_simulation_set() -> Result<()> {
    let test_setup_name = "exampleParamFiles";
    for dir in fs::read_dir(Path::new(TEST_SETUPS_PATH).join(test_setup_name))? {
        let dir_path = dir?.path();
        assert!(dir_path.is_file());
        let out = run_bob_on_setup(
            test_setup_name,
            &[NormalArg("show"), RelativePath(dir_path.to_str().unwrap())],
        );
    }
    assert!(false);
    Ok(())
    // assert!(out.success);
    // compare_output_lines(
    //     out.output,
    //     &[
    //         "0:", "\ta: 1", "\tb: 4", "\tc: 0", "1:", "\ta: 2", "\tb: 5", "\tc: 0", "2:", "\ta: 3",
    //         "\tb: 6", "\tc: 0",
    //     ],
    // );
}
