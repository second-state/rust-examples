# Data structures

Please [install the prerequisites](../README.md) first!

## Quick start with Docker

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-struct:latest
English Hello WasmEdge!
Spanish Hola WasmEdge!
Texan Howdy WasmEdge!
Chinese WasmEdge 你好!
```

## Code

The [`src/main.rs`](src/main.rs) source code shows

* The `Lang` enum defines several language choices. It is annotated with the `#[derive(Debug)]` macro so that it can be printed later in the program.
* The `Greeting` struct holds both a `String` type `message` and a `Lang` type lang. Rust struct could also have internal functions to operate on its data fields, which makes them similar to Java objects.
* The `main()` program creates a `Vector` to hold the `Greeting` struct instances. The `Vector::push()` function is used to add elemements to the vector.
* We iterate over the vector and print out each element.


## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/struct.wasm
English Hello WasmEdge!
Spanish Hola WasmEdge!
Texan Howdy WasmEdge!
Chinese WasmEdge 你好!
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub. The process is slightly different depending on how you plan to use the image.

### For Docker Desktop and containerd

For containerd based systems, such as the Docker Desktop and many flavors of Kubernetes,
you just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --platform wasi/wasm -t secondstate/rust-example-struct .
... ...
$ docker push secondstate/rust-example-struct
```

### For Podman, OpenShift and CRI-O

For `crun` based systems, such as Podman, OpenShift and CRI-O,
you will need to specify a special annotation for the image so that `crun` knowns to use WasmEdge to run it.
In this example, we push the image to Docker Hub with a `crun` tag.

```
$ sudo buildah build --annotation "module.wasm.image/variant=compat-smart" -t rust-example-struct .
#
# make sure docker is install and running
# systemctl status docker
# to make sure regular user can use docker
# sudo usermod -aG docker $USER#
# newgrp docker

# You may need to use docker login to create the `~/.docker/config.json` for auth.
#
# docker login

$ sudo buildah push --authfile ~/.docker/config.json rust-example-string docker://docker.io/secondstate/rust-example-struct:crun
```

