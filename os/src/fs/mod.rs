mod inode;
mod pipe;
mod stdio;
mod vfs;
mod ext4;
mod path;
use lazy_static::*;
use crate::drivers::BLOCK_DEVICE;
use ext4::Ext4FS;
use alloc::sync::Arc;
//use core
use crate::mm::UserBuffer;
use crate::sync::UPIntrFreeCell;
pub use inode::{OSInode, list_apps};
pub use vfs::{Dentry, FileSystemManager, Inode, InodeFileType, InodePerm};
pub use vfs::OpenFlags;
pub use pipe::make_pipe;
pub use stdio::{Stdin, Stdout};

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    //fn as_any(&self) -> &dyn Any;
}

lazy_static! {
    pub static ref FS_MANAGER: UPIntrFreeCell<FileSystemManager> = unsafe {UPIntrFreeCell::new(FileSystemManager::new())};
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<OSInode> = {
        let ext4fs = Arc::new(Ext4FS::new(BLOCK_DEVICE.clone()));
        FS_MANAGER.exclusive_access().mount(ext4fs, "/");
        let inode = FS_MANAGER.exclusive_access().rootfs().root_inode();
        Arc::new(OSInode::new(true, true, inode, "/"))
    };
}

pub fn init() {
    println!("init fs...");
    let _root = ROOT_INODE.clone();
}

/// 打开inode下的文件
pub fn open_file(inode: Arc<dyn Inode>, path: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    let create = flags.contains(OpenFlags::O_CREAT);
    println!("[kernel] create file: {} by {}", create, inode.ino());
    if let Some(inode) = inode.open(path, create) {
        println!("[kernel】open file success");
        Some(Arc::new(OSInode::new(true, true, inode, path)))
    } else {
        println!("[kernel] open file failed");
        None
    }
    // TODO: read_write
    // let (readable, writable) = flags.read_write();
    // if flags.contains(OpenFlags::O_CREAT) {
    //     if let Some(dentry) = inode.clone().lookup(name) {
    //         // clear size
    //         dentry.inode().clear();
    //         Some(Arc::new(OSInode::new(true, true, dentry.inode(), name)))
    //     } else {
    //         // create file
    //         let mut file_type = InodeFileType::S_IFREG;
    //         let mut perm= InodePerm::S_IREAD;
    //         if flags.contains(OpenFlags::O_DIRECTORY) {
    //             file_type = InodeFileType::S_IFDIR;
    //             perm = InodePerm::S_IREAD | InodePerm::S_IWRITE;
    //         } else {
    //             file_type = InodeFileType::S_IFREG;
    //             perm = InodePerm::S_IREAD | InodePerm::S_IWRITE;
    //         };
    //         let dentry = inode.create(name, file_type, perm)?;
    //         Some(Arc::new(OSInode::new(true, true, dentry.inode(), name)))
    //     }
    // } else if let Some(dentry) = inode.lookup(name) {
    //     if flags.contains(OpenFlags::O_TRUNC) {
    //         dentry.inode().clear();
    //     }
    //     Some(Arc::new(OSInode::new(true, true, dentry.inode(), name)))
    // } else {
    //     None
    // }
}