use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Bpf};
use log::info;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut bpf = Bpf::load(include_bytes_aligned!("../../ebpf/libbpf/dist/fork.bpf.o"))?;
    let program: &mut TracePoint = bpf.program_mut("fork").unwrap().try_into()?;
    program.load()?;
    program.attach("sched", "sched_process_fork")?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
