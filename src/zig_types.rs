
pub struct ZigConstSlice<T> {
    pub ptr: *const T,
    pub len: usize,
}

impl ZigConstSlice<u8> {
    pub const fn from_string(s: &'static str) -> Self {
        Self {
            ptr: s.as_ptr(),
            len: s.len(),
        }
    }
}

pub trait ZigToString {
    fn zigify(self) -> ZigConstSlice<u8>;
}

impl ZigToString for &'static str {
    fn zigify(self) -> ZigConstSlice<u8> {
        ZigConstSlice::<u8>::from_string(self)
    }
}

pub trait ZigToUSize {
    fn zigify(self) -> usize;
}

impl ZigToUSize for usize {
    fn zigify(self) -> usize {
        self
    }
}
