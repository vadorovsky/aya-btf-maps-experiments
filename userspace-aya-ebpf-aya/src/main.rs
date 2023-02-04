use aya::maps::HashMap;
use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Bpf};
use log::info;
use tokio::{
    signal,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/fork"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/fork"
    ))?;
    let program: &mut TracePoint = bpf.program_mut("fork").unwrap().try_into()?;
    program.load()?;
    program.attach("sched", "sched_process_fork")?;

    let pid_map: HashMap<_, u32, u32> = HashMap::try_from(bpf.map_mut("PID_MAP").unwrap())?;
    info!("waiting 3 seconds");
    sleep(Duration::from_secs(3)).await;
    info!("the current map content:");
    for r in pid_map.iter() {
        let (k, v) = r?;
        info!("parent: {k}, child: {v}");
    }

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
