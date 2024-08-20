//! Definitions related to ELF section headers.

/// 32-bit version of an ELF section header.
///
/// This allows for locating and interacting with data relevant for linking object files.
pub struct Elf32SectionHeader {
    /// The index into the section name string table that identifies the name of the section.
    pub name: u32,
    /// The kind of the section.
    pub kind: u32,
    /// Additional information about a section.
    pub flags: u32,
    /// The virtual address of the section at execution.
    pub address: u32,
    /// The offset of the section within the ELF file.
    pub offset: u32,
    /// THe size of the section in bytes.
    pub size: u32,
    /// A link to another section containing data relevent for decoding the contents of this
    /// section.
    pub link: u32,
    /// Additional information about the section.
    pub info: u32,
    /// The alignment of the section.
    pub address_align: u32,
    /// The size of an entry contained in the section if the section holds a table of etnries.
    pub entry_size: u32,
}

/// 32-bit version of an ELF section header.
///
/// This allows for locating and interacting with data relevant for linking object files.
pub struct Elf64SectionHeader {
    /// The index into the section name string table that identifies the name of the section.
    pub name: u32,
    /// The kind of the section.
    pub kind: u32,
    /// Additional information about a section.
    pub flags: u64,
    /// The virtual address of the section at execution.
    pub address: u64,
    /// The offset of the section within the ELF file.
    pub offset: u64,
    /// THe size of the section in bytes.
    pub size: u64,
    /// A link to another section containing data relevent for decoding the contents of this
    /// section.
    pub link: u32,
    /// Additional information about the section.
    pub info: u32,
    /// The alignment of the section.
    pub address_align: u64,
    /// The size of an entry contained in the section if the section holds a table of etnries.
    pub entry_size: u64,
}
