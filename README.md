# BTF map support in Aya

This is an attempt to make BTF maps working in Aya eBPF.

The work onn BTF map support is tracked and discussed in:

* [this Github issue](https://github.com/aya-rs/aya/issues/351)
* [this Discord tread](https://discord.com/channels/855676609003651072/1026937450624450652)

To make sure that we have to use BTF maps, this project is using Aya in the
eBPF crate and libbpf-sys in the userspace crate.

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

## Debug info

The main difference between this project and all the current Aya examples is
that it generates the full debug info in eBPF crate in all profiles. It's
necessary for generating BTF. So please note the:

```
debug = 2
```

option in [Cargo.toml](https://github.com/vadorovsky/aya-btf-maps/blob/main/fork-ebpf/Cargo.toml)
in all profiles.

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag

## Ensuring that debug_info and BTF are there

```
$ readelf -S ./target/bpfel-unknown-none/debug/fork
There are 26 section headers, starting at offset 0x22710:

Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
[...]
  [ 5] .maps             PROGBITS         0000000000000000  000001c8
[...]
  [ 9] .debug_info       PROGBITS         0000000000000000  0000092b
       0000000000004e99  0000000000000000           0     0     1
[...]
  [17] .BTF              PROGBITS         0000000000000000  000174c0
       0000000000000697  0000000000000000           0     0     4
  [18] .rel.BTF          REL              0000000000000000  00022338
       0000000000000010  0000000000000010   I      25    17     8
  [19] .BTF.ext          PROGBITS         0000000000000000  00017b58
       0000000000000220  0000000000000000           0     0     4
  [20] .rel.BTF.ext      REL              0000000000000000  00022348
       00000000000001f0  0000000000000010   I      25    19     8
  [21] .debug_frame      PROGBITS         0000000000000000  00017d78
       0000000000000058  0000000000000000           0     0     8
[...]
```

If those sections aren't there, it means that something went wrong with building
LLVM or/and bpf-linker.

You can also dump BTF info with:

```
$ bpftool btf dump file ./target/bpfel-unknown-none/debug/fork
[1] PTR '*const [i32; 1]' type_id=3
[2] INT 'i32' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
[3] ARRAY '(anon)' type_id=2 index_type_id=4 nr_elems=1
[4] INT '__ARRAY_SIZE_TYPE__' size=4 bits_offset=0 nr_bits=32 encoding=(none)
[5] PTR '*const u32' type_id=6
[6] INT 'u32' size=4 bits_offset=0 nr_bits=32 encoding=(none)
[7] PTR '*const [i32; 1024]' type_id=8
[8] ARRAY '(anon)' type_id=2 index_type_id=4 nr_elems=1024
[9] PTR '*const [i32; 0]' type_id=10
[10] ARRAY '(anon)' type_id=2 index_type_id=4 nr_elems=0
[11] STRUCT '_ty_PID_MAP' size=40 vlen=5
        'type' type_id=1 bits_offset=0
        'key' type_id=5 bits_offset=64
        'max_entries' type_id=7 bits_offset=192
        'map_flags' type_id=9 bits_offset=256
[12] VAR 'PID_MAP' type_id=11, linkage=global
[13] PTR '*mut core::ffi::c_void' type_id=14
[14] ENUM 'c_void' size=1 vlen=2
        '__variant1' val=0
        '__variant2' val=1
[15] FUNC_PROTO '(anon)' ret_type_id=6 vlen=1
        'ctx' type_id=13
[16] FUNC 'fork' type_id=15 linkage=global
[17] PTR '*mut u8' type_id=18
[18] INT 'u8' size=1 bits_offset=0 nr_bits=8 encoding=(none)
[19] INT 'usize' size=8 bits_offset=0 nr_bits=64 encoding=(none)
[20] FUNC_PROTO '(anon)' ret_type_id=0 vlen=3
        's' type_id=17
        'c' type_id=2
        'n' type_id=19
[21] FUNC 'memset' type_id=20 linkage=global
[22] FUNC_PROTO '(anon)' ret_type_id=0 vlen=3
        'dest' type_id=17
        'src' type_id=17
        'n' type_id=19
[23] FUNC 'memcpy' type_id=22 linkage=global
[24] DATASEC '.maps' size=0 vlen=1
        type_id=12 offset=0 size=40 (VAR 'PID_MAP')
```

## Build Userspace

```bash
cargo build
```

## Run

```bash
cargo xtask run
```
