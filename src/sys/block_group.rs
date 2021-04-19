use core::fmt::{self, Debug};
use core::mem;

use alloc::vec::Vec;

use error::Error;
use sector::{Address, SectorSize};
use volume::Volume;

/// The Block Group Descriptor Table contains a descriptor for each block group
/// within the file system. The number of block groups within the file system,
/// and correspondingly, the number of entries in the Block Group Descriptor
/// Table, is described above. Each descriptor contains information regarding
/// where important data structures for that group are located.
///
/// The (`BlockGroupDescriptor`) table is located in the block immediately
/// following the Superblock. So if the block size (determined from a field in
/// the superblock) is 1024 bytes per block, the Block Group Descriptor Table
/// will begin at block 2. For any other block size, it will begin at block 1.
/// Remember that blocks are numbered starting at 0, and that block numbers
/// don't usually correspond to physical block addresses.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct BlockGroupDescriptor {
    /// Block address of block usage bitmap
    pub block_usage_addr: u32,
    /// Block address of inode usage bitmap
    pub inode_usage_addr: u32,
    /// Starting block address of inode table
    pub inode_table_block: u32,
    /// Number of unallocated blocks in group
    pub free_blocks_count: u16,
    /// Number of unallocated inodes in group
    pub free_inodes_count: u16,
    /// Number of directories in group
    pub dirs_count: u16,
    #[doc(hidden)]
    _reserved: [u8; 14],
}

impl Debug for BlockGroupDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BlockGroupDescriptor")
            .field("block_usage_addr", &{ self.block_usage_addr })
            .field("inode_usage_addr", &{ self.inode_usage_addr })
            .field("inode_table_block", &{ self.inode_table_block })
            .field("free_blocks_count", &{ self.free_blocks_count })
            .field("free_inodes_count", &{ self.free_inodes_count })
            .field("dirs_count", &{ self.dirs_count })
            .finish()
    }
}

impl BlockGroupDescriptor {
    ///
    /// # Safety
    pub unsafe fn find_descriptor<S: SectorSize, V: Volume<u8, S>>(
        haystack: &V,
        offset: Address<S>,
    ) -> Result<(BlockGroupDescriptor, Address<S>), Error> {
        let end =
            offset + Address::from(mem::size_of::<BlockGroupDescriptor>());
        if haystack.size() < end {
            return Err(Error::AddressOutOfBounds {
                sector: end.sector(),
                offset: end.offset(),
                size: end.sector_size(),
            });
        }

        let descr = haystack
            .slice_unchecked(offset..end)
            .dynamic_cast::<BlockGroupDescriptor>();

        Ok(descr)
    }

    ///
    /// # Safety
    pub unsafe fn find_descriptor_table<S: SectorSize, V: Volume<u8, S>>(
        haystack: &V,
        offset: Address<S>,
        count: usize,
    ) -> Result<(Vec<BlockGroupDescriptor>, Address<S>), Error> {
        let end = offset
            + Address::from(count * mem::size_of::<BlockGroupDescriptor>());
        if haystack.size() < end {
            return Err(Error::AddressOutOfBounds {
                sector: end.sector(),
                offset: end.offset(),
                size: end.sector_size(),
            });
        }

        let mut vec = Vec::with_capacity(count);
        for i in 0..count {
            let offset = offset
                + Address::from(i * mem::size_of::<BlockGroupDescriptor>());
            vec.push({
                BlockGroupDescriptor::find_descriptor(haystack, offset)?.0
            });
        }

        Ok((vec, offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sector::{Address, Size512};

    #[test]
    fn find() {
        let volume = vec![0_u8; 4096];
        let table = unsafe {
            BlockGroupDescriptor::find_descriptor_table(
                &volume,
                Address::<Size512>::new(4, 0),
                8,
            )
        };
        assert!(
            table.is_ok(),
            "Err({:?})",
            table.err().unwrap_or_else(|| unreachable!()),
        );
        let table = table.unwrap_or_else(|_| unreachable!());
        assert_eq!(table.0.len(), 8);
    }
}
