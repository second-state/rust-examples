# Calling functions

Please [install the prerequisites](../README.md) first!

## Code

The [`src/main.rs`](src/main.rs) source code shows

* The `greet()` function creates a vector of `Greeting` struct instances. It takes an `Lang` as input argument to look up the matching `Greeting` and returns its message.
* The `greet()` function returns a `Result` struct. The struct has two fields.
  * The first field is a `String`. It is the return value from the function. Inside the function, we return `Ok(String)` to indicate success and wraps a string return value.
  * The second field is also a `String`. It is the error message when the function fails. We return `Err(String)` from inside the function to indicate failure and wraps an error message.
* When we call the `greet()` function from `main()`, we use the `?` notation to unwrap the `Result`. The `greet(Lang::English)?` provides the string value of the English greeting. If the English language is not supported in the `greet()` function, it will unwrap into a error and cause the `main()` function to return the same error.


## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/function.wasm
Hello WasmEdge!
Howdy WasmEdge!
Hola WasmEdge!
WasmEdge 你好!
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub.
You just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --provenance=false --platform wasi/wasm -t secondstate/rust-example-function .
... ...
$ docker push secondstate/rust-example-function
```
Then, with Docker Desktop and [Wasm support enabled](https://wasmedge.org/docs/start/build-and-run/docker_wasm), you can run it.

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-function:latest
Hello WasmEdge!
Howdy WasmEdge!
Hola WasmEdge!
WasmEdge 你好!
```

