pub mod dir_diff;

use std::process::{Command, Stdio};
use std::str;
use std::{env, fmt::Display};
use std::{ffi::OsStr, fs};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use tempdir::TempDir;

use bob::util::copy_recursive;
use dir_diff::get_first_difference;

pub static TEST_STAGE_PATH: &str = "bobTest";
pub static TEST_SETUPS_PATH: &str = "testSetups";

#[derive(Debug)]
pub struct TestEnv {
    pub dir: TempDir,
    pub executable: PathBuf,
}

pub struct TestOutput {
    pub env: TestEnv,
    pub success: bool,
    pub output: String,
    pub stderr: String,
}

#[allow(dead_code)] // Somehow rust doesnt realize I use these in other modules.
#[derive(Debug, Clone)]
pub enum TestArg<'a> {
    NormalArg(&'a str),
    AbsolutePath(&'a Path),
    RelativePath(&'a str),
}

impl<'a> TestArg<'a> {
    fn convert_to_string(&'a self, dir: &Path) -> Result<String> {
        match self {
            TestArg::NormalArg(s) => Ok(s.to_string()),
            TestArg::AbsolutePath(p) => Ok(p.to_str().unwrap().to_owned()),
            TestArg::RelativePath(p) => Ok(dir.join(p).to_str().unwrap().to_owned()),
        }
    }
}

fn convert_args<'a>(args: &'a [TestArg<'a>], dir: &'a Path) -> Result<Vec<String>> {
    args.iter().map(|arg| arg.convert_to_string(dir)).collect()
}

pub fn setup_test(executable_name: String, setups_folder: &Path, test_name: &str) -> TestEnv {
    let test_dir = env::current_exe().expect("build exe");
    let build_dir = test_dir
        .parent()
        .expect("deps")
        .parent()
        .expect("build dir");
    let executable = build_dir.join(executable_name);
    let env = TestEnv {
        executable: executable.to_path_buf(),
        dir: TempDir::new(TEST_STAGE_PATH).expect("Setup test directory"),
    };
    let source = setups_folder.join(test_name);
    copy_recursive(source, &env.dir).expect("Copying test files");
    env
}

pub fn get_shell_command_output<T: Display + AsRef<OsStr>>(
    command: &str,
    args: &[T],
) -> (bool, String, String) {
    print!("Running {}", command);
    for arg in args.iter() {
        print!(" {}", arg);
    }
    println!("");
    let child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to run command: {}", command));

    let output = child.wait_with_output().expect("Failed to read stdout");
    let exit_code = output.status;
    (
        exit_code.success(),
        str::from_utf8(&output.stdout)
            .expect("Failed to decode stdout as utf8")
            .to_owned(),
        str::from_utf8(&output.stderr)
            .expect("Failed to decode stderr as utf8")
            .to_owned(),
    )
}

pub fn run_bob(env: &TestEnv, args: &[TestArg]) -> Result<(bool, String, String)> {
    Ok(get_shell_command_output(
        env.executable.to_str().unwrap(),
        &convert_args(&args, &env.dir.path())?,
    ))
}

pub fn run_bob_on_setup_with_args(
    binary_name: String,
    setups_folder: &Path,
    setup_name: &str,
    args: &[TestArg],
) -> Result<TestOutput> {
    let env = setup_test(binary_name, setups_folder, setup_name);
    let output = run_bob(&env, args)?;
    Ok(TestOutput {
        env,
        success: output.0,
        output: output.1,
        stderr: output.2,
    })
}

pub fn run_bob_on_setup(setup_name: &str, args: &[TestArg]) -> TestOutput {
    let out = run_bob_on_setup_with_args(
        get_bob_executable(),
        Path::new(TEST_SETUPS_PATH),
        setup_name,
        &args,
    )
    .unwrap();
    show_output(&out);
    out
}

pub fn different_output_folders(env: &TestEnv, produced: &str, desired: &str) -> bool {
    let first_diff =
        get_first_difference(env.dir.path().join(produced), env.dir.path().join(desired)).unwrap();
    match first_diff {
        Some(ref diff) => println!("{}", diff),
        None => {}
    };
    !first_diff.as_ref().is_none()
}

pub fn compare_output_lines(out: String, lines_desired: &[&str]) {
    let mut lines_out = out.lines();
    println!("Comparing lines in output:");
    for line_desired in lines_desired.iter() {
        let line_out = lines_out.next();
        println!("Expected:\n{}\nFound:\n{:?}\n", line_desired, &line_out);
        assert_eq!(line_desired, &line_out.unwrap());
    }
    assert_eq!(lines_out.next(), None);
}

pub fn show_output(out: &TestOutput) {
    println!("Bob stdout:\n{}", &out.output);
    println!("Bob stderr:\n{}", &out.stderr);
}

pub fn get_bob_executable() -> String {
    if cfg!(windows) {
        "bob.exe".to_owned()
    } else {
        "bob".to_owned()
    }
}
