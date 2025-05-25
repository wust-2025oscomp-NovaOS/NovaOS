use bitflags::bitflags;
/// 定义打开模式
bitflags! {
    pub struct OpenFlags: i32 {
        const O_RDONLY    = 0o0;
        const O_WRONLY    = 0o1;
        const O_RDWR      = 0o2;
        const O_CREAT     = 0o100;
        const O_EXCL      = 0o200;
        const O_NOCTTY    = 0o400;
        const O_TRUNC     = 0o1000;
        const O_APPEND    = 0o2000;
        const O_NONBLOCK  = 0o4000;
        const O_DSYNC     = 0o10000;
        const O_SYNC      = 0o4010000;
        const O_RSYNC     = 0o4010000;
        const O_DIRECTORY = 0o200000;
        const O_NOFOLLOW  = 0o400000;
        const O_CLOEXEC   = 0o2000000;

        // 一些常用的组合
        const O_ASYNC     = 0o20000;
        const O_DIRECT    = 0o40000;
        const O_LARGEFILE = 0o100000;
        const O_NOATIME   = 0o1000000;
        const O_PATH      = 0o10000000;
        const O_TMPFILE   = 0o20200000;
    }
}

/// 文件类型标志位
bitflags! {
    pub struct InodeFileType: u16 {
        /// 命名管道 (FIFO)
        const S_IFIFO = 0x1000;
        /// 字符设备
        const S_IFCHR = 0x2000;
        /// 目录
        const S_IFDIR = 0x4000;
        /// 块设备
        const S_IFBLK = 0x6000;
        /// 普通文件
        const S_IFREG = 0x8000;
        /// 套接字文件
        const S_IFSOCK = 0xC000;
        /// 符号链接
        const S_IFLNK = 0xA000;
    }
}

/// 文件权限标志位
bitflags! {
    pub struct InodePerm: u16 {
        /// 用户读权限 (0400)
        const S_IREAD = 0x0100;
        /// 用户写权限 (0200)
        const S_IWRITE = 0x0080;
        /// 用户执行权限 (0100)
        const S_IEXEC = 0x0040;
        /// SUID 位 (设置用户ID)
        const S_ISUID = 0x0800;
        /// SGID 位 (设置组ID)
        const S_ISGID = 0x0400;
    }
}

#[derive(Debug, Clone)]
pub struct FileAttr {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: u32,
    pub mtime: u32,
    pub ctime: u32,
    pub uid: u32,
    pub gid: u32,
    pub file_type: InodeFileType,
    pub perm: InodePerm,
    // ... 其他扩展字段
}