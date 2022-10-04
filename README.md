# fork

## Prerequisites

1. Install a rust stable toolchain: `rustup install stable`
1. Install a rust nightly toolchain: `rustup install nightly`

## Build patched LLVM

You need to use [this fork and branch of LLVM](https://github.com/dave-tucker/llvm-project/tree/btfdebug-segfault).

After you clone it somewhere and enter its directory, build LLVM with the
following commands:

WARNING! This example with debug build requires at least 64 GB RAM!

```
mkdir build
cd build

cmake -DCMAKE_BUILD_TYPE=Debug -DLLVM_PARALLEL_LINK_JOBS=1 -GNinja ../llvm/
ninja
```

If you encounter any problems with OOM killer or your machine being unusable,
you can trim down the number of ninja threads:

```
ninja -j[number_of_threads]
```

If you still have problems or have less than 64GB, try a release build:

```
cmake -DCMAKE_BUILD_TYPE=Release -DLLVM_PARALLEL_LINK_JOBS=1 -GNinja ../llvm/
ninja
```

## Install bpf-linker with the patched LLVM

You need to use [this fork and branch of bpf-linker](https://github.com/dave-tucker/bpf-linker/tree/bpf-v2).

After cloning and entering the directory, we need to install bpf-linker with
*system-llvm* feature and point to the patched build with `LLVM_SYS_150_PREFIX`
variable:

```
LLVM_SYS_150_PREFIX=[path_to_your_llvm_repo]/build cargo install --path . --no-default-features --features system-llvm bpf-linker
```

For example:

```
LLVM_SYS_150_PREFIX=/home/vadorovsky/repos/llvm-project/build cargo install --path . --no-default-features --features system-llvm bpf-linker
```

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag

## Build Userspace

```bash
cargo build
```

## Run

```bash
cargo xtask run
```
