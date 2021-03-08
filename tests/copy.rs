mod setup;
use bob::util::{get_files, get_folders, read_file_contents};
use camino::Utf8Path;
use setup::TestArg::{NormalArg, RelativePath};
pub use setup::*;

#[test]
fn copy() {
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

#[test]
fn num_special_parameters() {
    let out = run_bob_on_setup(
        "specialParameters",
        &[
            NormalArg("copy"),
            RelativePath("input"),
            RelativePath("output"),
        ],
    );
    dbg!(&out.get_utf8_env_dir());
    for file in get_files(&out.get_utf8_env_dir().join("output").join("0")) {
        dbg!(file);
    }
    let job_file = out.get_utf8_env_dir().join("output").join("0").join("job");
    let contents = read_file_contents(&job_file).unwrap();
    assert!(contents.contains("21 0 10 10"));
    assert!(out.success);
}
