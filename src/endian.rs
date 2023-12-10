/// The endian of the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endian {
    Little,
    Big,
}

macro_rules! impl_read_method {
    ($type:ty, $method:ident, $size:literal) => {
        /// Read a `$type` from a byte slice.
        ///
        /// # Panics
        ///
        /// Panics if the slice is smaller than $size bytes.
        #[inline]
        pub fn $method(self, buf: &[u8]) -> $type {
            match self {
                Self::Little => <$type>::from_le_bytes(buf[..$size].try_into().unwrap()),
                Self::Big => <$type>::from_be_bytes(buf[..$size].try_into().unwrap()),
            }
        }
    };
}

macro_rules! impl_write_method {
    ($type:ty, $method:ident, $size:literal) => {
        /// Write a `$type` to a mutable byte slice.
        ///
        /// # Panics
        ///
        /// Panics if the slice is smaller than $size bytes.
        #[inline]
        pub fn $method(self, buf: &mut [u8], n: $type) {
            match self {
                Self::Little => buf[..$size].copy_from_slice(&n.to_le_bytes()),
                Self::Big => buf[..$size].copy_from_slice(&n.to_be_bytes()),
            }
        }
    };
}

impl Endian {
    /// The native endian.
    #[inline]
    pub const fn native() -> Self {
        #[cfg(target_endian = "little")]
        {
            Self::Little
        }
        #[cfg(target_endian = "big")]
        {
            Self::Big
        }
    }

    // Reading.

    // Unsigned integers
    impl_read_method!(u8, read_u8, 1);
    impl_read_method!(u16, read_u16, 2);
    impl_read_method!(u32, read_u32, 4);
    impl_read_method!(u64, read_u64, 8);
    impl_read_method!(u128, read_u128, 16);

    // Signed integers
    impl_read_method!(i8, read_i8, 1);
    impl_read_method!(i16, read_i16, 2);
    impl_read_method!(i32, read_i32, 4);
    impl_read_method!(i64, read_i64, 8);
    impl_read_method!(i128, read_i128, 16);

    // Floating point numbers
    impl_read_method!(f32, read_f32, 4);
    impl_read_method!(f64, read_f64, 8);

    // Writing.

    // Unsigned integers
    impl_write_method!(u8, write_u8, 1);
    impl_write_method!(u16, write_u16, 2);
    impl_write_method!(u32, write_u32, 4);
    impl_write_method!(u64, write_u64, 8);
    impl_write_method!(u128, write_u128, 16);

    // Signed integers
    impl_write_method!(i8, write_i8, 1);
    impl_write_method!(i16, write_i16, 2);
    impl_write_method!(i32, write_i32, 4);
    impl_write_method!(i64, write_i64, 8);
    impl_write_method!(i128, write_i128, 16);

    // Floating point numbers
    impl_write_method!(f32, write_f32, 4);
    impl_write_method!(f64, write_f64, 8);
}
