pub trait Plot {
    fn get_command(&self) -> String;
}

pub struct Slice {}
impl Plot for Slice {
    fn get_command(&self) -> String {
        "slice --axis z --field Temperature".into()
    }
}
