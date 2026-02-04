# Quick Start

Welcome to the "mate" quick start guide.
"mate" is a Job Queue inspired on solutions like Sidekiq
from Ruby and Celery from Python.

Job Queues are useful for scheduling a piece of logic to be
executed asynchronously on a desired point on time.

For instance you might want to send a reminder email to a
user in eleven months to remind it to renew a subscription.

You could also use the Job Queue to offload your server's
with background tasks.

For instance, you could perform an AI model analysis process
in the background instead of blocking your server's thread.

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

> You can find more details on the GitHub's Docker
> container [summary page](https://github.com/LeoBorai/mate/pkgs/container/mate).

### Locally

Binaries for `mate` are published as part of the GitHub
Action workflow for Release.

Visit the [releases page](https://github.com/LeoBorai/mate/releases) for more details.

```bash
mate run
```

## Register the Task in mate's Task Repository

In order to have the mate Task reachable it must be registered in mate's Task Repository.

The `task` directory from the GitHub's repository contains
example tasks. A HTTP task in included which is used for
performing HTTP requests.

```bash
mate task load --id mate/http@0.1.0 ./http.wasm
```

> Tasks are named with a namespace, task name and version.
> On this example we are storing a task `http` under
> the `mate` namespace.

## Create a Job

A Job is the definion of time and arguwents to execute a `Task`
on a desired moment of time with the desired arguments.

The following command creates a Job that will perform an HTTP POST
request to `https://httpbin.org/post`

```bash
mate job new \
    --name greet \
    --args "{\"api_url\": \"https://httpbin.org/post\",\"data\": {\"sample_key\": \"sample_value\"}}" \
    --task leo/http@0.1.0 \
    --scheduled-at 3s
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
