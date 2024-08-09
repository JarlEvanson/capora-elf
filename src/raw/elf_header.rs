//! Definitions related to parsing the ELF file ident and header.

/// The magic numbers expected in [`ElfIdent::magic`] when parsing.
pub const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];

/// The current version of the ELF file header.
pub const CURRENT_ELF_HEADER_VERSION: u8 = 1;

/// The current verson of the object file format this program supports.
pub const CURRENT_OBJECT_FILE_VERSION: u32 = 1;

/// Block of machine-independent data to mark the file as an ELF file
/// and provide enough information for the remainder of the ELF file to be
/// decoded.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElfIdent {
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
}

/// 32-bit version of the ELF file header.
///
/// This allows for determining the layout and target that the ELF
/// file supports.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32Header {
    /// Machine independent data determine how to interpret the remainder
    /// of the file.
    pub ident: ElfIdent,

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
    pub entry: u32,
    /// The program header table's file offset in bytes.
    ///
    /// If the ELF file has no program header table, this member should hold
    /// zero.
    pub program_header_offset: u32,
    /// The section header table's file offset in bytes.
    ///
    /// If the ELF file has no section header table, this member should hold
    /// zero.
    pub section_heaer_offset: u32,
    /// Processor specific flags.
    pub flags: u32,
    /// The size of the ELF file's header in bytes.
    pub elf_header_size: u16,
    /// The size, in bytes, of one entry in the program header table.
    pub program_header_entry_size: u16,
    /// The number of entries in the program header table.
    ///
    /// If a file has no program header table, this member should hold zero.
    pub program_header_count: u16,
    /// The size, in bytes, of one entry in the section header table.
    pub section_header_entry_size: u16,
    /// The number of entries in the section header table.
    ///
    /// If a file has no section header table, this member should hold zero.
    pub section_header_count: u16,
    /// The section header table index of the entry associated with the section
    /// name string table.
    ///
    /// If the section name string is greater than or equal to 0xFF00 this member
    /// has the value 0xFFFF and the actual index of the section name string table
    /// is contained in the sh_link file of the secton header at index 0.
    pub section_header_string_table_index: u16,
}

/// 64-bit version of the ELF file header.
///
/// This allows for determining the layout and target that the ELF
/// file supports.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64Header {
    /// Machine independent data determine how to interpret the remainder
    /// of the file.
    pub ident: ElfIdent,

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
    pub elf_header_size: u16,
    /// The size, in bytes, of one entry in the program header table.
    pub program_header_entry_size: u16,
    /// The number of entries in the program header table.
    ///
    /// If a file has no program header table, this member should hold zero.
    pub program_header_count: u16,
    /// The size, in bytes, of one entry in the section header table.
    pub section_header_entry_size: u16,
    /// The number of entries in the section header table.
    ///
    /// If a file has no section header table, this member should hold zero.
    pub section_header_count: u16,
    /// The section header table index of the entry associated with the section
    /// name string table.
    ///
    /// If the section name string is greater than or equal to 0xFF00 this member
    /// has the value 0xFFFF and the actual index of the section name string table
    /// is contained in the sh_link file of the secton header at index 0.
    pub section_header_string_table_index: u16,
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
