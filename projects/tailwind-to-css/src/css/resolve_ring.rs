use super::*;
use crate::attributes_ensure;

impl Display for CssRingResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for CssRingResolver {
    fn default() -> Self {
        todo!()
    }
}

impl CssAttributes {
    attributes_ensure! {ring_resolver_mut,ring_resolver, CssRingResolver}

    pub fn tw_ring_offset_width(&mut self) {
        // self.ring_resolver_mut().tw_ring_offset_width
    }
}
