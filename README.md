# RISC0 zkVM Minimal Verification Example

This repository demonstrates a minimal example of how to compile and use code in the RISC0 zkVM environment. The project focuses on verifying on-chain contract events from a Committee using zero-knowledge proofs.

## Project Overview

This project serves as a proof-of-concept to demonstrate the expected compilation process for the RISC0 zkVM. It consists of three main components that work together to create a zero-knowledge proof verifying that a specific event occurred on a blockchain.

## Current Status

**IMPORTANT NOTE**: This project is currently **not buildable** as is. It serves as a demonstration of which code needs to be compiled to the RISC0 platform. To make it work, modifications to the `sui-types` code are required. This repository is intended as a starting point to understand the compilation requirements and guide the necessary modifications.

## Project Structure

```
.
├── lib/             # Core verification logic library
├── methods/         # RISC0 zkVM methods
│   └── guest/       # zkVM guest program (runs inside the zkVM)
├── host/            # Host application (entry point)
└── ...
```

## Components

### 1. Core Library (`lib`)

The `lib` library contains the core functionality of the project. Its primary purpose is to verify events emitted by a specific contract on the blockchain. This verification is performed against a Committee's data.

Key features:
- Contains the verification logic for blockchain events
- Defines data structures for proofs and verification results
- Is designed to be compiled to both native and RISC0 target environments

### 2. zkVM Method (`methods/guest`)

This component is the RISC0 zkVM program that runs inside the zero-knowledge virtual machine. It:

- Is compiled to the RISC0 target architecture
- Imports the `lib` library and uses its verification logic
- Reads input data provided by the host
- Performs verification of on-chain events
- Returns a simple true/false result indicating verification success or failure

Note: This is not intended to be the final production zkVM program, but rather a minimal example to demonstrate compilation requirements and expected code usage patterns.

### 3. Host Application (`host`)

The host application serves as the entry point for the entire process:

- Prepares and formats the input data required for verification
- Executes the zkVM with the guest program
- Processes the verification result
- Generates and handles the zero-knowledge proof

## How It Works

1. The host application prepares input data containing Committee information and blockchain event data
2. This data is passed to the zkVM guest program
3. Inside the zkVM, the guest program uses the `lib` library to verify the event
4. The verification result is returned to the host application
5. The host processes this result and generates a zkproof that can be verified by third parties


## Environment Setup

1. Install rzup [link](https://dev.risczero.com/api/zkvm/install#installation-for-x86-64-linux-and-arm64-macos)
2. Switch RISC0 version to 1.2.6
```shell
rzup install # install default toolchain
rzup install cargo-risczero 1.2.6
rzup install r0vm 1.2.6
rzup install rust 1.85.0

# check if the installed components has switched to the expected version
rzup show
```

## ZK Program 

The ZK program is located at [guest](./methods/guest/), and it is compiled by [build.rs](./methods/build.rs). 

To compile, simply run:
```
cargo build -p methods
```

## Dependency Inspection

The zkVM compilation uses `risc0` toolchain. To have a better inspection of the dependencies, you can use:
```
cargo +risc0 tree --manifest-path methods/guest/Cargo.toml
```

## Next Steps

This project illustrates which code needs to be compiled to the RISC0 platform. The next step is to modify the `sui-types` code to make it compatible with the RISC0 target. Once those modifications are complete, this minimal example can be used as a foundation for building the full verification system.