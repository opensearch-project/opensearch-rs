# Developer Guide

So you want to contribute code to the OpenSearch Rust Client?  Excellent!  We're glad you're here.  Here's what you need to do:

## Getting Started

### Git Clone OpenSearch Rust Client Repository

Fork [opensearch-project/opensearch-rs](https://github.com/opensearch-project/opensearch-rs) and clone locally,
e.g. `git clone https://github.com/[your username]/opensearch-rs.git`.

### Install Prerequisites

## Development

The following information will help in getting up and running:

### Prerequisites

The project makes use of the following, which should be installed

- [**Docker**](https://www.docker.com/)

  Docker is used to start instances of OpenSearch by using
  [OpenSearch docker image](https://hub.docker.com/r/opensearchproject/opensearch).

- [**Cargo make**](https://sagiegurari.github.io/cargo-make/)

  Cargo make is used to define and configure a set of tasks, and run them as a flow. This helps with performing actions
  such as starting an OpenSearch instance for integration tests

  Cargo make can be installed with

  ```sh
  cargo install --force cargo-make
  ```


If you are running the tests in Docker, [set `vm.max_map_count` for your platform](https://opensearch.org/docs/latest/opensearch/install/important-settings) to allow OpenSearch to start.

### Cargo make

Cargo make is used to define and configure a set of tasks, and run them as a flow. To see all of the OpenSearch
category tasks defined

```sh
cargo make
```

The `OpenSearch` category of steps are specifically defined for this project and are defined in
[Makefile.toml](Makefile.toml).

- Build all packages

  ```sh
  cargo make build
  ```

- Generate client from REST specs

  ```sh
  cargo make generate-api
  ```

- Run OpenSearch package tests

  Optionally pass

  - `STACK_VERSION`: OpenSearch version like `1.0.0` or can be
  a snapshot release like `1.x-SNAPSHOT`

  ```sh
  STACK_VERSION=1.2.4 cargo make test
  ```

- Run YAML tests

  Optionally pass

  - `STACK_VERSION`: OpenSearch version like `1.0.0` or can be
  a snapshot release like `1.x-SNAPSHOT`

  ```sh
  STACK_VERSION=1.2.4 cargo make test-yaml
  ```

### Packages

The workspace contains the following packages:

- #### `opensearch`

  The client package crate. The client exposes all OpenSearch APIs as associated functions, either on
the root client, `OpenSearch`, or on one of the _namespaced clients_, such as `Cat`, `Indices`, etc. The _namespaced clients_
are based on the grouping of APIs within the [OpenSearch](https://github.com/opensearch-project/opensearch-specification/tree/main/specification) REST API specs from which much of the client is generated.
All API functions are `async` only, and can be `await`ed.

- #### `api_generator`

  A small executable that downloads REST API specs from GitHub and generates much of the client package from the specs.
The minimum REST API spec version compatible with the generator is `v7.4.0`.

  The `api_generator` package makes heavy use of the [`syn`](https://docs.rs/syn/1.0.5/syn/) and [`quote`](https://docs.rs/quote/1.0.2/quote/) crates to generate Rust code from the REST API specs.
The `quote!` macro is particularly useful as it accepts Rust code that can include placeholder tokens (prefixed with `#`)
that will be interpolated during expansion. Unlike procedural macros, the token stream returned by the `quote!` macro
can be `to_string()`'ed and written to disk, and this is used to create much of the client scaffolding.

- #### `yaml_test_runner`

  A small executable that downloads YAML tests from GitHub and generates client tests from the YAML tests. The
  version of YAML tests to download are determined from the commit hash of a running Elasticsearch instance.

  The `yaml_test_runner` package can be run with `cargo make test-yaml` to run the generated client tests and we can pass environment variable `STACK_VERSION` to control the distribution and version.

### Design principles

1. Generate as much of the client as feasible from the REST API specs

    The REST API specs contain information about
    - the URL parts e.g. `{index}/{type}/_search` and variants
    - accepted HTTP methods e.g. `GET`, `POST`
    - the URL query string parameters
    - whether the API accepts a body

2. Prefer generation methods that produce ASTs and token streams over strings.
The `quote` and `syn` crates help

3. Get it working, then refine/refactor

    - Start simple and iterate
    - Design of the API is conducive to ease of use
    - Asynchronous only
    - Control API invariants through arguments on API function. For example

      ```no_run
      client.delete_script(DeleteScriptParts::Id("script_id"))
          .send()
          .await?;
      ```

      An id must always be provided for a delete script API call, so the `delete_script()` function
      must accept it as a value.

### Coding style guide

The repository adheres to the styling enforced by `rustfmt`.

#### Formatting

Rust code can be formatted using [`rustfmt`](https://github.com/rust-lang/rustfmt) through cargo make.

To format all packages in a workspace, from the workspace root

```sh
cargo make format
```

It is strongly recommended to run this before opening a PR.

#### Clippy

[Clippy](https://github.com/rust-lang/rust-clippy) is a bunch of lints to catch common mistakes and improve your Rust code!

Run clippy before opening a PR

```sh
cargo make clippy
```

### Running MSVC debugger in VS Code

From [Bryce Van Dyk's blog post](https://www.brycevandyk.com/debug-rust-on-windows-with-visual-studio-code-and-the-msvc-debugger/),
if wishing to use the MSVC debugger with Rust in VS code, which may be preferred on Windows

1. Install [C/C++ VS Code extensions](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)

2. Place the following in `.vscode/launch.json` in the project root

    ```json
    {
        "version": "0.2.0",
        "configurations": [
            {
                "name": "Debug api_generator",
                "type": "cppvsdbg",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/api_generator.exe",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false
            }
        ]
    }
    ```

3. Add `"debug.allowBreakpointsEverywhere": true` to VS code settings.json
