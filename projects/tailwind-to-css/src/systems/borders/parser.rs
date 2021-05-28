
use super::*;


impl TailwindBorderStyle {
    pub fn into_instance(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }
}