pub mod inode;
mod defs;
pub mod fs;
mod dentry;
use super::path::Path;

pub use defs::*;
pub use fs::*;
pub use inode::*;
pub use dentry::*;


// /// 定义错误类型
// #[derive(Debug)]
// pub enum VfsError {
//     NotFound,        // 文件不存在
//     PermissionDenied, // 权限不足
//     NotDirectory,     // 非目录操作
//     // ... 其他错误类型
// }