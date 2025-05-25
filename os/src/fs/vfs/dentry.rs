use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use super::inode::Inode;
/// 目录项，用于缓存已经访问的文件位置
pub struct Dentry {
    name:  String,         // 名称
    inode: Arc<dyn Inode>, // 对应的inode
}

// lazy_static! {
//     pub static ref ROOT_DENTRY = Vec::new();
// }

impl Dentry {
    pub fn new(name: &str, inode: Arc<dyn Inode>) -> Self {
        Self {
            name: name.to_string(),
            inode,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// 返回对应的inode
    pub fn inode(&self) -> Arc<dyn Inode> {
        Arc::clone(&self.inode)
    }
}
