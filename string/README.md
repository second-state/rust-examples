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
$ wasmedge target/wasm32-wasi/release/string.wasm
... ART ...
Hello WasmEdge!
Howdy WasmEdge!
Howdy WasmEdge! -- from Texas
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub. The process is slightly different depending on how you plan to use the image.

### For Docker Desktop and containerd

For containerd based systems, such as the Docker Desktop and many flavors of Kubernetes,
you just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --platform wasi/wasm -t secondstate/rust-example-string .
... ...
$ docker push secondstate/rust-example-string
```

### For Podman, OpenShift and CRI-O

For `crun` based systems, such as Podman, OpenShift and CRI-O,
you will need to specify a special annotation for the image so that `crun` knowns to use WasmEdge to run it.
In this example, we push the image to Docker Hub with a `crun` tag.

```
$ sudo buildah build --annotation "module.wasm.image/variant=compat-smart" -t rust-example-string .
#
# make sure docker is install and running
# systemctl status docker
# to make sure regular user can use docker
# sudo usermod -aG docker $USER#
# newgrp docker

# You may need to use docker login to create the `~/.docker/config.json` for auth.
#
# docker login

$ sudo buildah push --authfile ~/.docker/config.json rust-example-string docker://docker.io/secondstate/rust-example-string:crun
```

