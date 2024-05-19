//! Implementation of [`TrapContext`]

use riscv::register::sstatus::{self, Sstatus, SPP};

/// Trap Context
#[repr(C)]
pub struct TrapContext {
    /// general regs [0..31]
    pub x: [usize; 32],
    /// Supervisor Status 寄存器，包含处理器状态控制的关键标志位 
    pub sstatus: Sstatus,
    /// CSR sepc 指向发生异常的指令地址
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR status
        // 设置异常发生时处理器权限模式为 User 
        sstatus.set_spp(SPP::User); 
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
