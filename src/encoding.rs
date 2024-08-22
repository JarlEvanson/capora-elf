//! Abstraction of the encoding of an [`ElfFile`].

use core::{error, fmt, mem};

use crate::raw::elf_ident::Encoding as RawEncoding;

/// An all-safe-code encoding-aware integer parsing trait.
pub trait EncodingParse: Clone + Copy + PartialEq + Eq {
    /// Retrieves the corresponding encoding-aware integer parsing object from
    /// [`ElfHeader::data`].
    ///
    /// # Errors
    ///
    /// Returns [`UnsupportedEncodingError`] if the [`EncodingParse`] type doesn't support
    /// parsing the encoding specified by `elf_ident_data`.
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncodingError>;

    /// Returns the [`Encoding`] of the current ELF file.
    fn into_encoding(self) -> Encoding;

    /// Retrives the [`u8`] at `offset` bytes from the start of `data`
    ///
    /// # Panics
    ///
    /// Panics if an arithmetic or bounds overflow error occurs.
    fn parse_u8_at(self, offset: usize, data: &[u8]) -> u8;
    /// Retrives the [`u16`] at `offset` bytes from the start of `data`
    ///
    /// # Panics
    ///
    /// Panics if an arithmetic or bounds overflow error occurs.
    fn parse_u16_at(self, offset: usize, data: &[u8]) -> u16;
    /// Retrives the [`u32`] at `offset` bytes from the start of `data`
    ///
    /// # Panics
    ///
    /// Panics if an arithmetic or bounds overflow error occurs.
    fn parse_u32_at(self, offset: usize, data: &[u8]) -> u32;
    /// Retrives the [`u64`] at `offset` bytes from the start of `data`
    ///
    /// # Panics
    ///
    /// Panics if an arithmetic or bounds overflow error occurs.
    fn parse_u64_at(self, offset: usize, data: &[u8]) -> u64;
    /// Retrives the [`i32`] at `offset` bytes from the start of `data`
    ///
    /// # Panics
    ///
    /// Panics if an arithmetic or bounds overflow error occurs.
    fn parse_i32_at(self, offset: usize, data: &[u8]) -> i32;
    /// Retrives the [`i64`] at `offset` bytes from the start of `data`
    ///
    /// # Panics
    ///
    /// Panics if an arithmetic or bounds overflow error occurs.
    fn parse_i64_at(self, offset: usize, data: &[u8]) -> i64;
}

/// Indicates how the ELF file should be parsed with respect to differences in the encoding of
/// integers.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Encoding {
    /// All integers should be parsed as two's complement little-endian format.
    TwosComplementLittleEndian,
    /// All integers should be parsed as two's complement big-endian format.
    TwosComplementBigEndian,
}

/// An error that occurs when the code does not support a particular [`Encoding`]
/// object.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsupportedEncodingError(u8);

impl fmt::Display for UnsupportedEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match RawEncoding(self.0) {
            RawEncoding::NONE => writeln!(f, "no data encoding ELF parsing not supported"),
            RawEncoding::LITTLE_ENDIAN_TWOS => writeln!(
                f,
                "two's complement little-endian ELF parsing not supported"
            ),
            RawEncoding::BIG_ENDIAN_TWOS => {
                writeln!(f, "two's complement big-endian parsing not supported")
            }
            RawEncoding(encoding) => writeln!(f, "unknown data encoding({encoding}) not supported"),
        }
    }
}

impl error::Error for UnsupportedEncodingError {}

macro_rules! setup_func {
    ($kind:ident, $func:ident, $convert:ident) => {
        fn $func(self, offset: usize, data: &[u8]) -> $kind {
            let byte_after = offset
                .checked_add(mem::size_of::<$kind>())
                .expect("`offset + size` overflowed");
            if byte_after >= data.len() {
                if mem::size_of::<$kind>() != 1 {
                    panic!(
                        "attempted read of {} bytes at an offset of {} bytes from {} byte buffer",
                        mem::size_of::<$kind>(),
                        offset,
                        data.len(),
                    )
                } else {
                    panic!(
                        "attempted read of 1 byte at an offset of {} bytes from {} byte buffer",
                        offset,
                        data.len(),
                    )
                }
            }

            let data = *data[offset..]
                .first_chunk::<{ mem::size_of::<$kind>() }>()
                .expect("broken sizing check");
            $kind::$convert(data)
        }
    };
}

/// A zero-sized object offering methods for safe unaligned,
/// two's complement, little-endian parsing.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LittleEndian;

impl EncodingParse for LittleEndian {
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncodingError> {
        if elf_ident_data != 1 {
            return Err(UnsupportedEncodingError(elf_ident_data));
        }
        Ok(LittleEndian)
    }

    fn into_encoding(self) -> Encoding {
        Encoding::TwosComplementLittleEndian
    }

    setup_func!(u8, parse_u8_at, from_le_bytes);
    setup_func!(u16, parse_u16_at, from_le_bytes);
    setup_func!(u32, parse_u32_at, from_le_bytes);
    setup_func!(u64, parse_u64_at, from_le_bytes);
    setup_func!(i32, parse_i32_at, from_le_bytes);
    setup_func!(i64, parse_i64_at, from_le_bytes);
}

/// A zero-sized object offering methods for safe unaligned,
/// two's complement, big-endian parsing.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigEndian;

impl EncodingParse for BigEndian {
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncodingError> {
        if elf_ident_data != 2 {
            return Err(UnsupportedEncodingError(elf_ident_data));
        }
        Ok(BigEndian)
    }

    fn into_encoding(self) -> Encoding {
        Encoding::TwosComplementBigEndian
    }

    setup_func!(u8, parse_u8_at, from_be_bytes);
    setup_func!(u16, parse_u16_at, from_be_bytes);
    setup_func!(u32, parse_u32_at, from_be_bytes);
    setup_func!(u64, parse_u64_at, from_be_bytes);
    setup_func!(i32, parse_i32_at, from_be_bytes);
    setup_func!(i64, parse_i64_at, from_be_bytes);
}

/// An object used to dispatch the encoding to be read from at runtime.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnyEncoding(Encoding);

impl EncodingParse for AnyEncoding {
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncodingError> {
        match RawEncoding(elf_ident_data) {
            RawEncoding::LITTLE_ENDIAN_TWOS => Ok(Self(Encoding::TwosComplementLittleEndian)),
            RawEncoding::BIG_ENDIAN_TWOS => Ok(Self(Encoding::TwosComplementBigEndian)),
            RawEncoding(unsupported) => Err(UnsupportedEncodingError(unsupported)),
        }
    }

    fn into_encoding(self) -> Encoding {
        self.0
    }

    fn parse_u8_at(self, offset: usize, data: &[u8]) -> u8 {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u8_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u8_at(offset, data),
        }
    }

    fn parse_u16_at(self, offset: usize, data: &[u8]) -> u16 {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u16_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u16_at(offset, data),
        }
    }

    fn parse_u32_at(self, offset: usize, data: &[u8]) -> u32 {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u32_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u32_at(offset, data),
        }
    }

    fn parse_u64_at(self, offset: usize, data: &[u8]) -> u64 {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u64_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u64_at(offset, data),
        }
    }

    fn parse_i32_at(self, offset: usize, data: &[u8]) -> i32 {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_i32_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_i32_at(offset, data),
        }
    }

    fn parse_i64_at(self, offset: usize, data: &[u8]) -> i64 {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_i64_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_i64_at(offset, data),
        }
    }
}
