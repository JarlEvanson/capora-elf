//! Abstraction of the various classes of an [`ElfFile`].

use core::{error, fmt};

use crate::raw::elf_ident::Class as RawClass;

/// A trait used to multiplex on the different classes of an [`ElfFile`].
pub trait ClassParse: Clone + Copy + PartialEq + Eq {
    /// Retrieves the corresponding class-aware integer parsing object from
    /// [`ElfHeader::class`].
    ///
    /// # Errors
    ///
    /// Returns [`UnsupportedClassError`] if the [`ClassParse`] type doesn't support
    /// parsing the class specified by `elf_ident_class`.
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClassError>;

    /// Returns the [`Class`] of the current ELF file.
    fn into_class(self) -> Class;
}

/// Indicates how the ELF file should be parsed with respect to differences in
/// different sized architectures.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    /// Should be parsed as a 32-bit format.
    Class32,
    /// Should be parsed as a 64-bit format.
    Class64,
}

/// An error that ocurrs when the code does not support a particular [`ClassParse`]
/// object.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsupportedClassError(u8);

impl fmt::Display for UnsupportedClassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match RawClass(self.0) {
            RawClass::NONE => writeln!(f, "no class ELF parsing not supported"),
            RawClass::CLASS32 => writeln!(f, "32-bit class ELF parsing not supported"),
            RawClass::CLASS64 => writeln!(f, "64-bit class ELF parsing not supported"),
            RawClass(class) => writeln!(f, "unknown class({class}) not supported"),
        }
    }
}

impl error::Error for UnsupportedClassError {}

/// A zero-sized object indicating that support for only [`Class32`] [`ElfFile`]s.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class32;

impl ClassParse for Class32 {
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClassError> {
        if elf_ident_class != RawClass::CLASS32.0 {
            return Err(UnsupportedClassError(elf_ident_class));
        }
        Ok(Class32)
    }

    fn into_class(self) -> Class {
        Class::Class32
    }
}

/// A zero-sized object indicating that support for only [`Class64`] [`ElfFile`]s.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class64;

impl ClassParse for Class64 {
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClassError> {
        if elf_ident_class != RawClass::CLASS64.0 {
            return Err(UnsupportedClassError(elf_ident_class));
        }
        Ok(Class64)
    }

    fn into_class(self) -> Class {
        Class::Class64
    }
}

/// An object used to dispatch the [`ElfFile`] encoding to be used at runtime.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnyClass(Class);

impl ClassParse for AnyClass {
    fn from_elf_class(elf_ident_class: u8) -> Result<Self, UnsupportedClassError> {
        match RawClass(elf_ident_class) {
            RawClass::CLASS32 => Ok(Self(Class::Class32)),
            RawClass::CLASS64 => Ok(Self(Class::Class64)),
            RawClass(unsupported) => Err(UnsupportedClassError(unsupported)),
        }
    }

    fn into_class(self) -> Class {
        match self {
            Self(Class::Class32) => Class::Class32,
            Self(Class::Class64) => Class::Class64,
        }
    }
}
