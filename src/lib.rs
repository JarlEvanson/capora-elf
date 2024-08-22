//! A simple library providing a pure-safe-rust interface for reading ELF object files.
//!

#![no_std]

use crate::{
    class::ClassParse,
    elf_header::{ElfHeader, ParseElfHeaderError},
    encoding::EncodingParse,
};

pub mod class;
pub mod elf_header;
pub mod elf_ident;
pub mod encoding;
pub mod raw;

/// An ELF file.
pub struct ElfFile<'slice, C: ClassParse, E: EncodingParse> {
    slice: &'slice [u8],
    class: C,
    encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfFile<'slice, C, E> {
    /// Parses an [`ElfFile`] from the provided `file`, checking various invariants
    /// before returning.
    pub fn parse(file: &'slice [u8]) -> Result<Self, ParseElfFileError> {
        let elf_header = ElfHeader::<C, E>::parse(file)?;

        Ok(Self {
            slice: file,
            class: elf_header.elf_ident().class_parse(),
            encoding: elf_header.elf_ident().encoding_parse(),
        })
    }

    pub fn header(&self) -> ElfHeader<'slice, C, E> {
        ElfHeader {
            slice: self.slice,
            class: self.class,
            encoding: self.encoding,
        }
    }
}

/// Various errors that can occur while parsing an [`ElfFile`].
pub enum ParseElfFileError {
    /// An error ocurred while parsing an [`ElfHeader`].
    ParseElfHeaderError(ParseElfHeaderError),
}

impl From<ParseElfHeaderError> for ParseElfFileError {
    fn from(value: ParseElfHeaderError) -> Self {
        Self::ParseElfHeaderError(value)
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
