//! Definitions related to ELF symbols.

/// 32-bit version of an ELF symbol entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32Symbol {
    /// The index into the ELF file's symbol string table, which holds the character
    /// representations of the symbol names.
    ///
    /// If the [`Elf32Symbol::name`] is non-zero, it represents a string table index
    /// that indicates the symbol's name. Otherwise, the symbol has no name.
    pub name: u32,
    /// The value of the associated symbol, which may be an absolute value, an address,
    /// or other type, depending on the context.
    pub value: u32,
    /// The size of the symbol.
    pub size: u32,
    /// The symbol's type and binding attributes.
    pub info: u8,
    /// Currently, this only specifies a symbol's visibility.
    pub other: u8,
    /// The index of the section to which this symbol is defined in relation.
    pub section_index: u16,
}

/// 64-bit version of an ELF symbol entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64Symbol {
    /// The index into the ELF file's symbol string table, which holds the character
    /// representations of the symbol names.
    ///
    /// If the [`Elf32Symbol::name`] is non-zero, it represents a string table index
    /// that indicates the symbol's name. Otherwise, the symbol has no name.
    pub name: u32,
    /// The symbol's type and binding attributes.
    pub info: u8,
    /// Currently, this only specifies a symbol's visibility.
    pub other: u8,
    /// The index of the section to which this symbol is defined in relation.
    pub section_index: u16,
    /// The value of the associated symbol, which may be an absolute value, an address,
    /// or other type, depending on the context.
    pub value: u64,
    /// The size of the symbol.
    pub size: u64,
}


