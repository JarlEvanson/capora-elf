//! Definitions and interfaces for interacting with the ELF file header.

use core::mem;

use crate::{
    class::{Class, ClassParse},
    elf_ident::{ElfIdent, ParseElfIdentError},
    encoding::{EncodingParse, ParseIntegerError},
    raw::elf_header::{Elf32Header, Elf64Header, CURRENT_OBJECT_FILE_VERSION},
};

/// The header of an ELF file, which contains important information about the layout and
/// interpretation of the ELF file.
pub struct ElfHeader<'slice, C: ClassParse, E: EncodingParse> {
    slice: &'slice [u8],
    class: C,
    encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfHeader<'slice, C, E> {
    /// Parses an [`ElfHeader`] from the provided `file`, checking as many invariants
    /// as possible.
    pub fn parse(file: &'slice [u8]) -> Result<Self, ParseElfHeaderError> {
        let elf_ident = ElfIdent::<C, E>::parse(file)?;

        match elf_ident.class_parse().into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => {
                if file.len() < mem::size_of::<Elf64Header>() {
                    return Err(ParseIntegerError::BoundsError {
                        read_offset: 0,
                        read_size: mem::size_of::<Elf64Header>(),
                        data_size: file.len(),
                    }
                    .into());
                }

                if elf_ident
                    .encoding_parse()
                    .parse_u32_at(mem::offset_of!(Elf64Header, object_file_version), file)?
                    != CURRENT_OBJECT_FILE_VERSION
                {
                    return Err(ParseElfHeaderError::UnsupportedElfFileVersion);
                }

                let program_header_offset = elf_ident
                    .encoding_parse()
                    .parse_u64_at(mem::offset_of!(Elf64Header, program_header_offset), file)?
                    as u64;
                let program_header_count = elf_ident
                    .encoding_parse()
                    .parse_u16_at(mem::offset_of!(Elf64Header, program_header_count), file)?
                    as u64;
                let program_header_entry_size = elf_ident.encoding_parse().parse_u16_at(
                    mem::offset_of!(Elf64Header, program_header_entry_size),
                    file,
                )? as u64;

                let program_header_table_size = program_header_count
                    .checked_mul(program_header_entry_size)
                    .ok_or(ParseIntegerError::ArithmeticError)?;
                let program_header_table_max_offset = program_header_offset
                    .checked_add(program_header_table_size)
                    .ok_or(ParseIntegerError::ArithmeticError)?;

                if file.len()
                    < program_header_table_max_offset
                        .try_into()
                        .map_err(|_| ParseIntegerError::ArithmeticError)?
                {
                    return Err(ParseIntegerError::BoundsError {
                        read_offset: program_header_offset as usize,
                        read_size: program_header_table_size as usize,
                        data_size: file.len(),
                    }
                    .into());
                }

                let section_header_offset = elf_ident
                    .encoding_parse()
                    .parse_u64_at(mem::offset_of!(Elf64Header, section_header_offset), file)?;
                let section_header_count = elf_ident
                    .encoding_parse()
                    .parse_u16_at(mem::offset_of!(Elf64Header, section_header_offset), file)?
                    as u64;
                let section_header_entry_size = elf_ident.encoding_parse().parse_u16_at(
                    mem::offset_of!(Elf64Header, section_header_entry_size),
                    file,
                )? as u64;

                let section_header_table_size = section_header_count
                    .checked_mul(section_header_entry_size)
                    .ok_or(ParseIntegerError::ArithmeticError)?;
                let section_header_table_max_offset = section_header_offset
                    .checked_add(section_header_table_size)
                    .ok_or(ParseIntegerError::ArithmeticError)?;

                if file.len()
                    < section_header_table_max_offset
                        .try_into()
                        .map_err(|_| ParseIntegerError::ArithmeticError)?
                {
                    return Err(ParseIntegerError::BoundsError {
                        read_offset: section_header_offset as usize,
                        read_size: section_header_table_size as usize,
                        data_size: file.len(),
                    }
                    .into());
                }
            }
        }

        Ok(Self {
            slice: file,
            class: elf_ident.class_parse(),
            encoding: elf_ident.encoding_parse(),
        })
    }

    /// Returns the virtual address to which the system first transfers control.
    pub fn entry(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf32Header, entry), self.slice)
                .unwrap() as u64,
            Class::Class64 => self
                .encoding
                .parse_u64_at(mem::offset_of!(Elf64Header, entry), self.slice)
                .unwrap(),
        }
    }

    /// Returns the processor-specific flags associated with the ELF file.
    pub fn flags(&self) -> u32 {
        match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf32Header, flags), self.slice)
                .unwrap(),
            Class::Class64 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf64Header, flags), self.slice)
                .unwrap(),
        }
    }

    /// Returns the offset, in bytes, from the start of the file to the start of the program header
    /// table.
    pub fn program_header_offset(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u32_at(
                    mem::offset_of!(Elf32Header, program_header_offset),
                    self.slice,
                )
                .unwrap() as u64,
            Class::Class64 => self
                .encoding
                .parse_u64_at(
                    mem::offset_of!(Elf64Header, program_header_offset),
                    self.slice,
                )
                .unwrap(),
        }
    }

    /// Returns the offset, in bytes, from the start of the file to the start of the section header
    /// table.
    pub fn section_header_offset(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u32_at(
                    mem::offset_of!(Elf32Header, section_header_offset),
                    self.slice,
                )
                .unwrap() as u64,
            Class::Class64 => self
                .encoding
                .parse_u64_at(
                    mem::offset_of!(Elf64Header, section_header_offset),
                    self.slice,
                )
                .unwrap(),
        }
    }
}

/// Various errors that can occur while parsing an [`ElfHeader`].
pub enum ParseElfHeaderError {
    /// An error occurred while parsing the associated [`ElfIdent`].
    ParseElfIdentError(ParseElfIdentError),
    /// The version of the ELF file is unsupported.
    UnsupportedElfFileVersion,
    /// An error ocurred while parsing an integer.
    ParseIntegerError(ParseIntegerError),
}

impl From<ParseElfIdentError> for ParseElfHeaderError {
    fn from(value: ParseElfIdentError) -> Self {
        Self::ParseElfIdentError(value)
    }
}

impl From<ParseIntegerError> for ParseElfHeaderError {
    fn from(value: ParseIntegerError) -> Self {
        Self::ParseIntegerError(value)
    }
}
