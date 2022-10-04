#![no_std]
#![no_main]

use aya_bpf::{
    bindings::bpf_map_type::BPF_MAP_TYPE_HASH,
    helpers::{bpf_map_update_elem, bpf_printk},
    macros::tracepoint,
    programs::TracePointContext,
};
use aya_btf_macros::btf_map;

btf_map!(PID_MAP, BPF_MAP_TYPE_HASH, u32, u32, 1024, 0);

#[tracepoint(name = "fork")]
pub fn fork(ctx: TracePointContext) -> u32 {
    match try_fork(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_fork(ctx: TracePointContext) -> Result<u32, u32> {
    // Load the pointer to the filename. The offset value can be found running:
    // sudo cat /sys/kernel/debug/tracing/events/sched/sched_process_fork/format
    const PARENT_PID_OFFSET: usize = 24;
    const CHILD_PID_OFFSET: usize = 44;
    let parent_pid: i32 = unsafe { ctx.read_at(PARENT_PID_OFFSET).map_err(|e| e as u32)? };
    let child_pid: i32 = unsafe { ctx.read_at(CHILD_PID_OFFSET).map_err(|e| e as u32)? };

    // let ret = unsafe {
    //     bpf_map_update_elem(
    //         &mut PID_MAP as *mut MapDef<i32, i32> as *mut _,
    //         &parent_pid as *const _ as *const _,
    //         &child_pid as *const _ as *const _,
    //         0,
    //     )
    // };
    // if ret != 0 {
    //     return Err(0);
    // }

    unsafe {
        bpf_printk!(
            b"fork! parent pid: {}, child pid: {}",
            parent_pid,
            child_pid
        );
    }
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
