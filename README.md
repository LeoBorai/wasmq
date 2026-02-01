<div align="center">
  <h1><code>mate</code></h1>
  <small>Job Queue for Rust applications powered with WASM</small>
</div>

<br />

<p align="center" dir="auto">
    <img src="assets/earlydev.svg" alt="Early development notice"/>
</p>

## Installation

### Docker

A Docker image is available as well. You can pull it from GitHub Container Registry.

```bash
docker pull ghcr.io/leoborai/mate:latest
```

```bash
docker run -p 6283:6283 ghcr.io/leoborai/mate
```

Then use `mate` CLI as regularly. `mate` CLI will perform requests to the `mate`
server running inside the Docker container.

#### Troubleshooting

##### Error response from daemon "denied"

If you are getting:

```bash
docker pull ghcr.io/leoborai/mate:latest
Error response from daemon: Head "https://ghcr.io/v2/leoborai/mate/manifests/latest": denied: denied
```

This is likely to be related to GHCR Credentials in your environment.
You can fix this by logging out usinc the following command:

```bash
docker logout ghcr.io
```

### GitHub Releases

You can also download precompiled binaries from the [GitHub Releases](https://githeub.com/LeoBorai/mate/releases) page.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
This project is also licensed under the Apache License 2.0 - see the [LICENSE-APACHE](LICENSE-APACHE.md) file for details.
