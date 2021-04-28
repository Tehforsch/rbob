use std::collections::HashMap;

pub struct PlotParams(pub HashMap<String, String>);

impl PlotParams {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, key: &str, value: impl std::fmt::Display) {
        self.0.insert(key.into(), format!("{}", value));
    }
}
