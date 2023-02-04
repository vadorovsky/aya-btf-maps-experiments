use std::{
    ffi::{c_char, CString},
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::Duration,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    let bpf = unsafe {
        libbpf_sys::bpf_object__open(
            CString::new("ebpf/fork-ebpf-libbpf/dist/fork.bpf.o")?.as_ptr() as *const c_char,
        )
    };

    let res = unsafe { libbpf_sys::bpf_object__load(bpf) };
    if res != 0 {
        return Err(anyhow::anyhow!("failed to load bpf object: {}", res));
    }

    let prog = unsafe {
        libbpf_sys::bpf_object__find_program_by_name(
            bpf,
            CString::new("fork")?.as_ptr() as *const c_char,
        )
    };

    let link = unsafe {
        libbpf_sys::bpf_program__attach_tracepoint(
            prog,
            CString::new("sched")?.as_ptr() as *const c_char,
            CString::new("sched_process_fork")?.as_ptr() as *const c_char,
        )
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(500))
    }
    println!("Exiting...");

    unsafe { libbpf_sys::bpf_link__detach(link) };

    Ok(())
}
