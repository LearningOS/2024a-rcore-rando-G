//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM, mm::{PageTableEntry, PhysAddr, VirtAddr}, task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
    }, timer::get_time_us
};

// use page_table::PageTable;
use mm::PageTable;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}
//由虚拟地址查物理地址
pub fn translated_physical_address(token: usize,ptr: *const u8) ->usize{
    let page_table=PageTable::from_token(token);
    let mut va=VirtAddr::from(ptr as usize);
    let ppn=page_table.find_pte(va.floor()).unwrap().ppn();
    PhysAddr::from(ppn).0+va.page_offset()
}

pub fn current_translated_physical_address(ptr: *const u8) -> usize {
    let token=crate::task::current_user_token();
    translated_physical_address(token,ptr)
}


/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let us=get_time_us();
    let ts=current_translated_physical_address(_ts as *const u8) as * mut TimeVal;
    unsafe {
        *ts=TimeVal{
            sec: us/1_000_000,
            usec: us%1_000_000,
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    let ti=current_translated_physical_address(_ti as *const u8) as *mut TaskInfo;
    unsafe {
        *ti=TaskInfo{
            status:get_current_status(),
            syscall_times:get_syscall_times(),
            time:(get_time_us()-get_current_start_time())/1_000_000,
        };
    }
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
