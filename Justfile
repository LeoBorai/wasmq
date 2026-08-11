set positional-arguments

target_release := "x86_64-unknown-linux-musl"

default:
    @echo "No default task defined."
    just --list

# Builds a example task into a WASM file
build-task task:
    rm ./{{task}}.wasm || true
    cd ./task/{{task}} && cargo +nightly build --release --target wasm32-wasip2
    mv ./target/wasm32-wasip2/release/{{task}}.wasm ./{{task}}.wasm

# Runs the DKC
dkc:
    docker pull ghcr.io/leoborai/dkc:latest
    docker run -it --rm \
        -v $(pwd):/app \
        -w /app \
        ghcr.io/leoborai/dkc:latest

# Builds docs into static files (docs/book/)
docs-build:
    cd ./docs && mdbook build

# Runs server to serve docs locally (http://localhost:3000)
docs-dev:
    cd ./docs && mdbook serve

# Builds the Server binary used in the Docker Image
docker-build:
    cargo zigbuild --target {{target_release}} --release --bin wasmq

# Builds the Docker image
docker-build-image: docker-build
    mkdir -p ./docker/tmp/
    cp ./target/{{target_release}}/release/wasmq ./docker/tmp/wasmq
    chmod +x ./docker/tmp/wasmq
    docker build -t "wasmq:$(cargo tag current)" ./docker

# Publishes the Docker image to the GitHub Container Registry
docker-publish-image:
    docker tag wasmq:$(cargo tag current) ghcr.io/leoborai/wasmq:$(cargo tag current)
    docker tag wasmq:$(cargo tag current) ghcr.io/leoborai/wasmq:latest
    docker push ghcr.io/leoborai/wasmq:$(cargo tag current)
    docker push ghcr.io/leoborai/wasmq:latest

# Runs the Docker image locally
docker-run-image: docker-build-image
    docker run wasmq:$(cargo tag current)

# Runs clippy and fmt on the entire workspace
fmt:
    cargo clippy --fix --workspace --allow-dirty --allow-staged && cargo fmt

# Uses `sqlx` CLI to perform database metadata retrieval
sqlx-prepare:
    cargo sqlx prepare --workspace

# Runs all tests in the workspace using `nextest`
test:
    cargo nextest run --workspace --no-fail-fast --all-features
