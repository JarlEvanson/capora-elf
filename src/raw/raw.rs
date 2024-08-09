//! Definitions and interfaces for loading an ELF file.

/// The current version of the ELF file header.
pub const CURRENT_ELF_HEADER_VERSION: u8 = 1;

/// The current verson of the object file format this program supports.
pub const CURRENT_OBJECT_FILE_VERSION: u32 = 1;

/// Header of the ELF file format.
///
/// This allows for determining the layout and target that the ELF
/// file supports.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64Header {
    /// Holds magic numbers to identify the file as an ELF file.
    pub magic: [u8; 4],
    /// The file's class (native word size).
    pub class: u8,
    /// Encoding of data structures used by the object file container
    /// and data contained in object file sections.
    pub data: u8,
    /// The ELF header.
    pub header_version: u8,
    /// Identifies the OS or ABI specific extensions used by this file.
    pub os_abi: u8,
    /// The version of the ABI to which the object file is targeted.
    ///
    /// This should be zero if the [`Elf64Header::os_abi`] field has no
    /// definitions or no version values in the processor supplement.
    pub abi_version: u8,
    /// Unused bytes, should all be zero.
    pub _padding: [u8; 7],

    /// Identifies the object file type.
    pub r#type: ElfType,
    /// The required architecture of the ELF file.
    pub machine: Machine,
    /// The object file version of this file.
    pub object_file_version: u32,
    /// The virtual address to which the system should transfer
    /// control.
    ///
    /// If the ELF file has no associated entry point, this member should
    /// hold zero.
    pub entry: u64,
    /// The program header table's file offset in bytes.
    ///
    /// If the ELF file has no program header table, this member should hold
    /// zero.
    pub program_header_offset: u64,
    /// The section header table's file offset in bytes.
    ///
    /// If the ELF file has no section header table, this member should hold
    /// zero.
    pub section_heaer_offset: u64,
    /// Processor specific flags.
    pub flags: u32,
    /// The size of the ELF file's header in bytes.
    pub elf_header_size: u32,
    /// The size, in bytes, of one entry in the program header table.
    pub program_header_entry_size: u32,
    /// The number of entries in the program header table.
    ///
    /// If a file has no program header table, this member should hold zero.
    pub program_header_count: u32,
    /// The size, in bytes, of one entry in the section header table.
    pub section_header_entry_size: u32,
    /// The number of entries in the section header table.
    ///
    /// If a file has no section header table, this member should hold zero.
    pub section_header_count: u32,
    /// The section header table index of the entry associated with the section
    /// name string table.
    ///
    /// If the section name string is greater than or equal to 0xFF00 this member
    /// has the value 0xFFFF and the actual index of the section name string table
    /// is contained in the sh_link file of the secton header at index 0.
    pub section_header_string_table_index: u32,
}

/// The type of the ELF file.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElfType(u16);

impl ElfType {
    /// No type.
    pub const NONE: Self = Self(0);
    /// Relocatable ELF file.
    pub const RELOCATABLE: Self = Self(1);
    /// Executable ELF file.
    pub const EXECUTABLE: Self = Self(2);
    /// Shared object ELF file.
    pub const SHARED: Self = Self(3);
    /// Core ELF file.
    pub const CORE: Self = Self(4);
}

/// The required architecture of the ELF file.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Machine(u16);

impl Machine {
    /// No required machine.
    pub const NONE: Self = Self(0);
    /// ELF file requires the AMD x86_64 architecture.
    pub const X86_64: Self = Self(62);
}

/// Header for a segment.
///
/// This descibes a segment or other information the system needs to prepare
/// for execution. This header is only meaningful for executable and shared
/// object files.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64ProgramHeader {
    /// Identifies the type of the segment.
    pub r#type: ProgramHeaderType,
    /// Various flags that are dependent on the [`Elf64ProgramHeader::r#type`].
    pub flags: ProgramHeaderFlags,
    /// The offset from the beginning of the file at which
    /// the first byte of the segment resides.
    pub offset: u64,
    /// The virtual address at which the first byte of the
    /// segment resides in memory.
    pub virtual_address: u64,
    /// The physical address at which the first byte of the
    /// segment resides in memory.
    ///
    /// If physical addressing is not relevant, then this
    /// member has unspecified contents for executable files
    /// and shared objects.
    pub physical_address: u64,
    /// The number of bytes in the file image of the segment.
    ///
    /// This may be zero.
    pub file_size: u64,
    /// The number of bytes in the memory image of the segment.
    ///
    /// This may be zero.
    pub memory_size: u64,
    /// The value to which this segment is aligned in memory and
    /// in the file.
    ///
    /// This should be a positive, integral power of two.
    pub alignment: u64,
}

/// The type of the [`Elf64ProgramHeader`], which determines its information should be 
/// interpreted.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramHeaderType(u32);

impl ProgramHeaderType {
    /// The [`Elf64ProgramHeader`] is unused.
    ///
    /// All other member's values are undefined.
    pub const NULL: Self = Self(0);
    /// The [`Elf64ProgramHeader`] defines a loadable segment.
    ///
    /// The bytes from the file are mapped to the beginning of 
    /// the memory segment. If [`Elf64ProgramHeader::memory_size`] is larger than 
    /// [`Elf64ProgramHeader::file_size`], the  "extra" bytes are defined to hold the
    /// value 0 and to immediately follow the segment's initialized area.
    ///
    /// [`Elf64ProgramHeader::file_size`] may not be larger than
    /// [`Elf64ProgramHeader::memory_size`].
    ///
    /// All [`ProgramHeaderType::LOAD`] [`Elf64ProgramHeader`]s appear in ascending 
    /// order, sorted on [`Elf64ProgramHeader::virtual_address`].
    pub const LOAD: Self = Self(1);
    /// The [`Elf64ProgramHeader`] specifies dynamic linking information.
    pub const DYNAMIC: Self = Self(2);
    /// The [`Elf64ProgramHeader`] specificies the location and size of a null-terminated
    /// path name to invoke as an interpreter.
    ///
    /// This [`ProgramHeaderType`] is only meaningful for [`ElfType::EXECUTABLE`] files,
    /// though it may appear in [`ElfType::SHARED`] files. It may not appear more than once
    /// in a file. If it is present, it must precede any [`ProgramHeaderType::LOAD`]
    /// [`Elf64ProgramHeader`]
    pub const INTERP: Self = Self(3);
    /// The [`Elf64ProgramHeader`] specifies the location and size of auxiliary information.
    pub const NOTE: Self = Self(4);
    /// This [`ProgramHeaderType`] is reserved but has unspecified sementics.
    ///
    /// All programs containing an [`Elf64ProgramHeader`] of this type are invalid.
    pub const SHLIB: Self = Self(5);
    /// This [`Elf64ProgramHeader`] specifies the location and size of the [`Elf64ProgramHeader`]
    /// table itself, both in the file and in the memory image of the program. 
    ///
    /// This [`ProgramHeaderType`] may not occur more than once in a file, and it may only
    /// occur only if the [`Elf64ProgramHeader`] table is part of the memory image of the 
    /// program. If present, it must precede any [`ProgramHeaderType::LOAD`]
    /// [`Elf64ProgramHeader`].
    pub const PROGRAM_HEADER: Self = Self(6);
    /// This [`Elf64ProgramHeader`] specifies the thread-local storage template.
    ///
    /// Implementations need not support this [`ProgramHeaderType`].
    pub const TLS: Self = Self(7);
}


/// The flags relevant to the segment.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramHeaderFlags(u32);

impl ProgramHeaderFlags {
    /// The section should be executable.
    pub const EXECUTE: Self = Self(0x1);
    /// The section should be writable.
    pub const WRITE: Self = Self(0x2);
    /// The section should be readable.
    pub const READ: Self = Self(0x4);
}
