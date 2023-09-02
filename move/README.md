# Ownership and borrowing references

Please [install the prerequisites](../README.md) first!

## Code

The [`src/main.rs`](src/main.rs) source code shows

* When the `hello` string variable is passed into the `take()` function, it is no longer available outside of `take()`.
* You can create a `clone()` of the `hello` string variable to pass to `take()`. This way, the original `hello` is still available outside of `take()`.
* You can also pass a reference of the `hello` string to the `borrow()` function. Since the `borrow()` function only borrowed a reference of this variable, the original `hello` is still available outside of `borrow()`.
  * The `&string` reference can be automatically cast to an immutable `&str`. So, the `borrow()` function can also take a `&str` as argument.
* If the `hello` string is mutable and the passed reference is also mutable, you can make changes to the string through its reference inside the `borrow_mut()` function and the `hello` string outside of the function will be changed too. That is the "side effect" of pass-by-reference.


## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/move.wasm
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub.
You just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --provenance=false --platform wasi/wasm -t secondstate/rust-example-move .
... ...
$ docker push secondstate/rust-example-move
```

Then, with Docker Desktop and [Wasm support enabled](https://wasmedge.org/docs/start/build-and-run/docker_wasm), you can run it.

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-move:latest
```

