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
    /// # Errors
    ///
    /// Returns [`ParseIntegerError`] if an arithmetic or bounds overflow error occurs.
    fn parse_u8_at(self, offset: usize, data: &[u8]) -> Result<u8, ParseIntegerError>;
    /// Retrives the [`u16`] at `offset` bytes from the start of `data`
    ///
    /// # Errors
    ///
    /// Returns [`ParseIntegerError`] if an arithmetic or bounds overflow error occurs.
    fn parse_u16_at(self, offset: usize, data: &[u8]) -> Result<u16, ParseIntegerError>;
    /// Retrives the [`u32`] at `offset` bytes from the start of `data`
    ///
    /// # Errors
    ///
    /// Returns [`ParseIntegerError`] if an arithmetic or bounds overflow error occurs.
    fn parse_u32_at(self, offset: usize, data: &[u8]) -> Result<u32, ParseIntegerError>;
    /// Retrives the [`u64`] at `offset` bytes from the start of `data`
    ///
    /// # Errors
    ///
    /// Returns [`ParseIntegerError`] if an arithmetic or bounds overflow error occurs.
    fn parse_u64_at(self, offset: usize, data: &[u8]) -> Result<u64, ParseIntegerError>;
    /// Retrives the [`i32`] at `offset` bytes from the start of `data`
    ///
    /// # Errors
    ///
    /// Returns [`ParseIntegerError`] if an arithmetic or bounds overflow error occurs.
    fn parse_i32_at(self, offset: usize, data: &[u8]) -> Result<i32, ParseIntegerError>;
    /// Retrives the [`i64`] at `offset` bytes from the start of `data`
    ///
    /// # Errors
    ///
    /// Returns [`ParseIntegerError`] if an arithmetic or bounds overflow error occurs.
    fn parse_i64_at(self, offset: usize, data: &[u8]) -> Result<i64, ParseIntegerError>;
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

/// An error that is returned when an error happens when parsing an integer
/// using [`EncodingParse`].
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseIntegerError {
    /// An arithmetic error occurred during the parsing attempt.
    ArithmeticError,
    /// A bounds checking error occurred during the parsing attempt.
    BoundsError {
        /// The offset where the intended read was located.
        read_offset: usize,
        /// The size of the intended read.
        read_size: usize,
        /// The size of the area from which the attempted read intended to read.
        data_size: usize,
    },
}

impl fmt::Display for ParseIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BoundsError {
                read_offset,
                read_size,
                data_size,
            } => {
                writeln!(
                    f,
                    "attempted read from buffer of {data_size} bytes \
                    at an offset of {read_offset} bytes of size {read_size} bytes failed"
                )
            }
            Self::ArithmeticError => writeln!(f, "an arithmetic error occurred"),
        }
    }
}

impl error::Error for ParseIntegerError {}

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

    fn parse_u8_at(self, offset: usize, data: &[u8]) -> Result<u8, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u8>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u8>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u8>() }>()
            .expect("broken sizing check");
        Ok(u8::from_le_bytes(data))
    }

    fn parse_u16_at(self, offset: usize, data: &[u8]) -> Result<u16, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u16>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u16>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u16>() }>()
            .expect("broken sizing check");
        Ok(u16::from_le_bytes(data))
    }

    fn parse_u32_at(self, offset: usize, data: &[u8]) -> Result<u32, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u32>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u32>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u32>() }>()
            .expect("broken sizing check");
        Ok(u32::from_le_bytes(data))
    }

    fn parse_u64_at(self, offset: usize, data: &[u8]) -> Result<u64, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u64>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u64>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u64>() }>()
            .expect("broken sizing check");
        Ok(u64::from_le_bytes(data))
    }

    fn parse_i32_at(self, offset: usize, data: &[u8]) -> Result<i32, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<i32>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<i32>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<i32>() }>()
            .expect("broken sizing check");
        Ok(i32::from_le_bytes(data))
    }

    fn parse_i64_at(self, offset: usize, data: &[u8]) -> Result<i64, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<i64>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<i64>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<i64>() }>()
            .expect("broken sizing check");
        Ok(i64::from_le_bytes(data))
    }
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

    fn parse_u8_at(self, offset: usize, data: &[u8]) -> Result<u8, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u8>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u8>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u8>() }>()
            .expect("broken sizing check");
        Ok(u8::from_be_bytes(data))
    }

    fn parse_u16_at(self, offset: usize, data: &[u8]) -> Result<u16, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u16>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u16>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u16>() }>()
            .expect("broken sizing check");
        Ok(u16::from_be_bytes(data))
    }

    fn parse_u32_at(self, offset: usize, data: &[u8]) -> Result<u32, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u32>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u32>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u32>() }>()
            .expect("broken sizing check");
        Ok(u32::from_be_bytes(data))
    }

    fn parse_u64_at(self, offset: usize, data: &[u8]) -> Result<u64, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<u64>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<u64>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<u64>() }>()
            .expect("broken sizing check");
        Ok(u64::from_be_bytes(data))
    }

    fn parse_i32_at(self, offset: usize, data: &[u8]) -> Result<i32, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<i32>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<i32>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<i32>() }>()
            .expect("broken sizing check");
        Ok(i32::from_be_bytes(data))
    }

    fn parse_i64_at(self, offset: usize, data: &[u8]) -> Result<i64, ParseIntegerError> {
        let remaining_space = data
            .len()
            .checked_sub(offset)
            .ok_or(ParseIntegerError::ArithmeticError)?;
        if remaining_space < mem::size_of::<i64>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: offset,
                read_size: mem::size_of::<i64>(),
                data_size: data.len(),
            });
        }

        let data = *data[offset..]
            .first_chunk::<{ mem::size_of::<i64>() }>()
            .expect("broken sizing check");
        Ok(i64::from_be_bytes(data))
    }
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

    fn parse_u8_at(self, offset: usize, data: &[u8]) -> Result<u8, ParseIntegerError> {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u8_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u8_at(offset, data),
        }
    }

    fn parse_u16_at(self, offset: usize, data: &[u8]) -> Result<u16, ParseIntegerError> {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u16_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u16_at(offset, data),
        }
    }

    fn parse_u32_at(self, offset: usize, data: &[u8]) -> Result<u32, ParseIntegerError> {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u32_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u32_at(offset, data),
        }
    }

    fn parse_u64_at(self, offset: usize, data: &[u8]) -> Result<u64, ParseIntegerError> {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_u64_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_u64_at(offset, data),
        }
    }

    fn parse_i32_at(self, offset: usize, data: &[u8]) -> Result<i32, ParseIntegerError> {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_i32_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_i32_at(offset, data),
        }
    }

    fn parse_i64_at(self, offset: usize, data: &[u8]) -> Result<i64, ParseIntegerError> {
        match self {
            Self(Encoding::TwosComplementLittleEndian) => LittleEndian.parse_i64_at(offset, data),
            Self(Encoding::TwosComplementBigEndian) => BigEndian.parse_i64_at(offset, data),
        }
    }
}
