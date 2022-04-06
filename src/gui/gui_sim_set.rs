use camino::Utf8PathBuf;

#[derive(PartialEq, Eq)]
pub struct GuiSimSet {
    pub path: Utf8PathBuf,
}

impl GuiSimSet {
    pub fn name(&self) -> &str {
        self.path.file_name().unwrap()
    }
}
