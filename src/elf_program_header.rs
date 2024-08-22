//! Definitions and interfaces for interacting with an ELF program header.

use core::mem;

use crate::{
    class::{Class, ClassParse},
    encoding::EncodingParse,
    raw::elf_program_header::{Elf64ProgramHeader, SegmentFlags, SegmentType},
};

/// Structure that describes how to locate and load data and configuration relevant to program
/// execution.
#[derive(Debug)]
pub struct ElfProgramHeader<'slice, C: ClassParse, E: EncodingParse> {
    pub(crate) slice: &'slice [u8],
    pub(crate) class: C,
    pub(crate) encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfProgramHeader<'slice, C, E> {
    /// Parses an [`ElfProgramHeader`] from the provided `file`.
    pub fn parse(
        file: &'slice [u8],
        offset: usize,
        class: C,
        encoding: E,
    ) -> Result<Self, ParseElfProgramHeaderError> {
        match class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => {
                if offset
                    .checked_add(mem::size_of::<Elf64ProgramHeader>())
                    .ok_or(ParseElfProgramHeaderError::FileTooSmall)?
                    >= file.len()
                {
                    return Err(ParseElfProgramHeaderError::FileTooSmall);
                }

                let elf_program_header = Self {
                    slice: &file[offset..],
                    class,
                    encoding,
                };

                if !elf_program_header.alignment().is_power_of_two() {
                    return Err(ParseElfProgramHeaderError::InvalidAlignment);
                }

                if elf_program_header.alignment() != 0
                    && elf_program_header.virtual_address() % elf_program_header.alignment()
                        != elf_program_header.file_offset() % elf_program_header.alignment()
                {
                    return Err(ParseElfProgramHeaderError::UnalignedSegment);
                }

                Ok(elf_program_header)
            }
        }
    }

    /// Returns the [`SegmentType`], which determines how to interpret the [`ElfProgramHeader`]'s
    /// information.
    pub fn segment_type(&self) -> SegmentType {
        let segment_type_value = match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf64ProgramHeader, r#type), self.slice),
        };

        SegmentType(segment_type_value)
    }

    /// Returns various flags relevant to the segment.
    pub fn flags(&self) -> SegmentFlags {
        let flags_value = match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self
                .encoding
                .parse_u32_at(mem::offset_of!(Elf64ProgramHeader, flags), self.slice),
        };

        SegmentFlags(flags_value)
    }

    /// Returns the offset from the beginning of the file at which the first byte of the segment
    /// exists.
    pub fn file_offset(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self
                .encoding
                .parse_u64_at(mem::offset_of!(Elf64ProgramHeader, file_offset), self.slice),
        }
    }

    /// Returns the virtual address at which the first byte of the segment resides in memory when
    /// loaded.
    pub fn virtual_address(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self.encoding.parse_u64_at(
                mem::offset_of!(Elf64ProgramHeader, virtual_address),
                self.slice,
            ),
        }
    }

    /// On systems for which physical addressing is relevant, this member is reserved for the
    /// segment's physical address.
    pub fn physical_address(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self.encoding.parse_u64_at(
                mem::offset_of!(Elf64ProgramHeader, physical_address),
                self.slice,
            ),
        }
    }

    /// Returns the number of bytes in the file image of the segment.
    ///
    /// This may be zero.
    pub fn file_size(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self
                .encoding
                .parse_u64_at(mem::offset_of!(Elf64ProgramHeader, file_size), self.slice),
        }
    }

    /// Returns the number of bytes in the memory image of the segment.
    ///
    /// This may be zero.
    pub fn memory_size(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self
                .encoding
                .parse_u64_at(mem::offset_of!(Elf64ProgramHeader, memory_size), self.slice),
        }
    }

    /// Returns the alignment of the segment referenced by this [`ElfProgramHeader`].
    ///
    /// This alignment is applicable both in the file and in memory.
    pub fn alignment(&self) -> u64 {
        match self.class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => self
                .encoding
                .parse_u64_at(mem::offset_of!(Elf64ProgramHeader, alignment), self.slice),
        }
    }
}

/// Various errors that can occur while parsing an [`ElfProgramHeader`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ParseElfProgramHeaderError {
    /// The given file was too small to contain an [`ElfProgramHeader`] at offset `offset`.
    FileTooSmall,
    /// The alignment of the segment is not a power of two.
    InvalidAlignment,
    /// The segment pointed to by the [`ElfProgramHeader`] is not properly aligned.
    UnalignedSegment,
}
