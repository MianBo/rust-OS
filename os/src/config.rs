#[allow(unused)]

pub const USER_STACK_SIZE: usize = 4096 * 2; // 用户栈
pub const KERNEL_STACK_SIZE: usize = 4096 * 2; // 内核栈
pub const KERNEL_HEAP_SIZE: usize = 0x100_0000; // 内核堆
pub const PAGE_SIZE: usize = 0x1000; // 页表
pub const PAGE_SIZE_BITS: usize = 0xc; // 页表项索引位

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1; // 跳板页
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE; // Trap上下文页

pub use crate::board::{CLOCK_FREQ, MEMORY_END, MMIO}; // 板级配置
