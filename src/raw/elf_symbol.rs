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
    ///
    /// In relocatable files, this holds alignment constraints for a symbol whose section index is
    /// [`SHN_COMMON`]. In relocatable files, this holds a section offset for a defined symbol from
    /// the beginning of the section that [`Elf64Symbol::section_index`] identitifies. In
    /// executable and shared object files, this holds a virtual address.
    pub value: u32,
    /// The size of the symbol.
    pub size: u32,
    /// The symbol's type and binding attributes.
    pub info: SymbolInfo,
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
    /// If the [`Elf64Symbol::name`] is non-zero, it represents a string table index
    /// that indicates the symbol's name. Otherwise, the symbol has no name.
    pub name: u32,
    /// The symbol's type and binding attributes.
    pub info: SymbolInfo,
    /// Currently, this only specifies a symbol's visibility.
    pub other: u8,
    /// The index of the section to which this symbol is defined in relation.
    pub section_index: u16,
    /// The value of the associated symbol, which may be an absolute value, an address,
    /// or other type, depending on the context.
    ///
    /// In relocatable files, this holds alignment constraints for a symbol whose section index is
    /// [`SHN_COMMON`]. In relocatable files, this holds a section offset for a defined symbol from
    /// the beginning of the section that [`Elf64Symbol::section_index`] identitifies. In
    /// executable and shared object files, this holds a virtual address.
    pub value: u64,
    /// The size of the symbol.
    pub size: u64,
}

/// Specifies the [`SymbolType`] and [`SymbolBinding`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolInfo(pub u8);

impl SymbolInfo {
    /// The [`SymbolBinding`] that this [`SymbolInfo`] indicates.
    pub const fn binding(self) -> SymbolBinding {
        SymbolBinding(self.0 >> 4)
    }

    /// The [`SymbolType`] that this [`SymbolInfo`] indicates.
    pub const fn symbol_type(self) -> SymbolType {
        SymbolType(self.0 & 0x3)
    }
}

/// The linkage visiblity and behavior.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolBinding(pub u8);

impl SymbolBinding {
    /// The symbol is local, which means that the symbol is not visible outside of the ELF file
    /// that defines the symbol.
    pub const LOCAL: Self = Self(0);
    /// The symbol is global, which means that the symbol is visible to all ELF files being
    /// combined and can satisfiy undefined references from other files.
    pub const GLOBAL: Self = Self(1);
    /// The symbol is a weak binding, which is a lower priority global symbol.
    pub const WEAK: Self = Self(2);

    /// Start of the range reserved for os-specific semantics.
    pub const OS_SPECIFIC_START: Self = Self(10);
    /// End of the range reserved for os-specific semantics.
    pub const OS_SPECIFIC_END: Self = Self(12);

    /// Start of the range reserved for processor-specific semantics.
    pub const PROCESSOR_SPECIFIC_START: Self = Self(13);
    /// End of the range reserved for processor-specific semantics.
    pub const PROCESSOR_SPECIFIC_END: Self = Self(15);
}

/// The type of the symbol.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolType(pub u8);

impl SymbolType {
    /// The symbol's type is not specified.
    pub const NO_TYPE: Self = Self(0);
    /// The symbol is associated with a data object.
    pub const OBJECT: Self = Self(1);
    /// The symbol is associated with a function or other code.
    pub const FUNCTION: Self = Self(2);
    /// The symbol is associated with a section.
    pub const SECTION: Self = Self(3);
    /// The symbol's name gives the name of the source file associated with the object file.
    ///
    /// If present, the binding is [`SymbolBinding::LOCAL`], its section index is [`SHN_ABS`], and
    /// it precedes the other [`SymbolBinding::LOCAL`] symbols for the file.
    pub const FILE: Self = Self(4);
    /// The symbol labels an uninitialized common block.
    pub const COMMON: Self = Self(5);
    /// The symbol specifies a thread-local storage entity, which when defined gives the assigned
    /// offset of the symbol.
    pub const TLS: Self = Self(6);

    /// Start of the range reserved for os-specific semantics.
    pub const OS_SPECIFIC_START: Self = Self(10);
    /// End of the range reserved for os-specific semantics.
    pub const OS_SPECIFIC_END: Self = Self(12);

    /// Start of the range reserved for processor-specific semantics.
    pub const PROCESSOR_SPECIFIC_START: Self = Self(13);
    /// End of the range reserved for processor-specific semantics.
    pub const PROCESSOR_SPECIFIC_END: Self = Self(15);
}

/// The visibility of the symbol.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolVisibility(pub u8);

impl SymbolVisibility {
    /// The visibility is as specified by the [`SymbolBinding`] type.
    pub const DEFAULT: Self = Self(0);
    /// This is defined by processor supplements to further constrain hidden symbols.
    pub const INTERNAL: Self = Self(1);
    /// The symbol's name is not visible to other ELF files.
    pub const HIDDEN: Self = Self(2);
    /// The symbol is not premeptable within the defining ELF file, but is still visible outside of
    /// the defining ELF file.
    pub const PROTECTED: Self = Self(3);
}
