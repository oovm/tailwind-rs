pub const BASE62: &[u8; 62] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Encodes a u64 hash to base-62 string.
pub trait Base62 {
    /// Encodes a u64 hash to base-62 string.
    fn base62(&self) -> String;
}

impl Base62 for u64 {
    /// Since `52*62^10 > 2^64`, so 11 digit base62 string can represent u64 hash.
    fn base62(&self) -> String {
        const L: usize = 11;
        const R1: u64 = 52;
        const R2: u64 = 62;
        let mut v = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(L);
        if v > 0 {
            let digit = (v % R1) as usize;
            v /= R1;
            digits.push(BASE62[digit]);
        }
        while v > 0 {
            let digit = (v % R2) as usize;
            v /= R2;
            digits.push(BASE62[digit]);
        }
        while digits.len() < L {
            digits.push(b'A');
        }
        unsafe { String::from_utf8_unchecked(digits) }
    }
}

impl Base62 for u32 {
    /// Since `52*62^5 > 2^32`, so 6 digit base62 string can represent u32 hash.
    fn base62(&self) -> String {
        const L: usize = 6;
        const R1: u32 = 52;
        const R2: u32 = 62;
        let mut v = *self;
        let mut digits: Vec<u8> = Vec::with_capacity(L);
        if v > 0 {
            let digit = (v % R1) as usize;
            v /= R1;
            digits.push(BASE62[digit]);
        }
        while v > 0 {
            let digit = (v % R2) as usize;
            v /= R2;
            digits.push(BASE62[digit]);
        }
        while digits.len() < L {
            digits.push(b'A');
        }
        unsafe { String::from_utf8_unchecked(digits) }
    }
}
