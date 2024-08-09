//! Abstraction of the various classes of an [`ElfFile`].

use core::{error, fmt};

/// A trait used to multiplex on the different classes of an [`ElfFile`].
pub trait ClassParse: Clone + Copy + Default + PartialEq + Eq {
    /// Retrieves the corresponding class-aware integer parsing object from
    /// [`ElfHeader::class`].
    ///
    /// # Errors
    ///
    /// Returns [`UnsupportedClass`] if the [`ClassParse`] type doesn't support
    /// parsing the class specified by `elf_ident_class`.
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClass>;

    /// Returns the [`Class`] of the current ELF file.
    fn into_class(self) -> Class;
}

/// Indicates how the ELF file should be parsed with respect to differences in
/// different sized architectures.
pub enum Class {
    /// Should be parsed as a 32-bit format.
    Class32,
    /// Should be parsed as a 64-bit format.
    Class64,
}

/// An error that ocurrs when the code does not support a particular [`ClassParse`]
/// object.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsupportedClass(u8);

impl fmt::Display for UnsupportedClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => writeln!(f, "invalid class not supported"),
            1 => writeln!(f, "32-bit class not supported"),
            2 => writeln!(f, "64-bit class not supported"),
            class => writeln!(f, "unknown class({class}) not supported"),
        }
    }
}

impl error::Error for UnsupportedClass {}

/// A zero-sized object indicating that support for only [`Class32`] [`ElfFile`]s.
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class32;

impl ClassParse for Class32 {
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClass> {
        if elf_ident_class != 1 {
            return Err(UnsupportedClass(elf_ident_class));
        }
        Ok(Class32)
    }

    fn into_class(self) -> Class {
        Class::Class32
    }
}

/// A zero-sized object indicating that support for only [`Class64`] [`ElfFile`]s.
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class64;

impl ClassParse for Class64 {
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClass> {
        if elf_ident_class != 2 {
            return Err(UnsupportedClass(elf_ident_class));
        }
        Ok(Class64)
    }

    fn into_class(self) -> Class {
        Class::Class64
    }
}

/// An object used to dispatch the [`ElfFile`] encoding to be used at runtime.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnyClass {
    /// Class is [`Class32`].
    Class32,
    /// Class is [`Class64`].
    Class64,
}

impl ClassParse for AnyClass {
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClass> {
        match elf_ident_class {
            1 => Ok(Self::Class32),
            2 => Ok(Self::Class64),
            unsupported => Err(UnsupportedClass(unsupported)),
        }
    }

    fn into_class(self) -> Class {
        match self {
            Self::Class32 => Class::Class32,
            Self::Class64 => Class::Class64,
        }
    }
}

impl Default for AnyClass {
    fn default() -> Self {
        Self::Class64
    }
}
