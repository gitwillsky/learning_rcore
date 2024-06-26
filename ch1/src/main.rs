//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing form `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`].
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

use log::{debug, error, info, trace, warn};

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" { // 申明外部块，这样 rust 可以调用外部函数
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// the rust entry-point of os
#[no_mangle] // 不要修改此函数名称
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // begin addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data segment
        fn sdata(); // begin addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // begin addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }

    clear_bss();
    logging::init();
    println!("[kernel] Hello world!");
    trace!(
        "[kernel] .text ({:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata ({:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data ({:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss ({:#x}, {:#x}", sbss as usize, ebss as usize);

    sbi::shutdown(false)
}
