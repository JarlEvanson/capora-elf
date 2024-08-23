//! Definitions and interfaces for interacting with an ELF program header.

use core::{fmt, mem};

use crate::{
    class::{Class, ClassParse},
    encoding::EncodingParse,
    raw::elf_program_header::{Elf64ProgramHeader, SegmentFlags, SegmentType},
};

/// Structure that describes how to locate and load data and configuration relevant to program
/// execution.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ElfProgramHeader<'slice, C: ClassParse, E: EncodingParse> {
    pub(crate) slice: &'slice [u8],
    pub(crate) class: C,
    pub(crate) encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfProgramHeader<'slice, C, E> {
    /// Parses an [`ElfProgramHeader`] from the provided `slice`.
    pub fn parse(
        slice: &'slice [u8],
        class: C,
        encoding: E,
    ) -> Result<Self, ParseElfProgramHeaderError> {
        match class.into_class() {
            Class::Class32 => todo!(),
            Class::Class64 => {
                if slice.len() < mem::size_of::<Elf64ProgramHeader>() {
                    return Err(ParseElfProgramHeaderError::SliceTooSmall);
                }

                let elf_program_header = Self {
                    slice,
                    class,
                    encoding,
                };

                if !elf_program_header.alignment().is_power_of_two()
                    && elf_program_header.alignment() != 0
                {
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

impl<'slice, C: ClassParse, E: EncodingParse> fmt::Debug for ElfProgramHeader<'slice, C, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ElfProgramHeader");

        debug_struct.field("segment_type", &self.segment_type());
        debug_struct.field("segment_flags", &self.flags());
        debug_struct.field("file_offset", &self.file_offset());
        debug_struct.field("virtual_address", &self.virtual_address());
        debug_struct.field("physical_address", &self.physical_address());
        debug_struct.field("file_size", &self.file_size());
        debug_struct.field("memory_size", &self.memory_size());
        debug_struct.field("alignment", &self.alignment());

        debug_struct.finish()
    }
}

/// Various errors that can occur while parsing an [`ElfProgramHeader`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ParseElfProgramHeaderError {
    /// The given slice was too small to contain an [`ElfProgramHeader`].
    SliceTooSmall,
    /// The alignment of the segment is not a power of two.
    InvalidAlignment,
    /// The segment pointed to by the [`ElfProgramHeader`] is not properly aligned.
    UnalignedSegment,
}

/// A table of [`ElfProgramHeader`]s.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ElfProgramHeaderTable<'slice, C: ClassParse, E: EncodingParse> {
    pub(crate) slice: &'slice [u8],
    pub(crate) entry_count: usize,
    pub(crate) entry_size: usize,
    pub(crate) class: C,
    pub(crate) encoding: E,
}

impl<'slice, C: ClassParse, E: EncodingParse> ElfProgramHeaderTable<'slice, C, E> {
    /// Parses an [`ElfProgramHeaderTable`] from the provided `slice`.
    pub fn parse(
        slice: &'slice [u8],
        entry_count: usize,
        entry_size: usize,
        class: C,
        encoding: E,
    ) -> Result<Self, ParseElfProgramHeaderTableError> {
        let total_size = entry_count
            .checked_mul(entry_size)
            .ok_or(ParseElfProgramHeaderTableError::SliceTooSmall)?;
        if slice.len() < total_size {
            return Err(ParseElfProgramHeaderTableError::SliceTooSmall);
        }

        let elf_program_header_table = Self {
            slice,
            entry_count,
            entry_size,
            class,
            encoding,
        };

        for index in 0..entry_count {
            ElfProgramHeader::parse(&slice[index * entry_size..], class, encoding).map_err(
                |error| ParseElfProgramHeaderTableError::ParseElfProgramHeaderError {
                    index,
                    error,
                },
            )?;
        }

        Ok(elf_program_header_table)
    }

    /// Returns the [`ElfProgramHeader`] located at `index`.
    pub fn get(&self, index: usize) -> Option<ElfProgramHeader<'slice, C, E>> {
        if index >= self.entry_count {
            return None;
        }

        Some(ElfProgramHeader {
            slice: &self.slice[index * self.entry_size..],
            class: self.class,
            encoding: self.encoding,
        })
    }
}

impl<'slice, C: ClassParse, E: EncodingParse> fmt::Debug for ElfProgramHeaderTable<'slice, C, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();

        for i in 0..self.entry_count {
            debug_list.entry(&self.get(i).unwrap());
        }

        debug_list.finish()
    }
}

/// Various errors that can occur while parsing an [`ElfProgramHeaderTable`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ParseElfProgramHeaderTableError {
    /// The given slice was too small to contain the specified [`ElfProgramHeaderTable`].
    SliceTooSmall,
    /// An error occurred while parsing the [`ElfProgramHeader`] at `index`.
    ParseElfProgramHeaderError {
        /// The index of the [`ElfProgramHeader`] that parsing failed on.
        index: usize,
        /// The error that was returned.
        error: ParseElfProgramHeaderError,
    },
}
