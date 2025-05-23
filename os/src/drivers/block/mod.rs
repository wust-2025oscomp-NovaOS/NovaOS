mod virtio_blk;

pub use virtio_blk::VirtIOBlock;

use crate::board::BlockDeviceImpl;
use alloc::sync::Arc;
//use easy_fs::BlockDevice;
use ext4_rs::BlockDevice;
use lazy_static::*;

pub const BLK_SIZE: usize = 512;

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(BlockDeviceImpl::new());
}

#[allow(unused)]
pub fn block_device_test() {
    let block_device = BLOCK_DEVICE.clone();
    let mut write_buffer = [0u8; 512];
    for i in 0..512 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_offset(i as usize, &write_buffer);
        let data = block_device.read_offset(i as usize);
        assert_eq!(write_buffer, data.as_slice());
    }
    println!("block device test passed!");
}
