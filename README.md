# Rust examples for WasmEdge

Simple Rust applications that run in WasmEdge. They also serve as a good learning resource for the Rust language.

## Prerequisites

You can just [install Docker Desktop](https://www.docker.com/products/docker-desktop/).

Or, you could install [Rust](https://www.rust-lang.org/tools/install) and [WasmEdge](https://wasmedge.org/book/en/quick_start/install.html) as follows.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup target add wasm32-wasi

curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -p /usr/local
```

**Note**: Running `cargo build` in any package directory the output will get stored at parent directory of folder because of Cargo Workspace.

```
├── Cargo.lock
├── Cargo.toml
├── control
├── function
├── hello
├── move
├── README.md
├── server
├── string
├── struct
├── target # Your Build files
└── wasi
```

## Standalone examples

This set of examples demonstrate Rust apps that each has a `main()` function and can be started and executed a standalone app.

> The standalone applications are dockerized so that you can get started easily.

* [Hello world](hello/)
* [Working with strings](string/)
* [Ownership and move](move/)
* [Control flows](control/)
* [Data types and structures](struct/)
* [Functions and return values](function/)
* [Use OS system libraries](wasi/)
* [Create an HTTP server](server/)

## Library examples

This set of examples demonstrate Rust library functions that can be called from outside of the WasmEdge Runtime (i.e., in a host application).

* [Add two numbers](add/)
* [Pass a string to the function](string/)
* [Parse JSON](json/)

## More examples

For Rust apps that compile to Wasm and run inside WasmEdge.

* [High-performance HTTP services using the hyper_wasi crate](https://github.com/WasmEdge/wasmedge_hyper_demo)
* [HTTP clients using the reqwest_wasi crate](https://github.com/WasmEdge/wasmedge_reqwest_demo)
* [HTTPS clients using the http_req_wasi crate](https://github.com/second-state/http_req#build-and-run)
* [Database applications](https://github.com/WasmEdge/wasmedge-db-examples)
* [An HTTP microservice with a MySQL backend](https://github.com/second-state/microservice-rust-mysql)
* [A Kafka service with a MySQL backend](https://github.com/docker/awesome-compose/tree/master/wasmedge-kafka-mysql)
* [ETL functions for databases](https://github.com/second-state/MEGA)
* [Complex microservices with Dapr](https://github.com/second-state/dapr-wasm)
* [Tensorflow, Pytorch and OpenVINO apps](https://github.com/second-state/WasmEdge-WASINN-examples)

For Rust apps that embed WasmEdge apps and provide host functions to WasmEdge.

* [WasmEdge Rust SDK examples](https://github.com/second-state/wasmedge-rustsdk-examples)

