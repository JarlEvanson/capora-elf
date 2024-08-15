//! A simple library providing a pure-safe-rust interface for reading ELF object files.
//!

#![no_std]

use core::mem;

use crate::{
    class::{ClassParse, UnsupportedClass},
    encoding::{EncodingParse, ParseIntegerError, UnsupportedEncoding},
    raw::elf_ident::ElfIdent,
};

pub mod class;
pub mod elf_ident;
pub mod encoding;
pub mod raw;

/// A partially parsed ELF file.
pub struct ElfFile<'slice, C: ClassParse, E: EncodingParse> {
    slice: &'slice [u8],
    class: C,
    encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfFile<'slice, C, E> {
    /// Parses a valid [`ElfFile`] from `slice`.
    ///
    /// # Errors
    ///
    /// Returns [`ParseElfFileError`] if anything that does not comply
    /// with the specification is detected.
    pub fn parse(file: &'slice [u8]) -> Result<Self, ParseElfFileError> {
        if file.len() < mem::size_of::<[u8; 4]>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: 0,
                read_size: mem::size_of::<[u8; 4]>(),
                data_size: file.len(),
            }
            .into());
        }
        if file[..4] == ElfIdent::MAGIC_BYTES {
            return Err(ParseElfFileError::InvalidMagicNumbers(
                *file.first_chunk::<4>().unwrap_or(&[0; 4]),
            ));
        }

        if file.len() < mem::offset_of!(ElfIdent, class) + mem::size_of::<u8>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: 0,
                read_size: mem::size_of::<[u8; 4]>(),
                data_size: file.len(),
            }
            .into());
        }

        let class = C::from_elf_class(file[mem::offset_of!(ElfIdent, class)])?;
        let encoding = E::from_elf_data(file[mem::offset_of!(ElfIdent, data)])?;

        let header_version =
            encoding.parse_u8_at(mem::offset_of!(ElfIdent, header_version), file)?;
        if header_version != ElfIdent::CURRENT_VERSION {
            return Err(ParseElfFileError::InvalidElfHeaderVersion(header_version));
        }

        if file.len() < mem::offset_of!(ElfIdent, _padding) + mem::size_of::<[u8; 7]>() {
            return Err(ParseIntegerError::BoundsError {
                read_offset: mem::offset_of!(ElfIdent, _padding),
                read_size: mem::size_of::<[u8; 7]>(),
                data_size: file.len(),
            }
            .into());
        }
        if file[mem::offset_of!(ElfIdent, _padding)..][..mem::size_of::<[u8; 7]>()]
            .iter()
            .all(|&val| val == 0)
        {
            return Err(ParseElfFileError::NonZeroPadding);
        }

        todo!()
    }
}

/// Various errors that can occur while parsing an [`ElfFile`].
pub enum ParseElfFileError {
    /// The magic numbers differed from [`ELF_MAGIC`].
    InvalidMagicNumbers([u8; 4]),
    /// The code does not support parsing the given class of [`ElfFile`].
    UnsupportedClass(UnsupportedClass),
    /// The code does not suport parsing the given encoding of [`ElfFile`].
    UnsupportedEncoding(UnsupportedEncoding),
    /// The embedded ELF header version does not match [`CURRENT_ELF_HEADER_VERSION`].
    InvalidElfHeaderVersion(u8),
    /// Padding was not all zeros.
    NonZeroPadding,
    /// An error occurred while trying to parse an integer.
    ParseIntegerError(ParseIntegerError),
}

impl From<ParseIntegerError> for ParseElfFileError {
    fn from(value: ParseIntegerError) -> Self {
        Self::ParseIntegerError(value)
    }
}

impl From<UnsupportedClass> for ParseElfFileError {
    fn from(value: UnsupportedClass) -> Self {
        Self::UnsupportedClass(value)
    }
}

impl From<UnsupportedEncoding> for ParseElfFileError {
    fn from(value: UnsupportedEncoding) -> Self {
        Self::UnsupportedEncoding(value)
    }
}

/// Obtains the size of the specfied filed, evaluated at const time.
///
/// This only works for [`Sized`] types.
#[macro_export]
macro_rules! field_size {
    ($t:ident, $field:ident) => {
        const {
            let m = core::mem::MaybeUninit::<$t>::uninit();

            // SAFETY:
            // $t is [`Sized`], and so the project to $field is
            // in bounds.
            let p = unsafe { core::ptr::addr_of!((*m.as_ptr()).$field) };

            const fn size_of_raw<T>(_: *const T) -> usize {
                core::mem::size_of::<T>()
            }

            size_of_raw(p)
        }
    };
}
