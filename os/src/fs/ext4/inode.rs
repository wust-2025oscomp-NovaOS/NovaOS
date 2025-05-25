use alloc::sync::Arc;
use crate::fs::vfs::{Dentry, FileAttr, Inode, InodeFileType, InodePerm};
use crate::sync::UPIntrFreeCell;
use crate::fs::{File, OpenFlags};
use alloc::string::String;
use alloc::vec::Vec;
use super::fs::Ext4FS;

// 定义ext4inode
pub struct Ext4Inode {
    pub fs:    Arc<Ext4FS>,
    pub ino:   u32,
    pub inner: UPIntrFreeCell<Ext4InodeInner>,
}
/// 文件偏移
pub struct Ext4InodeInner {
    pub fpos: usize,
}

impl Inode for Ext4Inode { 
    /// 获得文件类型
    fn file_type(&self) -> InodeFileType {
        self.get_attr().unwrap().file_type
    }
    /// 获得文件属性
    fn get_attr(&self) -> Option<FileAttr> {
        let file_attr: ext4_rs::FileAttr = self.fs.ext4.exclusive_access().fuse_getattr(self.ino as u64).unwrap();
        Some(FileAttr {
            ino: self.ino as u64,
            size: file_attr.size,
            blocks: file_attr.blocks,
            gid: file_attr.gid,
            uid: file_attr.uid,
            atime: file_attr.atime,
            mtime: file_attr.mtime,
            ctime: file_attr.ctime,
            file_type: InodeFileType::from_bits(file_attr.kind.bits()).unwrap(),
            perm: InodePerm::from_bits(file_attr.perm.bits()).unwrap(),
        })
    }
    /// 获得文件的inode编号
    fn ino(&self) -> u64 {
        self.ino as u64
    }
    /// 设置文件属性
    fn set_attr(&self, attr: &FileAttr) {
        self.fs.ext4.exclusive_access().fuse_setattr(self.ino as u64, Some((attr.file_type.bits() | attr.perm.bits()) as u32), Some(attr.uid), Some(attr.gid), Some(attr.size), Some(attr.atime), Some(attr.mtime), Some(attr.ctime), None, None, None, None, None);
    }

    fn read_at(&self, offset: usize, buf: &mut [u8]) -> usize {
        //let data = self.fs.ext4.exclusive_access().fuse_read(self.ino as u64, 0, offset as i64, buf.len() as u32, 0, None).unwrap();
        //buf.copy_from_slice(&data.as_slice());
        //data.len()
        let read_len = self.fs.ext4.exclusive_access().read_at(self.ino, offset, buf).unwrap();
        read_len
    }
    /// 写数据
    fn write_at(&self, offset: usize, buf: &[u8]) -> usize {
        self.fs.ext4.exclusive_access().fuse_write(self.ino as u64, 0, offset as i64, buf, 0, 0, None).unwrap()
    }
    fn permissions(&self) -> InodePerm {
        self.get_attr().unwrap().perm
    }
    /// 查找子项
    fn lookup(&self, name: &str) -> Option<Arc<Dentry>> {
        let ino = self.fs.ext4.exclusive_access().fuse_lookup(self.ino as u64, name).unwrap().ino;
        if ino == 0 {
            return None;
        }
        let inode = Arc::new(Ext4Inode {
            fs: self.fs.clone(),
            ino:  ino as u32,
            inner: unsafe { UPIntrFreeCell::new(Ext4InodeInner { fpos: 0 }) }
        });
        Some(
            Arc::new(Dentry::new(name,inode))
        )
    }
    /// 创建并打开一个文件
    fn create(
            &self,
            name: &str,
            file_type: InodeFileType,
            perm: InodePerm,
        ) -> Option<Arc<Dentry>> {
        self.fs.ext4.exclusive_access().fuse_create(self.ino as u64, name, (file_type.bits() | perm.bits()) as u32, 0, 0).unwrap();
        let ino = self.fs.ext4.exclusive_access().fuse_lookup(self.ino as u64, name).unwrap().ino;
        let inode = Arc::new(Ext4Inode {
            fs: self.fs.clone(),
            ino:  ino as u32,
            inner: unsafe { UPIntrFreeCell::new(Ext4InodeInner { fpos: 0 }) }
        });
        Some(
            Arc::new(Dentry::new(name,inode))
        ) 
    }

    /// 打开一个文件,根据create判断是否创建,默认创建普通文件
    fn open(&self, path: &str, create: bool) -> Option<Arc<dyn Inode>> {
        let mut par = self.ino;
        println!("[kernel] generic open path {:?}", path);
        let mut ino = 0u32;
        // if create {
        //     let inode_mode = InodeFileType::S_IFREG.bits();
        //     let inode_perm = (InodePerm::S_IREAD | InodePerm::S_IWRITE).bits();
        //     ino = self.fs.ext4.exclusive_access().create(2, "4G.txt", inode_mode | inode_perm).unwrap().inode_num;
        //     println!("[kernel] create ino {:x?}", ino);
        // } else {
            ino = self.fs.ext4.exclusive_access().generic_open(path, &mut par, create, InodeFileType::S_IFREG.bits(), &mut 0).expect("open generic file error");
            println!("[kernel] open ino {:x?}", ino);
        //}
        
        Some(Arc::new(Self {
            fs: self.fs.clone(),
            ino:  ino as u32,
            inner: unsafe { UPIntrFreeCell::new(Ext4InodeInner { fpos: 0 }) }
        }))
    }

    fn unlink(&self, name: &str) -> bool {
        self.fs.ext4.exclusive_access().fuse_unlink(self.ino as u64, name).is_ok()
    }

    fn symlink(&self, target: &crate::fs::path::Path, link_name: &str) -> bool {
        self.fs.ext4.exclusive_access().fuse_symlink(self.ino as u64, &target.path, link_name).is_ok()
    }

    fn ls(&self) -> Vec<String> {
        let v = self.fs.ext4.exclusive_access().dir_get_entries(self.ino).iter().map(|d| d.get_name()).collect();
        v
    }

}

impl File for Ext4Inode {
    fn readable(&self) -> bool {
        self.permissions().contains(InodePerm::S_IREAD)
    }
    fn writable(&self) -> bool {
        self.permissions().contains(InodePerm::S_IWRITE)
    }
    /// 读取文件
    fn read(&self, mut buf: crate::mm::UserBuffer) -> usize {
        let mut fpos = self.inner.exclusive_access().fpos;
        let mut total_read_bytes = 0usize;
        for slice in buf.buffers.iter_mut() {
            let read_size = self.read_at(fpos, *slice);
            if read_size == 0 {
                break;
            }
            fpos += read_size;
            total_read_bytes += read_size;
        }
        // 更新文件偏移
        self.inner.exclusive_access().fpos = fpos;
        total_read_bytes
    }
    /// 把 buf 中的内容写入到文件
    fn write(&self, buf: crate::mm::UserBuffer) -> usize {
        let mut fpos = self.inner.exclusive_access().fpos;
        let mut total_write_bytes = 0usize;
        for slice in buf.buffers.iter() {
            let write_size = self.write_at(fpos, *slice);
            assert_eq!(write_size, slice.len());
            fpos += write_size;
            total_write_bytes += write_size;
        }
        self.inner.exclusive_access().fpos = fpos;
        total_write_bytes
    }
}