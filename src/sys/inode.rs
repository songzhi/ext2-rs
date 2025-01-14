use core::fmt::{self, Debug};
use core::mem;

use error::Error;
use sector::{Address, SectorSize};
use volume::Volume;

/// An inode is a structure on the disk that represents a file, directory,
/// symbolic link, etc. Inodes do not contain the data of the file / directory /
/// etc. that they represent. Instead, they link to the blocks that actually
/// contain the data. This lets the inodes themselves have a well-defined size
/// which lets them be placed in easily indexed arrays. Each block group has an
/// array of inodes it is responsible for, and conversely every inode within a
/// file system belongs to one of such tables (and one of such block groups).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Inode {
    /// Type and Permissions (see below)
    pub type_perm: TypePerm,
    /// User ID
    pub uid: u16,
    /// Lower 32 bits of size in bytes
    pub size_low: u32,
    /// Last Access Time (in POSIX time)
    pub atime: u32,
    /// Creation Time (in POSIX time)
    pub ctime: u32,
    /// Last Modification time (in POSIX time)
    pub mtime: u32,
    /// Deletion time (in POSIX time)
    pub dtime: u32,
    /// Group ID
    pub gid: u16,
    /// Count of hard links (directory entries) to this inode. When this
    /// reaches 0, the data blocks are marked as unallocated.
    pub hard_links: u16,
    /// Count of disk sectors (not Ext2 blocks) in use by this inode, not
    /// counting the actual inode structure nor directory entries linking
    /// to the inode.
    pub sectors_count: u32,
    /// Flags
    pub flags: Flags,
    /// Operating System Specific value #1
    pub _os_specific_1: [u8; 4],
    /// Direct block pointers
    pub direct_pointer: [u32; 12],
    /// Singly Indirect Block Pointer (Points to a block that is a list of
    /// block pointers to data)
    pub indirect_pointer: u32,
    /// Doubly Indirect Block Pointer (Points to a block that is a list of
    /// block pointers to Singly Indirect Blocks)
    pub doubly_indirect: u32,
    /// Triply Indirect Block Pointer (Points to a block that is a list of
    /// block pointers to Doubly Indirect Blocks)
    pub triply_indirect: u32,
    /// Generation number (Primarily used for NFS)
    pub gen_number: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1,
    /// Extended attribute block (File ACL).
    pub ext_attribute_block: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Upper
    /// 32 bits of file size (if feature bit set) if it's a file,
    /// Directory ACL if it's a directory
    pub size_high: u32,
    /// Block address of fragment
    pub frag_block_addr: u32,
    /// Operating System Specific Value #2
    pub _os_specific_2: [u8; 12],
}

impl Debug for Inode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Inode")
            .field("type_perm", &{ self.type_perm })
            .field("uid", &{ self.uid })
            .field("size_low", &{ self.size_low })
            .field("atime", &{ self.atime })
            .field("ctime", &{ self.ctime })
            .field("mtime", &{ self.mtime })
            .field("dtime", &{ self.dtime })
            .field("gid", &{ self.gid })
            .field("hard_links", &{ self.hard_links })
            .field("sectors_count", &{ self.sectors_count })
            .field("flags", &{ self.flags })
            .field("os_specific_1", &self._os_specific_1)
            .field("direct_pointer", &{ self.direct_pointer })
            .field("indirect_pointer", &{ self.indirect_pointer })
            .field("doubly_indirect", &{ self.doubly_indirect })
            .field("triply_indirect", &{ self.triply_indirect })
            .field("gen_number", &{ self.gen_number })
            .field("ext_attribute_block", &{ self.ext_attribute_block })
            .field("size_high", &{ self.size_high })
            .field("frag_block_addr", &{ self.frag_block_addr })
            .field("os_specific_2", &self._os_specific_2)
            .finish()
    }
}

impl Inode {
    ///
    /// # Safety
    pub unsafe fn find_inode<S: SectorSize, V: Volume<u8, S>>(
        haystack: &V,
        offset: Address<S>,
        size: usize,
    ) -> Result<(Inode, Address<S>), Error> {
        if size != mem::size_of::<Inode>() {
            unimplemented!("inodes with a size != 128");
        }

        let end = offset + Address::from(size);
        if haystack.size() < end {
            return Err(Error::AddressOutOfBounds {
                sector: end.sector(),
                offset: end.offset(),
                size: end.sector_size(),
            });
        }

        let inode = haystack
            .slice_unchecked(offset..end)
            .dynamic_cast::<Inode>();

        Ok(inode)
    }
}

bitflags! {
    pub struct TypePerm: u16 {
        /// FIFO
        const FIFO = 0x1000;
        /// Character device
        const CHAR_DEVICE = 0x2000;
        /// Directory
        const DIRECTORY = 0x4000;
        /// Block device
        const BLOCK_DEVICE = 0x6000;
        /// Regular file
        const FILE = 0x8000;
        /// Symbolic link
        const SYMLINK = 0xA000;
        /// Unix socket
        const SOCKET = 0xC000;
        /// Other—execute permission
        const O_EXEC = 0x001;
        /// Other—write permission
        const O_WRITE = 0x002;
        /// Other—read permission
        const O_READ = 0x004;
        /// Group—execute permission
        const G_EXEC = 0x008;
        /// Group—write permission
        const G_WRITE = 0x010;
        /// Group—read permission
        const G_READ = 0x020;
        /// User—execute permission
        const U_EXEC = 0x040;
        /// User—write permission
        const U_WRITE = 0x080;
        /// User—read permission
        const U_READ = 0x100;
        /// Sticky Bit
        const STICKY = 0x200;
        /// Set group ID
        const SET_GID = 0x400;
        /// Set user ID
        const SET_UID = 0x800;
    }
}

bitflags! {
    pub struct Flags: u32 {
        /// Secure deletion (not used)
        const SECURE_DEL = 0x00000001;
        /// Keep a copy of data when deleted (not used)
        const KEEP_COPY = 0x00000002;
        /// File compression (not used)
        const COMPRESSION = 0x00000004;
        /// Synchronous updates—new data is written immediately to disk
        const SYNC_UPDATE = 0x00000008;
        /// Immutable file (content cannot be changed)
        const IMMUTABLE = 0x00000010;
        /// Append only
        const APPEND_ONLY = 0x00000020;
        /// File is not included in 'dump' command
        const NODUMP = 0x00000040;
        /// Last accessed time should not updated
        const DONT_ATIME = 0x00000080;
        /// Hash indexed directory
        const HASH_DIR = 0x00010000;
        /// AFS directory
        const AFS_DIR = 0x00020000;
        /// Journal file data
        const JOURNAL_DATA = 0x00040000;
    }
}

/// Unknown entry type
pub const UNKNOWN: u8 = 0;
/// FIFO entry type
pub const FIFO: u8 = 1;
/// Character device entry type
pub const CHAR_DEVICE: u8 = 2;
/// Directory entry type
pub const DIRECTORY: u8 = 3;
/// Block device entry type
pub const BLOCK_DEVICE: u8 = 4;
/// Regular file entry type
pub const FILE: u8 = 5;
/// Symbolic link entry type
pub const SYMLINK: u8 = 6;
/// Unix socket entry type
pub const SOCKET: u8 = 7;
