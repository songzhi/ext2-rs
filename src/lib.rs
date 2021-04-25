#![feature(const_fn)]
#![feature(step_trait)]
#![feature(step_trait_ext)]
#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;
extern crate genfs;
extern crate spin;

#[cfg(any(test))]
extern crate core;

pub mod error;
pub mod fs;
pub mod sector;
pub mod sys;
pub mod volume;

#[cfg(test)]
mod tests {
    use sys::block_group::*;
    use sys::inode::*;
    use sys::superblock::*;

    #[test]
    fn sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<Superblock>(), 1024);
        assert_eq!(size_of::<BlockGroupDescriptor>(), 32);
        assert_eq!(size_of::<Inode>(), 128);
    }
}
