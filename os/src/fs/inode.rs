use super::File;
use crate::mm::UserBuffer;
use crate::sync::UPIntrFreeCell;
use alloc::borrow::ToOwned;
use alloc::sync::Arc;
use alloc::string::String;
use alloc::vec::Vec;
use super::ROOT_INODE;
//use easy_fs::{EasyFileSystem, Inode};
use super::vfs::Inode;

pub struct OSInode {
    readable: bool,
    writable: bool,
    inner: UPIntrFreeCell<OSInodeInner>,
}

pub struct OSInodeInner {
    offset: usize,
    name: String,
    inode: Arc<dyn Inode>,
}

impl OSInode {
    pub fn new(readable: bool, writable: bool, inode: Arc<dyn Inode>, name: &str) -> Self {
        Self {
            readable,
            writable,
            inner: unsafe { UPIntrFreeCell::new(OSInodeInner { offset: 0, inode, name: name.to_owned() }) },
        }
    }
    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.exclusive_access();
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            //println!("read_all: offset = {}, len = {}", inner.offset, len);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }

    pub fn get_inode(&self) -> Arc<dyn Inode> {
        self.inner.exclusive_access().inode.clone()
    }
}

// lazy_static! {
//     pub static ref ROOT_INODE: Arc<dyn Inode> = {
//         let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
//         Arc::new(EasyFileSystem::root_inode(&efs))
//     };
// }

pub fn list_apps() {
    println!("/**** APPS ****");
    for app in ROOT_INODE.get_inode().ls() {
        println!("{}", app);
    }
    println!("**************/")
}

// bitflags! {
//     pub struct OpenFlags: u32 {
//         const RDONLY = 0;
//         const WRONLY = 1 << 0;
//         const RDWR = 1 << 1;
//         const CREATE = 1 << 9;
//         const TRUNC = 1 << 10;
//     }
// }

// impl OpenFlags {
//     /// Do not check validity for simplicity
//     /// Return (readable, writable)
//     pub fn read_write(&self) -> (bool, bool) {
//         if self.is_empty() {
//             (true, false)
//         } else if self.contains(Self::WRONLY) {
//             (false, true)
//         } else {
//             (true, true)
//         }
//     }
// }

// pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
//     let (readable, writable) = flags.read_write();
//     if flags.contains(OpenFlags::CREATE) {
//         if let Some(inode) = ROOT_INODE.lookup(name) {
//             // clear size
//             inode.clear();
//             Some(Arc::new(OSInode::new(readable, writable, inode)))
//         } else {
//             // create file
//             ROOT_INODE
//                 .create(name)
//                 .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
//         }
//     } else {
//         ROOT_INODE.lookup(name).map(|inode| {
//             if flags.contains(OpenFlags::TRUNC) {
//                 inode.clear();
//             }
//             Arc::new(OSInode::new(readable, writable, inode))
//         })
//     }
// }

impl File for OSInode {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_read_size = 0usize;
        for slice in buf.buffers.iter_mut() {
            let read_size = inner.inode.read_at(inner.offset, *slice);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }
    fn write(&self, buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_write_size = 0usize;
        for slice in buf.buffers.iter() {
            let write_size = inner.inode.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len());
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }
}
