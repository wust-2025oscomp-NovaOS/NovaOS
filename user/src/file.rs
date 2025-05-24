use super::*;
use bitflags::bitflags;
// 定义打开模式
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

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}
pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open(path, flags.bits())
}
pub fn close(fd: usize) -> isize {
    sys_close(fd)
}
pub fn pipe(pipe_fd: &mut [usize]) -> isize {
    sys_pipe(pipe_fd)
}
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
