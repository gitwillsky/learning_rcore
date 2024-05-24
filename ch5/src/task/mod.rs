//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the operating system.
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.

mod context;
mod manager;
mod pid;
mod processor;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use core::num;

use alloc::{sync::Arc, vec::Vec};
pub use context::TaskContext;
use lazy_static::lazy_static;
use log::{debug, error, info};

use crate::{
    loader::{get_app_data, get_app_data_by_name, get_num_app},
    sbi::shutdown,
    sync::UPSafeCell,
    task::task::TaskStatus,
    trap::TrapContext,
};

use self::{processor::schedule, switch::__switch, task::TaskControlBlock};

pub const IDLE_PID: usize = 0;
pub use self::manager::add_task;
pub use processor::{
    current_task, current_trap_cx, current_user_token, run_tasks, take_current_task,
};

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

/// suspend current task, then run next task
pub fn suspend_current_and_run_next() {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // push back to ready queue
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// exit current task, then run next task
pub fn exit_current_and_run_next(exit_code: i32) {
    let task = take_current_task().unwrap();

    let pid = task.getpid();
    if pid == IDLE_PID {
        error!("[kernel] Idle process exit with exit_code {}", exit_code);
        if exit_code != 0 {
            shutdown(true);
        } else {
            shutdown(false);
        }
    }

    let mut inner = task.inner_exclusive_access();
    inner.task_status = TaskStatus::Zombie;
    inner.exit_code = exit_code;

    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }

    inner.children.clear();
    inner.memory_set.recycle_data_pages();
    drop(inner);
    drop(task);
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}
