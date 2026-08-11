<div align="center">
  <h1><code>wasmq</code></h1>
  <small>Job Queue for Rust applications powered by WASM</small>
</div>

<br />

<p align="center" dir="auto">
    <img src="assets/earlydev.svg" alt="Early development notice"/>
</p>

## Installation

### Docker

A Docker image is available as well. You can pull it from GitHub Container Registry.

```bash
docker pull ghcr.io/leoborai/wasmq:latest
```

```bash
docker run -p 6283:6283 ghcr.io/leoborai/wasmq
```

Then use `wasmq` CLI as regularly. `wasmq` CLI will perform requests to the `wasmq`
server running inside the Docker container.

#### Troubleshooting

##### Error response from daemon "denied"

If you are getting:

```bash
docker pull ghcr.io/leoborai/wasmq:latest
Error response from daemon: Head "https://ghcr.io/v2/leoborai/wasmq/manifests/latest": denied: denied
```

This is likely to be related to GHCR Credentials in your environment.
You can fix this by logging out usinc the following command:

```bash
docker logout ghcr.io
```

### GitHub Releases

You can also download precompiled binaries from the [GitHub Releases](https://githeub.com/LeoBorai/wasmq/releases) page.

## Development

### Pre-requisites

- Rust toolchain (_stable_): Install Rust using [rustup](https://rustup.rs/).
- SQLx CLI: Use `cargo install sqlx-cli` to install the SQLx CLI tool.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
This project is also licensed under the Apache License 2.0 - see the [LICENSE-APACHE](LICENSE-APACHE.md) file for details.
