//! Definitions related to the ELF file identifier.

/// The current version of the ELF file header.
pub const CURRENT_ELF_HEADER_VERSION: u8 = 1;

/// Block of machine-independent data to mark the file as an ELF file
/// and provide enough information for the remainder of the ELF file to be
/// decoded.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElfIdent {
    /// Holds magic numbers to identify the file as an ELF file.
    pub magic: [u8; 4],
    /// The file's class (native word size).
    pub class: Class,
    /// Encoding of data structures used by the object file container
    /// and data contained in object file sections.
    pub data: Encoding,
    /// The ELF header version number..
    pub header_version: u8,
    /// Identifies the OS or ABI specific extensions used by this file.
    pub os_abi: OsAbi,
    /// The version of the ABI to which the object file is targeted.
    ///
    /// This should be zero if the [`ElfIdent::os_abi`] field has no
    /// definitions or no version values in the processor supplement.
    pub abi_version: u8,
    /// Unused bytes, should all be zero.
    pub _padding: [u8; 7],
}

impl ElfIdent {
    /// The magic numbers required by the specification to identify the
    /// file as an ELF file.
    pub const MAGIC_BYTES: [u8; 4] = [0x7F, b'E', b'L', b'F'];

    /// The current version of the ELF file header.
    pub const CURRENT_VERSION: u8 = 1;
}

/// Specifier of the ELF file class, which determines the sizing
/// of various items in the ELF file format.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class(u8);

impl Class {
    /// Invalid [`Class`] specifier.
    pub const NONE: Self = Self(0);
    /// ELF file is formatted in its 32-bit format.
    pub const CLASS32: Self = Self(1);
    /// ELF file is formatted in its 64-bit format.
    pub const CLASS64: Self = Self(2);
}

/// Specifier of the ELF file data encoding, which determines the encoding
/// of both the data structures used by the ELF file format and data contained
/// in the object file sections.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Encoding(u8);

impl Encoding {
    /// Invalid [`Encoding`] specifier.
    pub const NONE: Self = Self(0);
    /// The encoding of the ELF file format uses little endian
    /// two's complement integers.
    pub const LITTLE_ENDIAN_TWOS: Self = Self(1);
    /// The encoding of the ELF file format uses big endian
    /// two's complement integers.
    pub const BIG_ENDIAN_TWOS: Self = Self(2);
}

/// Specifier of the OS or ABI specific ELF extensions used by this file.
///
/// This field determines the interpretation of various OS or ABI specific values.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OsAbi(u8);

impl OsAbi {
    /// No extensions or unspecified extensions.
    pub const NONE: Self = Self(0);
    /// Hewlett-Packard HP_UX.
    pub const HP_UX: Self = Self(1);
    /// NetBSD
    pub const NETBSD: Self = Self(2);
    /// Gnu (Historically also Linux)
    pub const GNU: Self = Self(3);
    /// Sun Solaris
    pub const SUN_SOLARIS: Self = Self(6);
    /// AIX
    pub const AIX: Self = Self(7);
    /// IRIX
    pub const IRIX: Self = Self(8);
    /// FreeBSD
    pub const FREEBSD: Self = Self(9);
    /// Compaq TRU64 UNIX
    pub const COMPAQ_TRU64_UNIX: Self = Self(10);
    /// Novell Modesto
    pub const NOVELL_MODESTO: Self = Self(11);
    /// Open BSD
    pub const OPENBSD: Self = Self(12);
    /// Open VMS
    pub const OPEN_VMS: Self = Self(13);
    /// Hewlett-Packard Non-Stop Kernel
    pub const HP_NSK: Self = Self(14);
    /// Amiga Research OS
    pub const AMIGA_RESEARCH: Self = Self(15);
    /// The FenixOS highly scalable multi-core OS
    pub const FENIXOS: Self = Self(16);
    /// Nuxi CloudABI
    pub const CLOUD_ABI: Self = Self(17);
    /// Stratus Technologies OpenVOS
    pub const OPENVOS: Self = Self(18);
    /// Start of the architecture specific value range.
    pub const ARCHITECTURE_SPECIFIC_START: Self = Self(64);
    /// Inclusive end of the architecture specific value range.
    pub const ARCHITECTURE_SPECIFIC_END: Self = Self(255);
}
