//! A simple library providing a pure-safe-rust interface for reading ELF object files.
//!

#![no_std]

pub mod class;
pub mod elf_header;
pub mod elf_ident;
pub mod encoding;
pub mod raw;

/// Obtains the size of the specfied filed, evaluated at const time.
///
/// This only works for [`Sized`] types.
#[macro_export]
macro_rules! field_size {
    ($t:ident, $field:ident) => {
        const {
            let m = core::mem::MaybeUninit::<$t>::uninit();

            // SAFETY:
            // $t is [`Sized`], and so the project to $field is
            // in bounds.
            let p = unsafe { core::ptr::addr_of!((*m.as_ptr()).$field) };

            const fn size_of_raw<T>(_: *const T) -> usize {
                core::mem::size_of::<T>()
            }

            size_of_raw(p)
        }
    };
}
