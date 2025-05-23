use super::BlockDevice;
use crate::drivers::bus::virtio::VirtioHal;
use crate::sync::{Condvar, UPIntrFreeCell};
use crate::task::schedule;
use crate::DEV_NON_BLOCKING_ACCESS;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use virtio_drivers::{BlkResp, RespStatus, VirtIOBlk, VirtIOHeader};
use super::BLK_SIZE;
use ext4_rs::BLOCK_SIZE;

#[allow(unused)]
const VIRTIO0: usize = 0x10008000;

pub struct VirtIOBlock {
    // 块驱动程序的virtio设备
    virtio_blk: UPIntrFreeCell<VirtIOBlk<'static, VirtioHal>>,
    condvars: BTreeMap<u16, Condvar>, // 用于等待io完成时，将线程挂起
}

impl VirtIOBlock {
    // fn read_block(&self, block_id: usize, buf: &mut [u8]) {
    //     let nb = *DEV_NON_BLOCKING_ACCESS.exclusive_access();
    //     if nb {
    //         let mut resp = BlkResp::default();
    //         let task_cx_ptr = self.virtio_blk.exclusive_session(|blk| {
    //             let token = unsafe { blk.read_block_nb(block_id, buf, &mut resp).unwrap() };
    //             self.condvars.get(&token).unwrap().wait_no_sched()
    //         });
    //         schedule(task_cx_ptr);
    //         assert_eq!(
    //             resp.status(),
    //             RespStatus::Ok,
    //             "Error when reading VirtIOBlk"
    //         );
    //     } else {
    //         self.virtio_blk
    //             .exclusive_access()
    //             .read_block(block_id, buf)
    //             .expect("Error when reading VirtIOBlk");
    //     }
    // }
    // fn write_block(&self, block_id: usize, buf: &[u8]) {
    //     let nb = *DEV_NON_BLOCKING_ACCESS.exclusive_access();
    //     if nb {
    //         let mut resp = BlkResp::default();
    //         let task_cx_ptr = self.virtio_blk.exclusive_session(|blk| {
    //             let token = unsafe { blk.write_block_nb(block_id, buf, &mut resp).unwrap() };
    //             self.condvars.get(&token).unwrap().wait_no_sched()
    //         });
    //         schedule(task_cx_ptr);
    //         assert_eq!(
    //             resp.status(),
    //             RespStatus::Ok,
    //             "Error when writing VirtIOBlk"
    //         );
    //     } else {
    //         self.virtio_blk
    //             .exclusive_access()
    //             .write_block(block_id, buf)
    //             .expect("Error when writing VirtIOBlk");
    //     }
    // }

    // fn handle_irq(&self) {
    //     self.virtio_blk.exclusive_session(|blk| {
    //         while let Ok(token) = blk.pop_used() {
    //             self.condvars.get(&token).unwrap().signal();
    //         }
    //     });
    // }
}

impl BlockDevice for VirtIOBlock {
    /// 读取offset开始的一整块数据
    fn read_offset(&self, offset: usize) -> Vec<u8> {
        // debug!("read_offset: offset = {:#x}", offset);
        // if offset % BLK_SIZE != 0 {
        //     panic!("VirtIOBlock::read_offset: offset must be aligned to BLK_SIZE");
        // } else {
        //println!("read start-----");
        let mut total_read_bytes = 0;
        let mut start = offset;
        let mut data: Vec<u8> = Vec::new(); 
        while total_read_bytes < BLOCK_SIZE {
            // 读取一块数据
            let mut buf = [0u8; BLK_SIZE];
            self.virtio_blk.exclusive_access().read_block(start / BLK_SIZE, &mut buf).expect("读取失败");
            // 计算当前扇区的偏移量
            let offset = start % BLK_SIZE; 
            // 读取长度为剩余数据的长度和扇区的剩余长度的最小值
            let len = core::cmp::min(BLOCK_SIZE - total_read_bytes, BLK_SIZE - offset);
            data.extend_from_slice(&buf[offset..offset + len]);
            start += len;
            total_read_bytes += len;
        }
        //println!("read end-----");
            // debug!("read_offset = {:#x}, buf = {:x?}", offset, buf);
        data
        //}
    }
    /// 从offset开始，将data写入到磁盘中
    fn write_offset(&self, offset: usize, data: &[u8]) {
        //debug!("write_offset: offset = {:#x}", offset);
        //     debug!("data len = {:#x}", data.len());
        let mut total_write_bytes = 0;
        let mut start = offset;
        while total_write_bytes < data.len() {
            let block_id = start / BLK_SIZE;
            let block_offset = start % BLK_SIZE;
            let mut buf = [0u8; BLK_SIZE];
            // 将数据的剩余长度和块的剩余长度进行比较，取最小值
            let copy_size = core::cmp::min(data.len() - total_write_bytes, BLK_SIZE - block_offset);
            // 先将该block数据读出来，和修改的部分进行拼接
            self.virtio_blk.exclusive_access().read_block(block_id, &mut buf).expect("读取失败");
            buf[block_offset..block_offset + copy_size]
            .copy_from_slice(&data[total_write_bytes..total_write_bytes + copy_size]);
            //将拼接后的数据写入磁盘
            self.virtio_blk.exclusive_access().write_block(block_id, &buf).expect("写回失败");
            total_write_bytes += copy_size;
            start += copy_size;
        }
    }
}

impl VirtIOBlock {
    pub fn new() -> Self {
        let virtio_blk = unsafe {
            //VIRTIO0 ，这是 Qemu模拟的virtio_blk设备中I/O寄存器的物理内存地址， 
            //VirtIOBlk 需要这个地址来对 VirtIOHeader 数据结构所表示的virtio-blk I/O控制寄存器进行读写操作，从而完成对某个具体的virtio-blk设备的初始化过程
            UPIntrFreeCell::new(
                VirtIOBlk::<VirtioHal>::new(&mut *(VIRTIO0 as *mut VirtIOHeader)).unwrap(),
            )
        };
        let mut condvars = BTreeMap::new();
        let channels = virtio_blk.exclusive_access().virt_queue_size();
        // 创建条件变量队列
        for i in 0..channels {
            let condvar = Condvar::new();
            condvars.insert(i, condvar);
        }
        Self {
            virtio_blk,
            condvars,
        }
    }
}
