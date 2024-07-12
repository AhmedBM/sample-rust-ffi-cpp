# Rust FFI Sample with C, Rust, CMake, and Corrosion

This repository contains a sample project demonstrating how to use Rust's Foreign Function Interface (FFI) to interact with C code. It also utilizes CMake as the build system and Corrosion as a tool for managing the Rust code. ALthough this was tested on Ubuntu 22.04, it should work on any platform including Windows.

## Prerequisites

Before getting started, make sure you have the following dependencies installed or just use the included [DevContainer](.devcontainer/devcontainer.json) by using the [Remote - Containers](https://aka.ms/vscode-remote/download/containers) VSCode extension.

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- CMake (=> 3.19): [Install CMake](https://cmake.org/install/)
- A C/C++ build toolchain

## Getting Started

To build and run the project, follow these steps:

1. Clone the repository:

    ```shell
    git clone https://github.com/AhmedBM/sample-rust-ffi-cpp.git
    ```

2. Navigate to the project directory:

    ```shell
    cd sample-rust-ffi-cpp
    mkdir build
    ```

3. Build the code in its own directory:

    ```shell
    mkdir build && cd build
    cmake ..
    cmake --build .
    ```

4. Run the sample Rust code:

    ```shell
    rust/rust_ffi
    ```
The sample Rust application will then print on the console thge values retreived from the C

```shell
The integer from C is: 7
The string from C is: Hello from C++!
```