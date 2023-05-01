# Strings

Please [install the prerequisites](../README.md) first!

## Quick start with Docker

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-string:latest
... ART ...
Hello WasmEdge!
Howdy WasmEdge!
Howdy WasmEdge! -- from Texas
```

## Code

The [`src/main.rs`](src/main.rs) source code shows

* The Rust string can hold special characters and line breaks. The `art` variable is a complex string literal. It prints an ASCII art on the console.
* The `hello` variable is an immutable `&str` type. So, when you operate on it, the operation / function returns a `String` type.
* The `String` type variable `howdy` can now be manipulated and changed. It is very much like a `StringBuffer` in the Java world.

## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge ../target/wasm32-wasi/release/string.wasm
... ART ...
Hello WasmEdge!
Howdy WasmEdge!
Howdy WasmEdge! -- from Texas
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub.
You just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --platform wasi/wasm -t secondstate/rust-example-string .
... ...
$ docker push secondstate/rust-example-string
```
