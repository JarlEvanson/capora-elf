//! Abstraction of the encoding of an [`ElfFile`].

use core::{error, fmt, mem};

/// An all-safe-code encoding-aware integer parsing trait.
pub trait EncodingParse: Clone + Copy + Default + PartialEq + Eq {
    /// Retrieves the corresponding encoding-aware integer parsing object from
    /// [`ElfHeader::data`].
    ///
    /// # Errors
    ///
    /// Returns [`UnsupportedEncoding`] if the [`EncodingParse`] type doesn't support
    /// parsing the encoding specified by `elf_ident_data`.
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncoding>;

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

/// An error that occurs when the code does not support a particular [`EncodingParse`]
/// object.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsupportedEncoding(u8);

impl fmt::Display for UnsupportedEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => writeln!(f, "invalid data encoding not supported"),
            1 => writeln!(f, "two's complement little endian not supported"),
            2 => writeln!(f, "two's complement big endian not supported"),
            encoding => writeln!(f, "unknown data encoding({encoding}) not supported"),
        }
    }
}

impl error::Error for UnsupportedEncoding {}

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
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LittleEndian;

impl EncodingParse for LittleEndian {
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncoding> {
        if elf_ident_data != 1 {
            return Err(UnsupportedEncoding(elf_ident_data));
        }
        Ok(LittleEndian)
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
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigEndian;

impl EncodingParse for BigEndian {
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncoding> {
        if elf_ident_data != 2 {
            return Err(UnsupportedEncoding(elf_ident_data));
        }
        Ok(BigEndian)
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
pub enum AnyEndian {
    /// Encoding is [`LittleEndian`].
    LittleEndian(LittleEndian),
    /// Encoding is [`BigEndian`].
    BigEndian(BigEndian),
}

impl EncodingParse for AnyEndian {
    fn from_elf_data(elf_ident_data: u8) -> Result<Self, UnsupportedEncoding> {
        match elf_ident_data {
            1 => Ok(AnyEndian::LittleEndian(LittleEndian)),
            2 => Ok(AnyEndian::BigEndian(BigEndian)),
            unsupported => Err(UnsupportedEncoding(unsupported)),
        }
    }

    fn parse_u8_at(self, offset: usize, data: &[u8]) -> Result<u8, ParseIntegerError> {
        match self {
            Self::LittleEndian(encoding) => encoding.parse_u8_at(offset, data),
            Self::BigEndian(encoding) => encoding.parse_u8_at(offset, data),
        }
    }

    fn parse_u16_at(self, offset: usize, data: &[u8]) -> Result<u16, ParseIntegerError> {
        match self {
            Self::LittleEndian(encoding) => encoding.parse_u16_at(offset, data),
            Self::BigEndian(encoding) => encoding.parse_u16_at(offset, data),
        }
    }

    fn parse_u32_at(self, offset: usize, data: &[u8]) -> Result<u32, ParseIntegerError> {
        match self {
            Self::LittleEndian(encoding) => encoding.parse_u32_at(offset, data),
            Self::BigEndian(encoding) => encoding.parse_u32_at(offset, data),
        }
    }

    fn parse_u64_at(self, offset: usize, data: &[u8]) -> Result<u64, ParseIntegerError> {
        match self {
            Self::LittleEndian(encoding) => encoding.parse_u64_at(offset, data),
            Self::BigEndian(encoding) => encoding.parse_u64_at(offset, data),
        }
    }

    fn parse_i32_at(self, offset: usize, data: &[u8]) -> Result<i32, ParseIntegerError> {
        match self {
            Self::LittleEndian(encoding) => encoding.parse_i32_at(offset, data),
            Self::BigEndian(encoding) => encoding.parse_i32_at(offset, data),
        }
    }

    fn parse_i64_at(self, offset: usize, data: &[u8]) -> Result<i64, ParseIntegerError> {
        match self {
            Self::LittleEndian(encoding) => encoding.parse_i64_at(offset, data),
            Self::BigEndian(encoding) => encoding.parse_i64_at(offset, data),
        }
    }
}

impl Default for AnyEndian {
    fn default() -> Self {
        AnyEndian::LittleEndian(LittleEndian)
    }
}
