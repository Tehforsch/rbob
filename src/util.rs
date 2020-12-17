use anyhow::{Context, Result};
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;

pub fn read_file_contents(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| "While reading file")
}

pub fn write_file(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| "While writing file")
}

// Taken from 'Doug' from
// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy_recursive<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();
        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)
                .context(format!("Creating directory {}", dest.to_str().unwrap()))?;
        }

        for entry in fs::read_dir(&working_path).context(format!(
            "Reading directory {}",
            &working_path.to_str().unwrap()
        ))? {
            let entry = entry.context(format!(
                "Reading entry in directory {}",
                &working_path.to_str().unwrap()
            ))?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        fs::copy(&path, &dest_path).context(format!(
                            "Error copying {} to {}",
                            &path.to_str().unwrap(),
                            &dest_path.to_str().unwrap()
                        ))?;
                    }
                    None => {}
                }
            }
        }
    }

    Ok(())
}

fn traverse_folder_files(folder: &Path) -> Result<Box<dyn Iterator<Item = PathBuf>>> {
    let folder_files = Box::new(iter_files(folder)?);
    let sub_folders = iter_folders(folder)?;
    let sub_folder_results = sub_folders.map(|f| traverse_folder_files(&f));
    let sub_folder_files_iterators_result: Result<Vec<Box<dyn Iterator<Item = PathBuf>>>> =
        sub_folder_results.collect();
    let sub_folder_files_iterator = (sub_folder_files_iterators_result?)
        .into_iter()
        .flat_map(|it| it);
    Ok(Box::new(folder_files.chain(sub_folder_files_iterator)))
}

fn iter_files(folder: &Path) -> Result<impl Iterator<Item = PathBuf>> {
    get_entries_with_predicate(folder, Path::is_file)
}

fn iter_folders(folder: &Path) -> Result<impl Iterator<Item = PathBuf>> {
    get_entries_with_predicate(folder, Path::is_dir)
}

fn get_entries_with_predicate<F>(
    folder: &Path,
    predicate: F,
) -> Result<impl Iterator<Item = PathBuf>>
where
    F: Fn(&Path) -> bool,
{
    let entries = fs::read_dir(folder)?;
    let dir_entries: std::io::Result<Vec<DirEntry>> = entries.collect();
    Ok(dir_entries?
        .into_iter()
        .map(|entry| entry.path())
        .filter(move |path| predicate(path)))
}

pub fn get_files_recursively(folder: &Path) -> Result<Vec<PathBuf>> {
    Ok(traverse_folder_files(folder)?.collect())
}

pub fn get_files(folder: &Path) -> Result<Vec<PathBuf>> {
    Ok(iter_files(folder)?.collect())
}

pub fn get_folders(folder: &Path) -> Result<Vec<PathBuf>> {
    Ok(iter_folders(folder)?.collect())
}

pub struct ShellCommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub fn get_shell_command_output<T: Display + AsRef<OsStr>>(
    command_str: &str,
    args: &[T],
    working_dir: Option<&Path>,
) -> ShellCommandOutput {
    for arg in args.iter() {
        print!(" {}", arg);
    }
    println!("");
    let mut command = Command::new(command_str);
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    match working_dir {
        Some(dir) => {
            command.current_dir(dir);
        }
        _ => {}
    };
    let child = command
        .spawn()
        .expect(&format!("Failed to run command: {}", command_str));

    let output = child.wait_with_output().expect("Failed to read stdout");
    let exit_code = output.status;
    ShellCommandOutput {
        success: exit_code.success(),
        stdout: str::from_utf8(&output.stdout)
            .expect("Failed to decode stdout as utf8")
            .to_owned(),
        stderr: str::from_utf8(&output.stderr)
            .expect("Failed to decode stderr as utf8")
            .to_owned(),
    }
}

pub fn copy_file<U: AsRef<Path>, V: AsRef<Path>>(source: U, target: V) -> Result<()> {
    fs::copy(&source, &target)
        .with_context(|| {
            format!(
                "While copying Config file to the arepo folder ({:?} to {:?})",
                &source.as_ref(),
                &target.as_ref()
            )
        })
        .map(|_| ())
}
