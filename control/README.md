# Control flows

Please [install the prerequisites](../README.md) first!

## Quick start with Docker

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-control:latest
Hello WasmEdge!
Howdy WasmEdge!
Hola WasmEdge!
Bonjour WasmEdge!
guten tag WasmEdge!
WasmEdge 你好!
こんにちは  WasmEdge!
Salve WasmEdge!
Salve WasmEdge!
```

## Code

The [`src/main.rs`](src/main.rs) source code shows

* The `for` loop starts from value `0` and repeats `10` times, each increasing by `1`.
* The `match` clause matches the control variable to specific values and branches to the corresponding statements. If control variable does not match any listed value, it will match to the `_` branch.


## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/control.wasm
Hello WasmEdge!
Howdy WasmEdge!
Hola WasmEdge!
Bonjour WasmEdge!
guten tag WasmEdge!
WasmEdge 你好!
こんにちは  WasmEdge!
Salve WasmEdge!
Salve WasmEdge!
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub.
You just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --platform wasi/wasm -t secondstate/rust-example-control .
... ...
$ docker push secondstate/rust-example-control
```

