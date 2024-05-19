//! Implementation of [`PageTableEntry`] and [`PageTable`].

use bitflags::bitflags;
use alloc::vec::{self, Vec};
use super::address::PhysPageNum;

bitflags! {
    /// page table entry flags
    pub struct PTEFlags: u8 {
        const V = 1 << 0; // flags 其余字段是否有效
        const R = 1 << 1; // 可读
        const W = 1 << 2; // 可写
        const X = 1 << 3; // 可执行
        const U = 1 << 4; // 是否为用户页
        const G = 1 << 5; // 表示该映射是否存在于所有虚拟地址空间，借助该信息，硬件可提升地址翻译的性能。G 位通常只用于那些属于操作系统的页。
        const A = 1 << 6; // 自上次清除 A 位以来，该页是否被访问过
        const D = 1 << 7; // 自上次清除 D 位以来，该页是否被写入过
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
/// 页表项定义
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }

    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn ppn(&self) -> PhysPageNum {
        // 43 bits
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

/// 页表
pub struct PageTable {
    root_ppn: PhysPageNum,
}
