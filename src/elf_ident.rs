//! Definitions and interfaces for interacting with the ELF identifier.

use core::mem;

use crate::{
    class::{ClassParse, UnsupportedClass},
    encoding::{EncodingParse, ParseIntegerError, UnsupportedEncoding},
    field_size,
    raw::elf_ident::ElfIdent as RawElfIdent,
};

pub struct ElfIdent<'slice, C: ClassParse, E: EncodingParse> {
    slice: &'slice [u8],
    class: C,
    encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfIdent<'slice, C, E> {
    /// Parses an [`ElfIdent`] from the provided `file`, checking as many invariants
    /// as possible.
    pub fn parse(file: &'slice [u8]) -> Result<Self, ParseElfIdentError> {
        if file.len() < mem::size_of_val(&RawElfIdent::MAGIC_BYTES) {
            return Err(ParseIntegerError::BoundsError {
                read_offset: mem::offset_of!(RawElfIdent, magic),
                read_size: field_size!(RawElfIdent, magic),
                data_size: file.len(),
            }
            .into());
        }
        if file[..4] != RawElfIdent::MAGIC_BYTES {
            return Err(ParseElfIdentError::InvalidMagicBytes);
        }

        if file.len()
            < const { mem::offset_of!(RawElfIdent, class) + field_size!(RawElfIdent, class) }
        {
            return Err(ParseIntegerError::BoundsError {
                read_offset: mem::offset_of!(RawElfIdent, class),
                read_size: field_size!(RawElfIdent, class),
                data_size: file.len(),
            }
            .into());
        }
        let class = C::from_elf_class(file[mem::offset_of!(RawElfIdent, class)])?;

        if file.len()
            < const { mem::offset_of!(RawElfIdent, data) + field_size!(RawElfIdent, data) }
        {
            return Err(ParseIntegerError::BoundsError {
                read_offset: mem::offset_of!(RawElfIdent, data),
                read_size: field_size!(RawElfIdent, data),
                data_size: file.len(),
            }
            .into());
        }
        let encoding = E::from_elf_data(file[mem::offset_of!(RawElfIdent, data)])?;

        let header_version =
            encoding.parse_u8_at(mem::offset_of!(RawElfIdent, header_version), file)?;
        if header_version != RawElfIdent::CURRENT_VERSION {
            return Err(ParseElfIdentError::UnsupportedElfHeaderVersion);
        }

        if file.len()
            < const { mem::offset_of!(RawElfIdent, _padding) + field_size!(RawElfIdent, _padding) }
        {
            return Err(ParseIntegerError::BoundsError {
                read_offset: mem::offset_of!(RawElfIdent, _padding),
                read_size: field_size!(RawElfIdent, _padding),
                data_size: file.len(),
            }
            .into());
        }
        if file[mem::offset_of!(RawElfIdent, _padding)..][..field_size!(RawElfIdent, _padding)]
            .iter()
            .all(|&val| val == 0)
        {
            return Err(ParseElfIdentError::NonZeroPadding);
        }

        Ok(Self {
            slice: file,
            class,
            encoding,
        })
    }
}

pub enum ParseElfIdentError {
    InvalidMagicBytes,
    UnsupportedClass(UnsupportedClass),
    UnsupportedEncoding(UnsupportedEncoding),
    UnsupportedElfHeaderVersion,
    NonZeroPadding,
    ParseIntegerError(ParseIntegerError),
}

impl From<ParseIntegerError> for ParseElfIdentError {
    fn from(value: ParseIntegerError) -> Self {
        ParseElfIdentError::ParseIntegerError(value)
    }
}

impl From<UnsupportedClass> for ParseElfIdentError {
    fn from(value: UnsupportedClass) -> Self {
        ParseElfIdentError::UnsupportedClass(value)
    }
}

impl From<UnsupportedEncoding> for ParseElfIdentError {
    fn from(value: UnsupportedEncoding) -> Self {
        ParseElfIdentError::UnsupportedEncoding(value)
    }
}
