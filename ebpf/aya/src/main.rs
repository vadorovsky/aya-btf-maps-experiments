#![no_std]
#![no_main]

use aya_bpf::{
    cty::c_long,
    macros::{map, tracepoint},
    maps::HashMap,
    programs::TracePointContext,
    BpfContext,
};
use aya_log_ebpf::{error, info, WriteToBuf};

#[map]
static PID_MAP: HashMap<i32, i32> = HashMap::with_max_entries(1024, 0);

#[map]
static DUMMY_MAP: HashMap<i32, Dummy<u32, 1024>> = HashMap::with_max_entries(1024, 0);

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
    let parent_pid: i32 = unsafe {
        ctx.read_at(PARENT_PID_OFFSET).map_err(|e| {
            log_err(&ctx, e);
            e
        })?
    };
    let child_pid: i32 = unsafe {
        ctx.read_at(CHILD_PID_OFFSET).map_err(|e| {
            log_err2(&ctx, e);
            e
        })?
    };

    let dummy = Dummy {
        a: 0u32,
        b: [0; 1024],
    };
    info!(&ctx, "dummy: {}", dummy.a);

    if let Some(dummy) = unsafe { DUMMY_MAP.get(&parent_pid) } {
        info!(&ctx, "dummy: {}", dummy.a);
    }

    PID_MAP.insert(&parent_pid, &child_pid, 0)?;

    Ok(0)
}

// Just a dummy function which has a generic.
pub fn log_err<C: BpfContext>(ctx: &C, err: c_long) {
    error!(ctx, "error code: {}", err);
}

// Same, but with 2 generics.
pub fn log_err2<C: BpfContext, E: WriteToBuf>(ctx: &C, err: E) {
    error!(ctx, "error: {}", err);
}

// Keep tryin' with generics.
#[repr(C)]
pub struct Dummy<T, const N: usize> {
    pub a: T,
    pub b: [u8; N],
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
