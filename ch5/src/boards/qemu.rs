//! Constants used in rCore in qemu

/// RISC-V 架构要求处理器要有一个内置时钟，其频率一般低于 CPU 主频”。
/// 实际上在 CPU 的微架构内通常有多个不同频率的时钟源（比如各种晶振等），然后他们进行一些组合电路的处理又会得到更多不同频率的信号，
/// 不同的电路模块可能使用不同频率的时钟信号以满足同步需求。
/// CPU 的主体流水线所采用的时钟信号的频率是 CPU 的主频，但同时还有另一个用来计时的时钟模块（也就是上面提到的时钟）运行在另一个不同的频率。
/// 他们两个的另一个区别是，CPU 的时钟周期在mcycle寄存器中计数，而时钟的时钟周期在mtime寄存器中计数，因此这是两个独立且不同的频率
pub const CLOCK_FREQ: usize = 12_500_000;

/// From platform memory size
pub const MEMORY_END: usize = 0x88000000;

pub const MMIO: &[(usize, usize)] = &[
    (0x0010_0000, 0x00_2000), // VIRT_TEST/RTC in virt machine
];