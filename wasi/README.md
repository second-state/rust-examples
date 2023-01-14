# WASI

Please [install the prerequisites](../README.md) first!

## Quick start with Docker

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-wasi:latest
```

## Code

The [`src/main.rs`](src/main.rs) source code shows how to access OS services through standard libraries.

* Access the random number generator in the OS
* Access the local file system
* Access env vars in the OS
* Access command line args when starting this app


## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge --dir .:. target/wasm32-wasi/debug/wasi.wasm
Random number: -1157533356
Random bytes: [159, 159, 9, 119, 106, 172, 207, 82, 173, 145, 233, 214, 104, 35, 23, 53, 155, 12, 102, 231, 117, 67, 192, 215, 207, 202, 128, 198, 213, 41, 235, 57, 89, 223, 138, 70, 185, 137, 74, 162, 42, 20, 226, 177, 114, 170, 172, 39, 149, 99, 122, 68, 115, 205, 155, 202, 4, 48, 178, 224, 124, 42, 24, 56, 215, 90, 203, 150, 106, 128, 127, 201, 177, 187, 20, 195, 172, 56, 72, 28, 53, 163, 59, 36, 129, 160, 69, 203, 196, 72, 113, 61, 46, 249, 81, 134, 94, 134, 159, 51, 233, 247, 253, 116, 202, 210, 100, 75, 74, 95, 197, 44, 81, 87, 89, 115, 20, 226, 143, 139, 50, 60, 196, 59, 206, 105, 161, 226]
Printed from wasi: This is from a main function
This is from a main function
The env vars are as follows.
The args are as follows.
wasi.wasm
File content is This is in a file
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub. The process is slightly different depending on how you plan to use the image.

### For Docker Desktop and containerd

For containerd based systems, such as the Docker Desktop and many flavors of Kubernetes,
you just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ docker buildx build --platform wasi/wasm -t secondstate/rust-example-wasi .
... ...
$ docker push secondstate/rust-example-wasi
```

### For Podman, OpenShift and CRI-O

For `crun` based systems, such as Podman, OpenShift and CRI-O,
you will need to specify a special annotation for the image so that `crun` knowns to use WasmEdge to run it.
In this example, we push the image to Docker Hub with a `crun` tag.

```
$ sudo buildah build --annotation "module.wasm.image/variant=compat-smart" -t rust-example-wasi .
#
# make sure docker is install and running
# systemctl status docker
# to make sure regular user can use docker
# sudo usermod -aG docker $USER#
# newgrp docker

# You may need to use docker login to create the `~/.docker/config.json` for auth.
#
# docker login

$ sudo buildah push --authfile ~/.docker/config.json rust-example-string docker://docker.io/secondstate/rust-example-wasi:crun
```

