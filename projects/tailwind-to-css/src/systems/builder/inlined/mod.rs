use super::*;

#[derive(Clone, Debug, Default)]
pub struct Inlined {
    class: Vec<String>,
    style: Vec<String>,
}

impl Inlined {
    pub fn get_class(&self) -> String {
        self.class.join(" ")
    }
    pub fn add_class(&mut self, new: String) {
        self.class.push(new)
    }
    pub fn get_style(&self) -> String {
        self.style.join("")
    }
    pub fn add_style(&mut self, new: String) {
        self.style.push(new)
    }
}