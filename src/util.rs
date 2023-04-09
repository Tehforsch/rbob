use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::str;

use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;

pub fn write_file(path: &Utf8Path, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("While writing file {}", path))
}

fn iter_folders(folder: &Utf8Path) -> Result<impl Iterator<Item = Utf8PathBuf>> {
    get_entries_with_predicate(folder, Path::is_dir)
}

fn get_entries_with_predicate<F>(
    folder: &Utf8Path,
    predicate: F,
) -> Result<impl Iterator<Item = Utf8PathBuf>>
where
    F: Fn(&Path) -> bool,
{
    let entries = fs::read_dir(folder)?;
    let dir_entries: std::io::Result<Vec<DirEntry>> = entries.collect();
    Ok(dir_entries?
        .into_iter()
        .map(|entry| entry.path())
        .filter(move |path| predicate(path))
        .map(|path| Utf8Path::from_path(&path).unwrap().to_owned()))
}

pub fn get_folders(folder: &Utf8Path) -> Result<Vec<Utf8PathBuf>> {
    Ok(iter_folders(folder)?.collect())
}

#[derive(Debug)]
pub struct ShellCommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub fn get_shell_command_output<T: Display + AsRef<OsStr>>(
    command_str: &str,
    args: &[T],
    working_dir: Option<&Utf8Path>,
    verbose: bool,
) -> ShellCommandOutput {
    let mut command = Command::new(command_str);
    command.args(args).stdin(Stdio::piped());
    if !verbose {
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
    }
    if let Some(dir) = working_dir {
        command.current_dir(dir);
    };
    let child = command
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to run command: {}", command_str));

    let output = child.wait_with_output().expect("Failed to read stdout");
    let exit_code = output.status;
    let result = ShellCommandOutput {
        success: exit_code.success(),
        stdout: str::from_utf8(&output.stdout)
            .expect("Failed to decode stdout as utf8")
            .to_owned(),
        stderr: str::from_utf8(&output.stderr)
            .expect("Failed to decode stderr as utf8")
            .to_owned(),
    };
    if verbose {
        println!("STDERR: {}", result.stderr);
    }
    result
}

pub fn copy_file<U: AsRef<Path>, V: AsRef<Path>>(source: U, target: V) -> Result<()> {
    fs::copy(&source, &target)
        .with_context(|| {
            format!(
                "While copying file ({:?} to {:?})",
                &source.as_ref(),
                &target.as_ref()
            )
        })
        .map(|_| ())
}

pub fn expanduser(path: &Utf8Path) -> Result<Utf8PathBuf> {
    let expanded = shellexpand::tilde(path.as_str());
    let path_buf = Path::new(&*expanded)
        .canonicalize()
        .context(format!("While reading {}", &expanded))?;
    Ok(Utf8PathBuf::from_path_buf(path_buf).unwrap())
}
