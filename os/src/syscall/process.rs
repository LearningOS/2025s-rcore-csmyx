//! Process management syscalls
use crate::mm::{translated_ptr_get, translated_ptr_get_mut, MapPermission, VirtAddr, MapType};
use crate::task::{
    change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_cnt, suspend_current_and_run_next, try_push_area, try_remove_area
};
use crate::timer::get_time_us;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    if let Some(ts) = translated_ptr_get_mut::<TimeVal>(current_user_token(), ts as *const u8) {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
        0
    } else {
        -1
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            if let Some(id) = translated_ptr_get::<u8>(current_user_token(), id as *const u8) {
                *id as isize
            } else {
                -1
            }
        }
        1 => {
            if let Some(id) = translated_ptr_get_mut::<u8>(current_user_token(), id as *const u8) {
                *id = data as u8;
                0
            } else {
                -1
            }
        }
        2 => {
            let cnt = get_syscall_cnt(id) as isize;
            // use for testing
            warn!("sys_trace: syscall id: {}, counter: {}", id, cnt);
            cnt
        }
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap");
    let start_va = VirtAddr::from(start);
    if !start_va.aligned() || prot == 0 || (prot >> 3) != 0 {
        return -1;
    }

    let end_va = VirtAddr::from(start + len);
    let mut map_perm = MapPermission::from_bits_truncate((prot << 1) as u8);
    map_perm |= MapPermission::U;

    match try_push_area(start_va, end_va, MapType::Framed, map_perm) {
        true => 0,
        false => -1,
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let start_va = VirtAddr::from(start);
    if !start_va.aligned() {
        return -1;
    }
    let end_va = VirtAddr::from(start + len);

    match try_remove_area(start_va, end_va) {
        true => 0,
        false => -1,
    }
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
