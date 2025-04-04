use crate::Hasher as _;

const BASIS: u32 = 0x811c9dc5;
const PRIME: u32 = 0x1000193;

/// 32-bit Fowler-Noll-Vo hasher
///
/// Specifically this implements the [FNV-1a hash].
///
/// [FNV-1a hash]: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash
pub struct FnvHasher {
    state: u32,
}

impl Default for FnvHasher {
    fn default() -> Self {
        Self { state: BASIS }
    }
}

impl crate::Hasher for FnvHasher {
    #[inline]
    fn finish32(&self) -> u32 {
        self.state
    }
}

impl core::hash::Hasher for FnvHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state ^= u32::from(*byte);
            self.state = self.state.wrapping_mul(PRIME);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.finish32().into()
    }
}
