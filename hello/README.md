# Hello WasmEdge

Please [install the prerequisites](../README.md) first!

## Quick start with Docker

```
$ docker run secondstate/rust-example-hello:latest
Hello WasmEdge!
```

## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/debug/hello.wasm
Hello WasmEdge!
```

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub. The process is slightly different depending on how you plan to use the image.

### For Docker Desktop and containerd

For containerd based systems, such as the Docker Desktop and many flavors of Kubernetes, 
you just need to specify that the WasmEdge application image is for the `wasi/wasm32` platform.

```
$ docker buildx build --platform wasi/wasm32 -t secondstate/rust-example-hello .
... ...
$ docker push secondstate/rust-example-hello
```

### For Podman, OpenShift and CRI-O

For `crun` based systems, such as Podman, OpenShift and CRI-O, 
you will need to specify a special annotation for the image so that `crun` knowns to use WasmEdge to run it.
In this example, we push the image to Docker Hub with a `crun` tag.

```
$ sudo buildah build --annotation "module.wasm.image/variant=compat-smart" -t rust-example-hello .
#
# make sure docker is install and running
# systemctl status docker
# to make sure regular user can use docker
# sudo usermod -aG docker $USER#
# newgrp docker

# You may need to use docker login to create the `~/.docker/config.json` for auth.
#
# docker login

$ sudo buildah push --authfile ~/.docker/config.json rust-example-hello docker://docker.io/secondstate/rust-example-hello:crun
```

