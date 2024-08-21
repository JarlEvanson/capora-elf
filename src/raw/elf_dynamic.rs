//! Definitions related to ELF dynamic tags.

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32Dynamic {
    pub tag: Elf32DynamicTag,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64Dynamic {
    pub tag: Elf64DynamicTag,
    pub value: u64,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf32DynamicTag(i32);

impl Elf32DynamicTag {
    pub const NULL: Self = Self(0);
    pub const NEEDED: Self = Self(1);
    pub const PLT_REL_SIZE: Self = Self(2);
    pub const PLT_GOT: Self = Self(3);
    pub const HASH: Self = Self(4);
    pub const STRING_TABLE: Self = Self(5);
    pub const SYMBOL_TABLE: Self = Self(6);
    pub const RELA_TABLE: Self = Self(7);
    pub const RELA_SIZE: Self = Self(8);
    pub const RELA_ENTRY_SIZE: Self = Self(9);
    pub const STRING_TABLE_SIZE: Self = Self(10);

    pub const SYMBOL_ENTRY_SIZE: Self = Self(11);
    pub const INIT: Self = Self(12);
    pub const FINI: Self = Self(13);
    pub const SO_NAME: Self = Self(14);
    pub const RPATH: Self = Self(15);

    pub const SYMBOLIC: Self = Self(16);
    pub const REL_TABLE: Self = Self(17);
    pub const REL_SIZE: Self = Self(18);
    pub const REL_ENTRY_SIZE: Self = Self(19);

    pub const PLT_REL: Self = Self(20);
    pub const DEBUG: Self = Self(21);

    pub const TEXT_REL: Self = Self(22);
    pub const JMP_REL: Self = Self(23);
    pub const BIND_NOW: Self = Self(24);
    pub const INIT_ARRAY: Self = Self(25);
    pub const FINI_ARRAY: Self = Self(26);
    pub const INIT_ARRAY_SIZE: Self = Self(27);
    pub const FINI_ARRAY_SIZE: Self = Self(28);

    pub const RUNPATH: Self = Self(29);
    pub const FLAGS: Self = Self(30);

    pub const ENCODING: Self = Self(31);
    pub const PREINIT_ARRAY: Self = Self(32);
    pub const PREINIT_ARRAY_SIZE: Self = Self(33);
    pub const SYMBOL_TABLE_SECTION_INDEX: Self = Self(34);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf64DynamicTag(i64);

impl Elf64DynamicTag {
    pub const NULL: Self = Self(0);
    pub const NEEDED: Self = Self(1);
    pub const PLT_REL_SIZE: Self = Self(2);
    pub const PLT_GOT: Self = Self(3);
    pub const HASH: Self = Self(4);
    pub const STRING_TABLE: Self = Self(5);
    pub const SYMBOL_TABLE: Self = Self(6);
    pub const RELA_TABLE: Self = Self(7);
    pub const RELA_SIZE: Self = Self(8);
    pub const RELA_ENTRY_SIZE: Self = Self(9);
    pub const STRING_TABLE_SIZE: Self = Self(10);

    pub const SYMBOL_ENTRY_SIZE: Self = Self(11);
    pub const INIT: Self = Self(12);
    pub const FINI: Self = Self(13);
    pub const SO_NAME: Self = Self(14);
    pub const RPATH: Self = Self(15);

    pub const SYMBOLIC: Self = Self(16);
    pub const REL_TABLE: Self = Self(17);
    pub const REL_SIZE: Self = Self(18);
    pub const REL_ENTRY_SIZE: Self = Self(19);

    pub const PLT_REL: Self = Self(20);
    pub const DEBUG: Self = Self(21);

    pub const TEXT_REL: Self = Self(22);
    pub const JMP_REL: Self = Self(23);
    pub const BIND_NOW: Self = Self(24);
    pub const INIT_ARRAY: Self = Self(25);
    pub const FINI_ARRAY: Self = Self(26);
    pub const INIT_ARRAY_SIZE: Self = Self(27);
    pub const FINI_ARRAY_SIZE: Self = Self(28);

    pub const RUNPATH: Self = Self(29);
    pub const FLAGS: Self = Self(30);

    pub const ENCODING: Self = Self(31);
    pub const PREINIT_ARRAY: Self = Self(32);
    pub const PREINIT_ARRAY_SIZE: Self = Self(33);
    pub const SYMBOL_TABLE_SECTION_INDEX: Self = Self(34);
}
