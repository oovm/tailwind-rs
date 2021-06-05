use super::*;

impl TailwindBorderCollapse {
    pub fn into_instance(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }
}
