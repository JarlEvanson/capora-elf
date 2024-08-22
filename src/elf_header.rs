//! Definitions and interfaces for interacting with the ELF file header.

use core::mem;

use crate::{
    class::{Class, ClassParse},
    elf_ident::{ElfIdent, ParseElfIdentError},
    encoding::EncodingParse,
    raw::{
        elf_header::{Elf32Header, Elf64Header, ElfType, Machine, CURRENT_OBJECT_FILE_VERSION},
        elf_program_header::Elf64ProgramHeader,
        elf_section_header::Elf64SectionHeader,
    },
};

/// The header of an ELF file, which contains important information about the layout and
/// interpretation of the ELF file.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ElfHeader<'slice, C: ClassParse, E: EncodingParse> {
    pub(crate) slice: &'slice [u8],
    pub(crate) class: C,
    pub(crate) encoding: E,
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
                    return Err(ParseElfHeaderError::FileTooSmall);
                }

                if elf_ident
                    .encoding_parse()
                    .parse_u32_at(mem::offset_of!(Elf64Header, object_file_version), file)
                    != CURRENT_OBJECT_FILE_VERSION
                {
                    return Err(ParseElfHeaderError::UnsupportedElfFileVersion);
                }

                let elf_header_size = elf_ident.encoding_parse().parse_u16_at(mem::offset_of!(Elf64Header, elf_header_size), file);
                if (elf_header_size as usize) < mem::size_of::<Elf64Header>() {
                    return Err(ParseElfHeaderError::InvalidElfHeaderSize);
                }

                let program_header_entry_size = elf_ident.encoding_parse().parse_u16_at(
                    mem::offset_of!(Elf64Header, program_header_entry_size),
                    file,
                );
                if (program_header_entry_size as usize) < mem::size_of::<Elf64ProgramHeader>() {
                    return Err(ParseElfHeaderError::InvalidProgramHeaderSize);
                }

                let section_header_entry_size = elf_ident.encoding_parse().parse_u16_at(
                    mem::offset_of!(Elf64Header, section_header_entry_size),
                    file,
                );
                if (section_header_entry_size as usize) < mem::size_of::<Elf64SectionHeader>() {
                    return Err(ParseElfHeaderError::InvalidSectionHeaderSize);
                }
            }
        }

        Ok(Self {
            slice: file,
            class: elf_ident.class_parse(),
            encoding: elf_ident.encoding_parse(),
        })
    }

    /// Returns the [`ElfIdent`] this [`ElfHeader`] contains.
    pub fn elf_ident(&self) -> ElfIdent<'slice, C, E> {
        ElfIdent {
            slice: self.slice,
            class: self.class,
            encoding: self.encoding,
        }
    }

    /// The type of the ELF file.
    pub fn elf_type(&self) -> ElfType {
        let elf_type_value = match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u16_at(mem::offset_of!(Elf32Header, r#type), self.slice),
            Class::Class64 => self
                .encoding
                .parse_u16_at(mem::offset_of!(Elf64Header, r#type), self.slice),
        };

        ElfType(elf_type_value)
    }

    /// The machine architecture that this object file is targeted towards.
    pub fn machine(&self) -> Machine {
        let machine_value = match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u16_at(mem::offset_of!(Elf32Header, machine), self.slice),
            Class::Class64 => self
                .encoding
                .parse_u16_at(mem::offset_of!(Elf64Header, machine), self.slice),
        };

        Machine(machine_value)
    }

    /// Returns the version of the ELF header.
    pub fn object_file_version(&self) -> u32 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u32_at(
                mem::offset_of!(Elf32Header, object_file_version),
                self.slice,
            ),
            Class::Class64 => self.encoding.parse_u32_at(
                mem::offset_of!(Elf64Header, object_file_version),
                self.slice,
            ),
        }
    }

    /// Returns the virtual address to which the system first transfers control.
    pub fn entry(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf32Header, entry), self.slice)
                as u64,
            Class::Class64 => self
                .encoding
                .parse_u64_at(mem::offset_of!(Elf64Header, entry), self.slice),
        }
    }

    /// Returns the offset, in bytes, from the start of the file to the start of the program header
    /// table.
    pub fn program_header_offset(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u32_at(
                mem::offset_of!(Elf32Header, program_header_offset),
                self.slice,
            ) as u64,
            Class::Class64 => self.encoding.parse_u64_at(
                mem::offset_of!(Elf64Header, program_header_offset),
                self.slice,
            ),
        }
    }

    /// Returns the offset, in bytes, from the start of the file to the start of the section header
    /// table.
    pub fn section_header_offset(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u32_at(
                mem::offset_of!(Elf32Header, section_header_offset),
                self.slice,
            ) as u64,
            Class::Class64 => self.encoding.parse_u64_at(
                mem::offset_of!(Elf64Header, section_header_offset),
                self.slice,
            ),
        }
    }

    /// Returns the processor-specific flags associated with the ELF file.
    pub fn flags(&self) -> u32 {
        match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf32Header, flags), self.slice),
            Class::Class64 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf64Header, flags), self.slice),
        }
    }

    /// Returns the size of the elf header.
    pub fn elf_header_size(&self) -> u16 {
        match self.class.into_class() {
            Class::Class32 => self
                .encoding
                .parse_u16_at(mem::offset_of!(Elf32Header, elf_header_size), self.slice),
            Class::Class64 => self
                .encoding
                .parse_u16_at(mem::offset_of!(Elf64Header, elf_header_size), self.slice),
        }
    }

    /// Returns the size of the program headers this ELF file contains.
    pub fn program_header_entry_size(&self) -> u16 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf32Header, program_header_entry_size),
                self.slice,
            ),
            Class::Class64 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf64Header, program_header_entry_size),
                self.slice,
            ),
        }
    }

    /// Returns the number of program headers this ELF file contains.
    pub fn program_header_count(&self) -> u16 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf32Header, program_header_count),
                self.slice,
            ),
            Class::Class64 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf64Header, program_header_count),
                self.slice,
            ),
        }
    }

    /// Returns the size of the program headers this ELF file contains.
    pub fn section_header_entry_size(&self) -> u16 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf32Header, section_header_entry_size),
                self.slice,
            ),
            Class::Class64 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf64Header, section_header_entry_size),
                self.slice,
            ),
        }
    }

    /// Returns the number of section headers this ELF file contains.
    pub fn section_header_count(&self) -> u16 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf32Header, section_header_count),
                self.slice,
            ),
            Class::Class64 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf64Header, section_header_count),
                self.slice,
            ),
        }
    }

    /// Returns the section header index of the string table for section names.
    pub fn section_header_string_table_index(&self) -> u16 {
        match self.class.into_class() {
            Class::Class32 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf32Header, section_header_string_table_index),
                self.slice,
            ),
            Class::Class64 => self.encoding.parse_u16_at(
                mem::offset_of!(Elf64Header, section_header_string_table_index),
                self.slice,
            ),
        }
    }
}

/// Various errors that can occur while parsing an [`ElfHeader`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ParseElfHeaderError {
    /// An error occurred while parsing the associated [`ElfIdent`].
    ParseElfIdentError(ParseElfIdentError),
    /// The given `file` was too small to contain an [`ElfHeader`].
    FileTooSmall,
    /// The version of the ELF file is unsupported.
    UnsupportedElfFileVersion,
    /// The given size of the [`ElfHeader`] is smaller than supported.
    InvalidElfHeaderSize,
    /// The given size of [`ElfProgramHeader`]s is smaller than supported.
    InvalidProgramHeaderSize,
    /// The given size of [`ElfSectionHeader`]s is smaller than supported.
    InvalidSectionHeaderSize,
}

impl From<ParseElfIdentError> for ParseElfHeaderError {
    fn from(value: ParseElfIdentError) -> Self {
        Self::ParseElfIdentError(value)
    }
}
