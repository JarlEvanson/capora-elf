//! Definitions and interfaces for interacting with the ELF identifier.

use core::{fmt, mem};

use crate::{
    class::{Class, ClassParse, UnsupportedClassError},
    encoding::{Encoding, EncodingParse, UnsupportedEncodingError},
    field_size,
    raw::elf_ident::{ElfIdent as RawElfIdent, OsAbi},
};

/// Basic information about an ELF file that can be obtained in an architecture independent manner.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ElfIdent<'slice, C: ClassParse, E: EncodingParse> {
    pub(crate) slice: &'slice [u8],
    pub(crate) class: C,
    pub(crate) encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfIdent<'slice, C, E> {
    /// Parses an [`ElfIdent`] from the provided `file`, checking as many invariants
    /// as possible.
    pub fn parse(file: &'slice [u8]) -> Result<Self, ParseElfIdentError> {
        if file.len() < mem::size_of::<RawElfIdent>() {
            return Err(ParseElfIdentError::FileTooSmall);
        }

        if file[..4] != RawElfIdent::MAGIC_BYTES {
            return Err(ParseElfIdentError::InvalidMagicBytes);
        }

        let class = C::from_elf_class(file[mem::offset_of!(RawElfIdent, class)])?;
        let encoding = E::from_elf_data(file[mem::offset_of!(RawElfIdent, data)])?;

        let header_version = file[mem::offset_of!(RawElfIdent, header_version)];
        if header_version != RawElfIdent::CURRENT_VERSION {
            return Err(ParseElfIdentError::UnsupportedElfHeaderVersion);
        }

        if file[mem::offset_of!(RawElfIdent, _padding)..][..field_size!(RawElfIdent, _padding)]
            .iter()
            .any(|&val| val != 0)
        {
            return Err(ParseElfIdentError::NonZeroPadding);
        }

        Ok(Self {
            slice: file,
            class,
            encoding,
        })
    }

    /// Returns the magic bytes that identify this file as an ELF file.
    pub fn magic(&self) -> [u8; 4] {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.slice[..4]);
        bytes
    }

    /// Returns the [`Class`] of the ELF file.
    pub fn class(&self) -> Class {
        self.class.into_class()
    }

    /// Returns the [`Encoding`] of the ELF file.
    pub fn encoding(&self) -> Encoding {
        self.encoding.into_encoding()
    }

    /// Returns the version of the ELF ident header.
    pub fn header_version(&self) -> u8 {
        self.encoding
            .parse_u8_at(mem::offset_of!(RawElfIdent, header_version), self.slice)
    }

    /// Returns the [`OsAbi`] of the ELF file.
    pub fn os_abi(&self) -> OsAbi {
        OsAbi(
            self.encoding
                .parse_u8_at(mem::offset_of!(RawElfIdent, os_abi), self.slice),
        )
    }

    /// Returns the version of the ABI to which the object is targeted.
    pub fn abi_version(&self) -> u8 {
        self.encoding
            .parse_u8_at(mem::offset_of!(RawElfIdent, abi_version), self.slice)
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

impl<'slice, C: ClassParse, E: EncodingParse> fmt::Debug for ElfIdent<'slice, C, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ElfIdent");

        debug_struct.field("magic", &self.magic());
        debug_struct.field("class", &self.class());
        debug_struct.field("data", &self.encoding());
        debug_struct.field("header_version", &self.header_version());
        debug_struct.field("os_abi", &self.os_abi());
        debug_struct.field("abi_version", &self.abi_version());

        debug_struct.finish()
    }
}

/// Various errors that can occur while parsing a [`ElfIdent`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ParseElfIdentError {
    /// The given `file` was too small to contain an [`ElfIdent`].
    FileTooSmall,
    /// The bytes occupying the magic bytes location did not match the specified ELF magic bytes.
    InvalidMagicBytes,
    /// The class of the ELF file is unsupported.
    UnsupportedClassError(UnsupportedClassError),
    /// The encoding of the ELF file is unsupported.
    UnsupportedEncodingError(UnsupportedEncodingError),
    /// The ELF header version is unsupported.
    UnsupportedElfHeaderVersion,
    /// The padding of the header is non-zero.
    NonZeroPadding,
}

impl From<UnsupportedClassError> for ParseElfIdentError {
    fn from(value: UnsupportedClassError) -> Self {
        ParseElfIdentError::UnsupportedClassError(value)
    }
}

impl From<UnsupportedEncodingError> for ParseElfIdentError {
    fn from(value: UnsupportedEncodingError) -> Self {
        ParseElfIdentError::UnsupportedEncodingError(value)
    }
}
