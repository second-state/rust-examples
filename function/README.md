# Calling functions

Please [install the prerequisites](../README.md) first!

## Quick start with Docker

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-function:latest
Hello WasmEdge!
Howdy WasmEdge!
Hola WasmEdge!
WasmEdge 你好!
```

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
Now, we need to publish the container image to Docker Hub. The process is slightly different depending on how you plan to use the image.

### For Docker Desktop and containerd

For containerd based systems, such as the Docker Desktop and many flavors of Kubernetes,
you just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --platform wasi/wasm -t secondstate/rust-example-function .
... ...
$ docker push secondstate/rust-example-function
```

### For Podman, OpenShift and CRI-O

For `crun` based systems, such as Podman, OpenShift and CRI-O,
you will need to specify a special annotation for the image so that `crun` knowns to use WasmEdge to run it.
In this example, we push the image to Docker Hub with a `crun` tag.

```
$ sudo buildah build --annotation "module.wasm.image/variant=compat-smart" -t rust-example-function .
#
# make sure docker is install and running
# systemctl status docker
# to make sure regular user can use docker
# sudo usermod -aG docker $USER#
# newgrp docker

# You may need to use docker login to create the `~/.docker/config.json` for auth.
#
# docker login

$ sudo buildah push --authfile ~/.docker/config.json rust-example-string docker://docker.io/secondstate/rust-example-function:crun
```

