The project is an new risc0 project with only one line change [Cargo.toml](./methods/guest/Cargo.toml).
We need to compile `sui-types` into zkvm, so than we can verify the integrity.
The target platform of the `risc0` is like a bare-metal system, without network/filesystem support.


## Environment Setup

1. install rzup [link](https://dev.risczero.com/api/zkvm/install#installation-for-x86-64-linux-and-arm64-macos)
2. switch risc0 version to 1.2.6
```shell
rzup install # install default toolchain
rzup install cargo-risczero 1.2.6
rzup install r0vm 1.2.6
rzup install rust 1.85.0

# check if the installed components has switched to the expected version
rzup show
```


## zk program 

The zk program is located at [guest](./methods/guest/), and it is compiled by [build.rs](./methods/build.rs). 

To compile, simply run:
```
cargo build -p methods
```

## Dependency Inspect

The zkvm compilation uses `risc0` toolchain. To have a better inspection of the dependencies. You can use:
```
cargo +risc0 tree --manifest-path methods/guest/Cargo.toml
```
