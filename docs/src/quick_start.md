# Quick Start

## Requirements

- `mate`: The mate CLI and server
- `just`: Command runner used for convenience

## Building an example task

Tasks are unit of logic used to define workflows in mate.

As of today mate supports Task written in Rust and compiled to WebAssembly (Wasm) format,
in the future other languages and formats may be supported.

The following task performs an HTTP request to a given URL with given data.

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

## Register the Task in mate's Task Repository

In order to have the mate Task reachable it must be registered in mate's Task Repository.

```bash
mate task load --id username/http@0.1.0 ./http.wasm
```

## Create a Job

A Job is a scheduled execution of a Task with specific arguments.
The following command creates a Job that will perform an HTTP POST request to `https://httpbin.org/post`

```bash
mate job new \
    --name hello-mate \
    --args "{\"api_url\": \"https://httpbin.org/post\",\"data\": {\"sample_key\": \"sample_value\"}}" \
    --task username/http@0.1.0
```

## List Jobs

Run the following command to list all Jobs registered in mate:

```bash
mate job ls -a
```

This will list all jobs and its status.

## Inspect a Job

You can inspect on a Job's result, status and more details by using
the `view` subcommand along with the Job ID obtained from the previous command:

```bash
mate job view <JOB_ID>
```

## Congratulations!

You have created your first mate Job!
