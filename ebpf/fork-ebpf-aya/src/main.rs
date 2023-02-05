#![no_std]
#![no_main]

use aya_bpf::{cty::c_long, helpers::bpf_printk, macros::tracepoint, programs::TracePointContext};
use aya_btf_map::{macros::btf_map, HashMap};

#[btf_map]
static mut PID_MAP: HashMap<i32, i32, 1024> = HashMap::new();

#[tracepoint(name = "fork")]
pub fn fork(ctx: TracePointContext) -> u32 {
    match try_fork(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret as u32,
    }
}

fn try_fork(ctx: TracePointContext) -> Result<u32, c_long> {
    // Load the pointer to the filename. The offset value can be found running:
    // sudo cat /sys/kernel/debug/tracing/events/sched/sched_process_fork/format
    const PARENT_PID_OFFSET: usize = 24;
    const CHILD_PID_OFFSET: usize = 44;
    let parent_pid: i32 = unsafe { ctx.read_at(PARENT_PID_OFFSET)? };
    let child_pid: i32 = unsafe { ctx.read_at(CHILD_PID_OFFSET)? };

    unsafe { PID_MAP.insert(&parent_pid, &child_pid, 0)? };

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
