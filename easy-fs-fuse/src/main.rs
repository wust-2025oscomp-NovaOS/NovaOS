use clap::{App, Arg};
//use easy_fs::{BlockDevice, EasyFileSystem};
use ext4_rs::{BlockDevice, BLOCK_SIZE, Ext4, ROOT_INODE, InodeFileType};
use std::fs::{read_dir, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Arc;
use std::sync::Mutex;

// const BLOCK_SZ: usize = 512;

#[derive(Debug)]
pub struct Disk {}

impl BlockDevice for Disk {
    fn read_offset(&self, offset: usize) -> Vec<u8> {
        use std::fs::OpenOptions;
        use std::io::{Read, Seek};
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("fs.img")
            .unwrap();
        let mut buf = vec![0u8; BLOCK_SIZE as usize];
        let _r = file.seek(std::io::SeekFrom::Start(offset as u64));
        let _r = file.read_exact(&mut buf);

        buf
    }

    fn write_offset(&self, offset: usize, data: &[u8]) {
        use std::fs::OpenOptions;
        use std::io::{Seek, Write};
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("fs.img")
            .unwrap();

        let _r = file.seek(std::io::SeekFrom::Start(offset as u64));
        let _r = file.write_all(&data);
    }

    fn handle_irq(&self) {
        println!("Disk IRQ!");
    }
    // fn read_offset(&self, offset: usize) -> Vec<u8> {
    //     let mut file = self.0.lock().unwrap();
    //     let mut buf = [0; BLOCK_SIZE];
    //     file.seek(SeekFrom::Start(offset as u64))
    //         .expect("Error when seeking!");
    //     assert_eq!(file.read(&mut buf).unwrap(), BLOCK_SIZE, "Not a complete block!");
    //     buf.to_vec()
    // }
        
    

    // fn write_offset(&self, offset: usize, data: &[u8]) {
    //     let mut file = self.0.lock().unwrap();
    //     file.seek(SeekFrom::Start(offset as u64))
    //         .expect("Error when seeking!");
    //     //assert_eq!(file.write(data).unwrap(), BLOCK_SIZE, "Not a complete block!");
    // }
        
    
}

fn main() {
    //println!("--------EasyFileSystem started!--------------");
    easy_fs_pack().expect("Error when packing easy-fs!");
    //println!("--------EasyFileSystem packed!--------------");
}

fn easy_fs_pack() -> std::io::Result<()> {
    let matches = App::new("EasyFileSystem packer")
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .takes_value(true)
                .help("Executable source dir(with backslash)"),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .help("Executable target dir(with backslash)"),
        )
        .get_matches();
    let src_path = matches.value_of("source").unwrap();
    let target_path = matches.value_of("target").unwrap();
    println!("src_path = {}\ntarget_path = {}", src_path, target_path);
    // let block_file = Arc::new(BlockFile(Mutex::new({
    //     let f = OpenOptions::new()
    //         .read(true)
    //         .write(true)
    //         .create(true)
    //         .open(format!("{}{}", target_path, "fs.img"))?;
    //     // f.set_len(32 * 2048 * 512).unwrap();
    //     f
    // })));


    let disk = Arc::new(Disk {});
    let ext4 = Ext4::open(disk);
    // 32MiB, at most 4095 files
    //let ext4 = Ext4::open(block_file);
    // let root_inode = Arc::new(Ext4::(&efs));
    let apps: Vec<_> = read_dir(src_path)
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    for app in apps {
        // load app data from host file system
        let mut host_file = File::open(format!("{}{}", target_path, app)).unwrap();
        let mut all_data: Vec<u8> = Vec::new();
        host_file.read_to_end(&mut all_data).unwrap();
        // create a file in easy-fs
        //println!("开始创建文件");
        let inode_ref = ext4.create(ROOT_INODE, &app, InodeFileType::S_IFREG.bits()).expect("文件创建失败");
        // write data to easy-fs
        //println!("开始写入数据");
        ext4.write_at(inode_ref.inode_num, 0, all_data.as_slice()).expect("数据写入失败");
    }
    // list apps
    // for app in root_inode.ls() {
    //     println!("{}", app);
    // }
    Ok(())
}

#[test]
fn efs_test() -> std::io::Result<()> {
    let block_file = Arc::new(BlockFile(Mutex::new({
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("target/fs.img")?;
        f.set_len(8192 * 512).unwrap();
        f
    })));
    EasyFileSystem::create(block_file.clone(), 4096, 1);
    let efs = EasyFileSystem::open(block_file.clone());
    let root_inode = EasyFileSystem::root_inode(&efs);
    root_inode.create("filea");
    root_inode.create("fileb");
    for name in root_inode.ls() {
        println!("{}", name);
    }
    let filea = root_inode.find("filea").unwrap();
    let greet_str = "Hello, world!";
    filea.write_at(0, greet_str.as_bytes());
    //let mut buffer = [0u8; 512];
    let mut buffer = [0u8; 233];
    let len = filea.read_at(0, &mut buffer);
    assert_eq!(greet_str, core::str::from_utf8(&buffer[..len]).unwrap(),);

    let mut random_str_test = |len: usize| {
        filea.clear();
        assert_eq!(filea.read_at(0, &mut buffer), 0,);
        let mut str = String::new();
        use rand;
        // random digit
        for _ in 0..len {
            str.push(char::from('0' as u8 + rand::random::<u8>() % 10));
        }
        filea.write_at(0, str.as_bytes());
        let mut read_buffer = [0u8; 127];
        let mut offset = 0usize;
        let mut read_str = String::new();
        loop {
            let len = filea.read_at(offset, &mut read_buffer);
            if len == 0 {
                break;
            }
            offset += len;
            read_str.push_str(core::str::from_utf8(&read_buffer[..len]).unwrap());
        }
        assert_eq!(str, read_str);
    };

    random_str_test(4 * BLOCK_SZ);
    random_str_test(8 * BLOCK_SZ + BLOCK_SZ / 2);
    random_str_test(100 * BLOCK_SZ);
    random_str_test(70 * BLOCK_SZ + BLOCK_SZ / 7);
    random_str_test((12 + 128) * BLOCK_SZ);
    random_str_test(400 * BLOCK_SZ);
    random_str_test(1000 * BLOCK_SZ);
    random_str_test(2000 * BLOCK_SZ);

    Ok(())
}
