#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{close, open, read, OpenFlags};

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    println!("argc = {}", argc);
    for (i, arg) in argv.iter().enumerate() {
        println!("argv[{}] = {}", i, arg);
    }
    assert!(argc == 2);
    let fd = open(argv[1], OpenFlags::O_RDONLY);
    if fd == -1 {
        panic!("Error occurred when opening file");
    }
    let fd = fd as usize;
    let mut buf = [0u8; 256];
    loop {
        let size = read(fd, &mut buf) as usize;
        if size == 0 {
            break;
        }
        print!("{}", core::str::from_utf8(&buf[..size]).unwrap());
    }
    close(fd);
    0
}
