use alloc::sync::Arc;
use super::defs::{InodeFileType, InodePerm, FileAttr};
use super::{OpenFlags, Path};
use crate::fs::vfs::Dentry;
use alloc::string::String;
use alloc::vec::Vec;
//use super::{VfsError, Metadata};

pub trait Inode: Send + Sync  {
    // --- 基础属性 ---
    /// 获取 inode 编号
    fn ino(&self) -> u64;
    
    /// 获取文件类型（普通文件/目录/符号链接等）
    fn file_type(&self) -> InodeFileType;

    /// 获取权限位
    fn permissions(&self) -> InodePerm;

    // --- 元数据操作 ---
    /// 获取文件属性
    fn get_attr(&self) -> Option<FileAttr>;
    
    /// 设置文件属性（支持部分更新）
    fn set_attr(&self, attr: &FileAttr);

    // --- 数据操作 ---
    /// 从指定偏移量读取数据
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> usize;
    
    /// 向指定偏移量写入数据
    fn write_at(&self, offset: usize, buf: &[u8]) -> usize;
    
    /// 清理inode
    fn clear(&self) {
        todo!()
    }

    // --- 目录操作 ---
    /// 查找子项
    fn lookup(&self, name: &str) -> Option<Arc<Dentry>>;

    /// 打开一个目录项
    fn open(&self, path: &str, create: bool) -> Option<Arc<dyn Inode>> {
        unimplemented!()
    }
    
    /// 创建并打开一个文件
    fn create(
        &self,
        name: &str,
        file_type: InodeFileType,
        perm: InodePerm,
    ) -> Option<Arc<Dentry>> {
        unimplemented!()
    }

    /// 删除目录项
    fn unlink(&self, name: &str) -> bool;

    // --- 扩展功能 ---
    /// 创建符号链接
    fn symlink(&self, target: &Path, link_name: &str) -> bool {
        unimplemented!()
    }
    
    /// 创建硬链接
    fn hardlink(&self, target: &dyn Inode, link_name: &str) -> isize {
        unimplemented!()
    }

    /// 重命名
    fn rename(&self, old_name: &str, new_parent: &dyn Inode, new_name: &str) -> isize {
        unimplemented!()
    }
    /// 截断文件到指定大小
    fn truncate(&self, size: usize) -> isize {
        unimplemented!()
    }
    /// list all inodes in the directory
    fn ls(&self) -> Vec<String>;
}


/* Inode Status */

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InodeStat {
    Dirty,
    Synced,
}