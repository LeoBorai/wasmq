# Quick Start

## Requirements

- `mate`: The mate CLI and server
- `just`: Command runner used for convenience

## Building an example task

Tasks are unit of logic used to define workflows in mate.

As of today mate supports Task written in Rust and compiled to WebAssembly (Wasm) format,
in the future other languages and formats may be supported.

```bash
just build-task http
```

This will compile the example HTTP task located at `task/http` into WebAssembly format and copy
the output file to the current directory.

## Starting the mate server

Run a mate instance, you can either run it in Docker or locally.

### Docker

```bash
docker pull ghcr.io/leoborai/mate:latest
docker run -p 6283:6283 ghcr.io/leoborai/mate
```

### Locally

```bash
mate run
```
