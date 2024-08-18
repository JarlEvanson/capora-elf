//! Definitions related to ELF relocations.

/// 32-bit version of an ELF relocation entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32Rel {
    /// The offset to the location that requires relocating.
    ///
    /// For [`ElfType::EXECUTABLE`][ete] binaries, this value denotes an offset
    /// within a section header. For [`ElfType::RELOCATABLE`][etr] binaries, this
    /// value denotes a virtual address affected by a relocation.
    pub offset: u32,
    /// Gives the symbol table index and the type of relocation to display.
    pub info: u32,
}

/// 32-bit version of an ELF relocation with addend entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32Rela {
    /// The offset to the location that requires relocating.
    ///
    /// For [`ElfType::EXECUTABLE`][ete] binaries, this value denotes an offset
    /// within a section header. For [`ElfType::RELOCATABLE`][etr] binaries, this
    /// value denotes a virtual address affected by a relocation.
    pub offset: u32,
    /// Gives the symbol table index and the type of relocation to display.
    pub info: u32,
    /// A constant addend used to compute the value stored in the relocatable
    /// field.
    pub addend: i32,
}

/// 64-bit version of an ELF relocation entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64Rel {
    /// The offset to the location that requires relocating.
    ///
    /// For [`ElfType::EXECUTABLE`][ete] binaries, this value denotes an offset
    /// within a section header. For [`ElfType::RELOCATABLE`][etr] binaries, this
    /// value denotes a virtual address affected by a relocation.
    pub offset: u64,
    /// Gives the symbol table index and the type of relocation to display.
    pub info: u64,
}

/// 64-bit version of an ELF relocation with addend entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64Rela {
    /// The offset to the location that requires relocating.
    ///
    /// For [`ElfType::EXECUTABLE`][ete] binaries, this value denotes an offset
    /// within a section header. For [`ElfType::RELOCATABLE`][etr] binaries, this
    /// value denotes a virtual address affected by a relocation.
    pub offset: u64,
    /// Gives the symbol table index and the type of relocation to display.
    pub info: u64,
    /// A constant addend used to compute the value stored in the relocatable
    /// field.
    pub addend: i64,
}
