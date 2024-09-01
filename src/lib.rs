//! A simple library providing a pure-safe-rust interface for reading ELF object files.
//!

#![no_std]

use crate::{
    class::ClassParse,
    elf_header::{ElfHeader, ParseElfHeaderError},
    elf_program_header::{ElfProgramHeaderTable, ParseElfProgramHeaderTableError},
    encoding::EncodingParse,
};

pub mod class;
pub mod elf_header;
pub mod elf_ident;
pub mod elf_program_header;
pub mod encoding;
pub mod raw;

/// An ELF file.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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
        if elf_header.program_header_count() != 0 {
            if (file.len() as u64) < elf_header.program_header_offset() {
                return Err(ParseElfFileError::ParseElfProgramHeaderTableError(
                    ParseElfProgramHeaderTableError::SliceTooSmall,
                ));
            }

            ElfProgramHeaderTable::parse(
                &file[elf_header.program_header_offset() as usize..],
                elf_header.program_header_count() as usize,
                elf_header.program_header_entry_size() as usize,
                elf_header.elf_ident().class_parse(),
                elf_header.elf_ident().encoding_parse(),
            )?;
        }

        Ok(Self {
            slice: file,
            class: elf_header.elf_ident().class_parse(),
            encoding: elf_header.elf_ident().encoding_parse(),
        })
    }

    /// Returns the [`ElfHeader`] of this [`ElfFile`].
    pub fn header(&self) -> ElfHeader<'slice, C, E> {
        ElfHeader {
            slice: self.slice,
            class: self.class,
            encoding: self.encoding,
        }
    }

    /// Returns the [`ElfProgramHeaderTable`] of this [`ElfFile`].
    pub fn program_header_table(&self) -> Option<ElfProgramHeaderTable<'slice, C, E>> {
        if self.header().program_header_count() == 0 {
            return None;
        }

        Some(ElfProgramHeaderTable {
            slice: &self.slice[self.header().program_header_offset() as usize..],
            entry_count: self.header().program_header_count() as usize,
            entry_size: self.header().program_header_entry_size() as usize,
            class: self.class,
            encoding: self.encoding,
        })
    }
}

/// Various errors that can occur while parsing an [`ElfFile`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ParseElfFileError {
    /// An error ocurred while parsing the [`ElfHeader`].
    ParseElfHeaderError(ParseElfHeaderError),
    /// An error ocurred while parsing the [`ElfProgramHeaderTable`].
    ParseElfProgramHeaderTableError(ParseElfProgramHeaderTableError),
}

impl From<ParseElfHeaderError> for ParseElfFileError {
    fn from(value: ParseElfHeaderError) -> Self {
        Self::ParseElfHeaderError(value)
    }
}

impl From<ParseElfProgramHeaderTableError> for ParseElfFileError {
    fn from(value: ParseElfProgramHeaderTableError) -> Self {
        Self::ParseElfProgramHeaderTableError(value)
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
