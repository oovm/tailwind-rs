//! Process-stable hashing (FNV-1a 64).
//!
//! Do **not** use `std::collections::hash_map::DefaultHasher` for content hashes —
//! it is randomized per process and breaks cross-run determinism.

#[derive(Clone, Debug)]
pub struct StableHasher {
    state: u64,
}

impl Default for StableHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl StableHasher {
    pub const fn new() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.state ^= u64::from(byte);
        self.state = self.state.wrapping_mul(0x100000001b3);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.write_u8(*b);
        }
    }

    pub fn write_str(&mut self, s: &str) {
        self.write_u64(s.len() as u64);
        self.write_bytes(s.as_bytes());
    }

    pub fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_u16(&mut self, value: u16) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(u8::from(value));
    }

    pub fn finish(self) -> u64 {
        self.state
    }

    pub fn finish_hex(self) -> String {
        format!("{:016x}", self.finish())
    }
}

/// Hash a string to a stable hex digest.
pub fn hash_str(s: &str) -> String {
    let mut h = StableHasher::new();
    h.write_str(s);
    h.finish_hex()
}
