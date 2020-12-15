mod setup;

use anyhow::Result;
use setup::compare_output_lines;
use setup::TestArg;
use setup::TestArg::{AbsolutePath, NormalArg, RelativePath};
use setup::TEST_SETUPS_PATH;
use setup::{get_bob_executable, run_bob, run_bob_on_setup, setup_test, TestOutput};
use std::{fs, path::Path};

#[test]
fn check_different_param_files() -> Result<()> {
    let test_setup_name = "exampleParamFiles";
    let env = setup_test(
        get_bob_executable(),
        Path::new(TEST_SETUPS_PATH),
        test_setup_name,
    );
    for dir in fs::read_dir(env.dir.path())? {
        let dir_path = dir?.path();
        assert!(dir_path.is_dir());
        dbg!(&dir_path);
        let output = run_bob(
            &env,
            &[NormalArg("show"), RelativePath(&dir_path.to_str().unwrap())],
        )?;
        let success = output.0;
        let stdout = output.1;
        let stderr = output.2;
        println!("{}", stdout);
        println!("{}", stderr);
        assert!(success);
    }
    Ok(())
}

#[test]
fn check_wrong_param_files() -> Result<()> {
    let out = run_bob_on_setup("wrongParamFiles", &[NormalArg("show"), RelativePath("in")]);
    assert!(!out.success);
    assert!(out.stderr.contains(&"Invalid line in parameter file"));
    Ok(())
}
