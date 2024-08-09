//! Definitions related to parsing and interacting with the Elf file identifier and header.

use core::mem;

use crate::{
    class::{Class, ClassParse, UnsupportedClass},
    encoding::{EncodingParse, ParseIntegerError, UnsupportedEncoding},
    raw::elf_header::{Elf64Header, ElfIdent, ELF_MAGIC},
};

/// Abstraction of the various formats for a raw [`ElfHeader`].
pub struct ElfHeader<'slice, E: EncodingParse, C: ClassParse> {
    slice: &'slice [u8],
    encoding: E,
    class: C,
}

impl<'slice, E: EncodingParse, C: ClassParse> ElfHeader<'slice, E, C> {
    pub fn parse(slice: &[u8]) -> Result<Self, ParseElfHeaderError> {
        let mut magic = [0; 4];
        magic[0] = crate::encoding::LittleEndian
            .parse_u8_at(mem::offset_of!(ElfIdent, magic) + 0, slice)?;
        magic[1] = crate::encoding::LittleEndian
            .parse_u8_at(mem::offset_of!(ElfIdent, magic) + 1, slice)?;
        magic[2] = crate::encoding::LittleEndian
            .parse_u8_at(mem::offset_of!(ElfIdent, magic) + 2, slice)?;
        magic[3] = crate::encoding::LittleEndian
            .parse_u8_at(mem::offset_of!(ElfIdent, magic) + 3, slice)?;

        if magic != ELF_MAGIC {
            return Err(ParseElfHeaderError::InvalidMagicBytes(magic));
        }

        // So far, encodings are viable for parsing bytes since byte endianness doesn't matter
        // on the single byte level.
        let elf_ident_data =
            crate::encoding::LittleEndian.parse_u8_at(mem::offset_of!(ElfIdent, data), slice)?;
        let encoding = E::from_elf_data(elf_ident_data)?;

        let elf_ident_class =
            encoding.parse_u8_at(mem::offset_of!(ElfIdent, class), slice)?;
        let class = C::from_elf_class(elf_ident_class)?;

        match class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => {


            }
        }

        todo!()
    }
}

pub enum ParseElfHeaderError {
    InvalidMagicBytes([u8; 4]),
    InvalidElfHeaderVersion(u8),
    NonZeroPadding,
    UnsupportedClass(UnsupportedClass),
    UnsupportedEncoding(UnsupportedEncoding),
    ParseIntegerError(ParseIntegerError),
}

impl From<UnsupportedClass> for ParseElfHeaderError {
    fn from(value: UnsupportedClass) -> Self {
        Self::UnsupportedClass(value)
    }
}

impl From<UnsupportedEncoding> for ParseElfHeaderError {
    fn from(value: UnsupportedEncoding) -> Self {
        Self::UnsupportedEncoding(value)
    }
}

impl From<ParseIntegerError> for ParseElfHeaderError {
    fn from(value: ParseIntegerError) -> Self {
        Self::ParseIntegerError(value)
    }
}
