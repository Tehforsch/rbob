use camino::Utf8PathBuf;

pub struct GuiSimSet {
    pub path: Utf8PathBuf,
}

impl GuiSimSet {
    pub fn name(&self) -> &str {
        self.path.file_name().unwrap()
    }
}
