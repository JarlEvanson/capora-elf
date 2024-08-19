//! Definitions and interfaces for interacting with the ELF identifier.

use core::mem;

use crate::{
    class::{Class, ClassParse, UnsupportedClass},
    encoding::{Encoding, EncodingParse, ParseIntegerError, UnsupportedEncoding},
    field_size,
    raw::elf_ident::{ElfIdent as RawElfIdent, OsAbi},
};

/// Basic information about an ELF file that can be obtained in an architecture independent manner.
pub struct ElfIdent<'slice, C: ClassParse, E: EncodingParse> {
    slice: &'slice [u8],
    class: C,
    encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfIdent<'slice, C, E> {
    /// Parses an [`ElfIdent`] from the provided `file`, checking as many invariants
    /// as possible.
    pub fn parse(file: &'slice [u8]) -> Result<Self, ParseElfIdentError> {
        if file.len() < mem::size_of::<RawElfIdent>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: 0,
                read_size: mem::size_of::<RawElfIdent>(),
                data_size: file.len(),
            }
            .into());
        }

        if file[..4] != RawElfIdent::MAGIC_BYTES {
            return Err(ParseElfIdentError::InvalidMagicBytes);
        }

        let class = C::from_elf_class(file[mem::offset_of!(RawElfIdent, class)])?;
        let encoding = E::from_elf_data(file[mem::offset_of!(RawElfIdent, data)])?;

        let header_version =
            encoding.parse_u8_at(mem::offset_of!(RawElfIdent, header_version), file)?;

        if header_version != RawElfIdent::CURRENT_VERSION {
            return Err(ParseElfIdentError::UnsupportedElfHeaderVersion);
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

    /// Returns the [`Class`] of the ELF file.
    pub fn class(&self) -> Class {
        self.class.into_class()
    }

    /// Returns the [`Encoding`] of the ELF file.
    pub fn encoding(&self) -> Encoding {
        self.encoding.into_encoding()
    }

    /// Returns the [`OsAbi`] of the ELF file.
    pub fn os_abi(&self) -> OsAbi {
        OsAbi(
            self.encoding
                .parse_u8_at(mem::offset_of!(RawElfIdent, os_abi), self.slice)
                .unwrap(),
        )
    }

    /// Returns the version of the ABI to which the object is targeted.
    pub fn abi_version(&self) -> u8 {
        self.encoding
            .parse_u8_at(mem::offset_of!(RawElfIdent, abi_version), self.slice)
            .unwrap()
    }

    /// Returns the [`ClassParse`] that this ELF identifier header uses.
    pub fn class_parse(&self) -> C {
        self.class
    }

    /// Returns the [`EncodingParse`] that this ELF identifier header uses.
    pub fn encoding_parse(&self) -> E {
        self.encoding
    }
}

/// Various errors that can occur while parsing a [`ElfIdent`].
pub enum ParseElfIdentError {
    /// The bytes occupying the magic bytes location did not match the specified ELF magic bytes.
    InvalidMagicBytes,
    /// The class of the ELF file is unsupported.
    UnsupportedClass(UnsupportedClass),
    /// The encoding of the ELF file is unsupported.
    UnsupportedEncoding(UnsupportedEncoding),
    /// The ELF header version is unsupported.
    UnsupportedElfHeaderVersion,
    /// The padding of the header is non-zero.
    NonZeroPadding,
    /// An error ocurred while parsing an integer.
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
