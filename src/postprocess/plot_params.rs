use std::collections::HashMap;

pub struct PlotParams(pub HashMap<String, String>);

impl PlotParams {
    pub fn add(&mut self, key: &str, value: impl std::fmt::Display) {
        self.0.insert(key.into(), format!("{}", value));
    }
}

impl Default for PlotParams {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
