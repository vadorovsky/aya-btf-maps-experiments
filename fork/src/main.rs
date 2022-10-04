use std::ffi::{c_char, CString};

use clap::Parser;
use log::info;

#[derive(Debug, Parser)]
struct Opt {}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();

    #[cfg(debug_assertions)]
    let bpf = unsafe {
        libbpf_sys::bpf_object__open(
            CString::new("target/bpfel-unknown-none/debug/fork")?.as_ptr() as *const c_char,
        )
    };
    #[cfg(not(debug_assertions))]
    let bpf = unsafe {
        libbpf_sys::bpf_object__open(
            CString::new("target/bpfel-unknown-none/release/fork")?.as_ptr() as *const c_char,
        )
    };

    unsafe { libbpf_sys::bpf_object__load(bpf) };

    // info!("Waiting for Ctrl-C...");
    // signal::ctrl_c().await?;
    // info!("Exiting...");

    Ok(())
}
