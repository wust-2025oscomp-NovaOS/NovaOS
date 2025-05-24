#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{close, open, read, write, OpenFlags};

#[no_mangle]
pub fn main() -> i32 {
    println!("filetest_simple start!");
    let test_str = "Hello, world!";
    let filea = "filea\0";
    let fd = open(filea, OpenFlags::O_CREAT | OpenFlags::O_WRONLY);
    println!("filea opened!");
    assert!(fd > 0);
    let fd = fd as usize;
    write(fd, test_str.as_bytes());
    println!("filea written!");
    close(fd);
    println!("filea closed!");

    let fd = open(filea, OpenFlags::O_RDONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let mut buffer = [0u8; 100];
    let read_len = read(fd, &mut buffer) as usize;
    println!("filea read!");
    close(fd);

    assert_eq!(test_str, core::str::from_utf8(&buffer[..read_len]).unwrap(),);
    println!("file_test passed!");
    0
}
