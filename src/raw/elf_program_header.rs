//! Definitions related to ELF program headers.

/// 32-bit version of an ELF program header.
///
/// This allows for locating and loading data relevant to program
/// execution.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32ProgramHeader {
    /// The type of the segment.
    pub r#type: SegmentType,
    /// The offset from the start of the ELF file to the segment data.
    pub file_offset: u32,
    /// The virtual address of the segment.
    pub virtual_address: u32,
    /// The physical address of the segment.
    pub physical_address: u32,
    /// The size of the segment in the ELF file.
    pub file_size: u32,
    /// The size of the segment in memory.
    pub memory_size: u32,
    /// Defines the permissions of the the segment in memory.
    pub flags: SegmentFlags,
    /// The alignment of the segment in memory.
    pub alignment: u32,
}

/// 64-bit version of an ELF program header.
///
/// This allows for locating and loading data relevant to program
/// execution.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64ProgramHeader {
    /// The type of the segment.
    pub r#type: SegmentType,
    /// Defines the permissions of the the segment in memory.
    pub flags: SegmentFlags,
    /// The offset from the start of the ELF file to the segment data.
    pub file_offset: u64,
    /// The virtual address of the segment.
    pub virtual_address: u64,
    /// The physical address of the segment.
    pub physical_address: u64,
    /// The size of the segment in the ELF file.
    pub file_size: u64,
    /// The size of the segment in memory.
    pub memory_size: u64,
    /// The alignment of the segment in memory.
    pub alignment: u64,
}

/// The type of the segment.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentType(pub u32);

impl SegmentType {
    /// Unused program header.
    pub const NULL: Self = Self(0);
    /// Loadable  program segment.
    pub const LOAD: Self = Self(1);
    /// Dynamic linking information.
    pub const DYNAMIC: Self = Self(2);
    /// The program interpreter.
    pub const INTERP: Self = Self(3);
    /// Auxiliary information.
    pub const NOTE: Self = Self(4);
    /// Reserved.
    pub const SHLIB: Self = Self(5);
    /// Program header table.
    pub const PHDR: Self = Self(6);
    /// Thread local storage.
    pub const TLS: Self = Self(7);
}

/// The permissions of the loaded segment.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentFlags(pub u32);

impl SegmentFlags {
    /// The segment is executable.
    pub const EXECUTE: Self = Self(1);
    /// The segment is writable.
    pub const WRITE: Self = Self(2);
    /// The segment is readable.
    pub const READ: Self = Self(4);
}
