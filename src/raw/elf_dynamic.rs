//! Definitions related to ELF dynamic tags.

/// 32-bit version of an ELF dynamic array entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32Dynamic {
    /// The tag that identitifes how [`Elf32DynamicTag::value`] should be interpreted.
    pub tag: Elf32DynamicTag,
    /// The value associated with this [`Elf32Dynamic`].
    pub value: u32,
}

/// 64-bit version of an ELF dynamic array entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64Dynamic {
    /// The tag that identitifes how [`Elf32DynamicTag::value`] should be interpreted.
    pub tag: Elf64DynamicTag,
    /// The value associated with this [`Elf64Dynamic`].
    pub value: u64,
}

/// 32-bit version of an ELF dynamic tag.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32DynamicTag(pub i32);

/// 64-bit version of an ELF dynamic tag.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64DynamicTag(pub i64);

/// [`Class`][c] independent version of an ELF dynamic tag.
///
/// [c]: crate::class:Class
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElfDynamicTag(pub i32);

impl ElfDynamicTag {
    /// Marks the end of the ELF dynamic array.
    pub const NULL: Self = Self(0);
    /// Holds the string table offset of a null-terminated string which is the name of a needed
    /// library. This offset is an index into the table recording in the
    /// [`ElfDynamicTag::STRING_TABLE`] entry.
    ///
    /// The dynamic array may contain multiple entries with this type, and the order of these
    /// entries is significant, but only relative to entries of the same type.
    pub const NEEDED: Self = Self(1);
    /// Holds the total size, in bytes, of the relocation entries associated with the procedure
    /// linkage table. If an [`ElfDynamicTag::JMP_REL`] entry is present, this tag must accompany
    /// it.
    pub const PLT_REL_SIZE: Self = Self(2);
    /// Holds an address associated with the procedure linkage table and/or the global offset
    /// table.
    pub const PLT_GOT: Self = Self(3);
    /// Holds the address of the symbol hash table, which refers to the symbol table referenced in
    /// an [`ElfDynamicTag::SYMBOL_TABLE`] entry.
    pub const HASH: Self = Self(4);
    /// Holds the address of the string table.
    pub const STRING_TABLE: Self = Self(5);
    /// Holds the address of the symbol table.
    pub const SYMBOL_TABLE: Self = Self(6);
    /// Holds the address of a relocation table, with explicit addends.
    ///
    /// If this entry is present, the dynamic array must also have [`ElfDynamicTag::RELA_SIZE`] and
    /// [`ElfDynamicTag::RELA_ENTRY_SIZE`] entries.
    pub const RELA_TABLE: Self = Self(7);
    /// Holds the total size, in bytes, of the relocation table pointed to by the [`ElfDynamicTag::RELA_TABLE`] .
    pub const RELA_SIZE: Self = Self(8);
    /// Holds the size, in bytes, of the relocation table pointed to by the
    /// [`ElfDynamicTag::RELA_TABLE`].
    pub const RELA_ENTRY_SIZE: Self = Self(9);
    /// Holds the total size, in bytes, of the string table pointed to by the
    /// [`ElfDynamicTag::STRING_TABLE`] entry.
    pub const STRING_TABLE_SIZE: Self = Self(10);
    /// Holds the size, in bytes, of an entry in the symbol table pointed to by the
    /// [`ElfDynamicTag::SYMBOL_TABLE`] entry.
    pub const SYMBOL_ENTRY_SIZE: Self = Self(11);
    /// Holds the address of the initialization function.
    pub const INIT: Self = Self(12);
    /// Holds the address of the termination function.
    pub const FINI: Self = Self(13);
    /// Holds the string table offset of a null-terminated string giving the name of the shared
    /// object.
    pub const SO_NAME: Self = Self(14);
    /// Holds the string table offset of a null-terminated string giving the library search path
    /// string.
    ///
    /// The use of this has been superseded by [`ElfDynamicTag::RUNPATH`].
    pub const RPATH: Self = Self(15);
    /// Indicates that the dynamic linker's symbol resolution algorithm should start from the
    /// shared object and then if the shared object fails to provided the referenced symbol, then
    /// the linker searches the executable file and other shared objects as usual.
    pub const SYMBOLIC: Self = Self(16);
    /// Holds the address of a relocation table, with implicit addends.
    ///
    /// If this entry is present, the dynamic array must also have [`ElfDynamicTag::REL_SIZE`] and
    /// [`ElfDynamicTag::RELA_ENTRY_SIZE`] entries.
    pub const REL_TABLE: Self = Self(17);
    /// The total size, in bytes, of the relocation table pointed to be the
    /// [`ElfDynamicTag::RELA_TABLE`] entry.
    pub const REL_SIZE: Self = Self(18);
    /// The size, in bytes, of an entry in the relocation table pointed to be the
    /// [`ElfDynamicTag::RELA_TABLE`] entry.
    pub const REL_ENTRY_SIZE: Self = Self(19);
    /// The type of relocation entry to which the prodedure linkage table refers.
    pub const PLT_REL: Self = Self(20);
    /// This member is used for debugging, but its contents are not specified by the ABI.
    pub const DEBUG: Self = Self(21);
    /// Indicates that one or more relocation entries might cause a modification to a non-writable segment.
    ///
    /// The use of this has been superseded by [`ElfDynamicTag::TEXT_REL`].
    pub const TEXT_REL: Self = Self(22);
    /// Holds the address of relocation entries associated solely with the procedure linkage table.
    ///
    /// If this entry is present, the dynamic array must also have [`ElfDynamicTag::PLT_REL`] and
    /// [`ElfDynamicTag::PLT_REL_SIZE`] entries.
    pub const JMP_REL: Self = Self(23);
    /// Indicates that the dynamic linker should process all relocations for the object containing
    /// this entry before transferring control to the program.
    pub const BIND_NOW: Self = Self(24);
    /// Holds the address of the array of pointers to initialization functions.
    pub const INIT_ARRAY: Self = Self(25);
    /// Holds the address of the array of pointers to termination functions.
    pub const FINI_ARRAY: Self = Self(26);
    /// Holds the size, in bytes, of the array of pointers to initialization functions.
    pub const INIT_ARRAY_SIZE: Self = Self(27);
    /// Holds the size, in bytes, of the array of pointers to termination functions.
    pub const FINI_ARRAY_SIZE: Self = Self(28);
    /// Holds the strin table offset of ta null-terminated library search path string.
    pub const RUNPATH: Self = Self(29);
    /// Holds flag values specfic to the object being loaded.
    pub const FLAGS: Self = Self(30);

    /// Holds the address of the array of pointers to pre-initialization functions.
    ///
    /// This is processed only in an executable file.
    pub const PREINIT_ARRAY: Self = Self(32);
    /// Holds the size, in bytes, of the array of pointers to pre-initialization functions.
    pub const PREINIT_ARRAY_SIZE: Self = Self(33);
    /// Holds the address of the [`SHT_SYMTAB_SHNDX`] section associated with the dynamic symbol
    /// table referenced by the [`ElfDynamicTag::SYMBOL_TABLE`] element.
    pub const SYMBOL_TABLE_SECTION_INDEX: Self = Self(34);
}

impl From<Elf32DynamicTag> for ElfDynamicTag {
    fn from(value: Elf32DynamicTag) -> Self {
        Self(value.0)
    }
}

impl From<Elf64DynamicTag> for ElfDynamicTag {
    fn from(value: Elf64DynamicTag) -> Self {
        Self(TryInto::<i32>::try_into(value.0).expect("out of range according to specification"))
    }
}
