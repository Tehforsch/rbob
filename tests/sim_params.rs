mod setup;
use setup::TestArg::{NormalArg, RelativePath};
pub use setup::*;

use anyhow::Result;
use std::{fs, path::Path};

#[test]
fn different_param_files() -> Result<()> {
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
        println!("{}", output.stdout);
        println!("{}", output.stderr);
        assert!(output.success);
    }
    Ok(())
}

#[test]
fn wrong_param_files() -> Result<()> {
    let out = run_bob_on_setup("wrongParamFiles", &[NormalArg("show"), RelativePath("in")]);
    assert!(!out.success);
    assert!(out.stderr.contains(&"Invalid line in parameter file"));
    Ok(())
}
