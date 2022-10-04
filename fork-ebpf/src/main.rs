#![no_std]
#![no_main]

use core::marker::PhantomData;

use aya_bpf::{
    bindings::bpf_map_type::BPF_MAP_TYPE_HASH,
    helpers::{bpf_map_update_elem, bpf_printk},
    macros::tracepoint,
    programs::TracePointContext,
};

#[repr(C)]
pub struct MapDef<K, V> {
    r#type: u32,
    // r#type: *const u32,
    max_entries: u32,
    map_flags: u32,
    key: PhantomData<K>,
    value: PhantomData<V>,
}

unsafe impl<K: Sync, V: Sync> Sync for MapDef<K, V> {}

#[link_section = ".maps"]
#[export_name = "PID_MAP"]
static mut PID_MAP: MapDef<i32, i32> = MapDef {
    r#type: BPF_MAP_TYPE_HASH,
    // r#type: &BPF_MAP_TYPE_HASH,
    max_entries: 1024,
    map_flags: 0,
    key: PhantomData,
    value: PhantomData,
};

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

    let ret = unsafe {
        bpf_map_update_elem(
            &mut PID_MAP as *mut MapDef<i32, i32> as *mut _,
            &parent_pid as *const _ as *const _,
            &child_pid as *const _ as *const _,
            0,
        )
    };
    if ret != 0 {
        return Err(0);
    }

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
