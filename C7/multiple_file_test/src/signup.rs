pub mod util;

pub struct OtherStructLol {
    pub name: String,
    pub other_attr: u32,
}

impl OtherStructLol {
    pub fn new(name: &str) -> Self {
        OtherStructLol {
            name: name.to_string(),
            other_attr: 4,
        }
    }
}
