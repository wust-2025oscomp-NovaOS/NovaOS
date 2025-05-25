use super::{
    def::ROOT_INO,
    inode::{Ext4Inode, Ext4InodeInner},
};
use alloc::sync::Arc;
use crate::sync::UPIntrFreeCell;
use ext4_rs::{Ext4, BlockDevice};
use crate::fs::vfs::{Inode, FileSystem};
/// 对文件系统进行封装
pub struct Ext4FS {
    pub ext4: UPIntrFreeCell<Ext4>,
}

impl Ext4FS {
    pub fn new(block_dev: Arc<dyn BlockDevice>) -> Self {
        let ext4 = Ext4::open(block_dev);
        Self { 
            ext4: unsafe { UPIntrFreeCell::new(ext4) } 
        }
    }
}

impl FileSystem for Ext4FS {
    // fn fs_type(&self) -> FileSystemType {
    //     FileSystemType::EXT4
    // }
    fn root_inode(self: Arc<Self>) -> Arc<dyn Inode> {
        let inode = Ext4Inode {
            fs:    self.clone(),
            ino:   ROOT_INO,
            inner: unsafe { UPIntrFreeCell::new(Ext4InodeInner { fpos: 0 }) },
        };
        Arc::new(inode)
    }

    fn fs_type(&self) -> crate::fs::vfs::FileSystemType {
        crate::fs::vfs::FileSystemType::EXT4
    }
}
