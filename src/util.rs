use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::str;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use pathdiff::diff_paths;

pub fn read_file_contents(path: &Utf8Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("While reading file {}", path))
}

pub fn write_file(path: &Utf8Path, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("While writing file {}", path))
}

// Taken from 'Doug' from
// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy_recursive<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    let mut stack = vec![PathBuf::from(from.as_ref())];

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
            } else if let Some(filename) = path.file_name() {
                let dest_path = dest.join(filename);
                fs::copy(&path, &dest_path).context(format!(
                    "Error copying {} to {}",
                    &path.to_str().unwrap(),
                    &dest_path.to_str().unwrap()
                ))?;
            }
        }
    }

    Ok(())
}

fn traverse_folder_files(folder: &Utf8Path) -> Result<Box<dyn Iterator<Item = Utf8PathBuf>>> {
    let folder_files = Box::new(iter_files(folder)?);
    let sub_folders = iter_folders(folder)?;
    let sub_folder_results = sub_folders.map(|f| traverse_folder_files(&f));
    let sub_folder_files_iterators_result: Result<Vec<Box<dyn Iterator<Item = Utf8PathBuf>>>> =
        sub_folder_results.collect();
    let sub_folder_files_iterator = (sub_folder_files_iterators_result?).into_iter().flatten();
    Ok(Box::new(folder_files.chain(sub_folder_files_iterator)))
}

fn iter_files(folder: &Utf8Path) -> Result<impl Iterator<Item = Utf8PathBuf>> {
    get_entries_with_predicate(folder, Path::is_file)
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

pub fn get_files_recursively(folder: &Utf8Path) -> Result<Vec<Utf8PathBuf>> {
    Ok(traverse_folder_files(folder)?.collect())
}

pub fn get_files(folder: &Utf8Path) -> Result<Vec<Utf8PathBuf>> {
    Ok(iter_files(folder)?.collect())
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

pub fn get_relative_path(folder: &Utf8Path, base_folder: &Utf8Path) -> Result<Utf8PathBuf> {
    let path_buf = diff_paths(folder, base_folder).ok_or_else(|| {
        anyhow!(format!(
            "Failed to construct relative link from {:?} to {:?}",
            folder, base_folder
        ))
    })?;
    Ok(Utf8PathBuf::from_path_buf(path_buf).unwrap())
}

pub fn create_folder_if_nonexistent(folder: &Utf8Path) -> Result<()> {
    if !folder.is_dir() {
        fs::create_dir_all(&folder)?;
    };
    Ok(())
}

pub fn get_common_path<'a>(paths: impl Iterator<Item = &'a Utf8Path> + 'a) -> Option<Utf8PathBuf> {
    let paths = paths.map(|path| Utf8PathBuf::from_path_buf(path.canonicalize().unwrap()).unwrap());
    get_common_path_for_canonicalized_paths(paths)
}

fn get_common_path_for_canonicalized_paths<'a>(
    mut paths: impl Iterator<Item = Utf8PathBuf>,
) -> Option<Utf8PathBuf> {
    let mut common_path = paths.next()?;
    for path in paths {
        let common_components = common_path
            .components()
            .zip(path.components())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a);
        common_path = common_components.collect();
    }
    Some(common_path)
}

#[cfg(test)]
mod tests {
    use camino::Utf8Path;

    use super::get_common_path_for_canonicalized_paths;

    #[test]
    fn test_common_path() {
        let paths = [
            Utf8Path::new("/a/b").to_path_buf(),
            Utf8Path::new("/a/b/c").to_path_buf(),
            Utf8Path::new("/a/b").to_path_buf(),
        ];
        let common_path = get_common_path_for_canonicalized_paths(paths.into_iter());
        assert!(common_path.unwrap().as_str() == "/a/b");
    }

    #[test]
    fn test_common_path_single_path() {
        let paths = [Utf8Path::new("/a/b/c").to_path_buf()];
        let common_path = get_common_path_for_canonicalized_paths(paths.into_iter());
        assert!(common_path.unwrap().as_str() == "/a/b/c");
    }

    #[test]
    fn test_common_path_multiple_subfolders() {
        let paths = [
            Utf8Path::new("/a/b/c").to_path_buf(),
            Utf8Path::new("/a/b/d").to_path_buf(),
            Utf8Path::new("/a/b/e").to_path_buf(),
        ];
        let common_path = get_common_path_for_canonicalized_paths(paths.into_iter());
        assert!(common_path.unwrap().as_str() == "/a/b");
    }
}
