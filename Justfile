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

# Builds docs into static files (docs/book/)
docs-build:
    cd ./docs && mdbook build

# Runs server to serve docs locally (http://localhost:3000)
docs-dev:
    cd ./docs && mdbook serve

# Builds the Server binary used in the Docker Image
docker-build:
    cargo zigbuild --target {{target_release}} --release --bin mate

# Builds the Docker image
docker-build-image: docker-build
    mkdir -p ./docker/tmp/
    cp ./target/{{target_release}}/release/mate ./docker/tmp/mate
    chmod +x ./docker/tmp/mate
    docker build -t "mate:$(cargo tag current)" ./docker

# Publishes the Docker image to the GitHub Container Registry
docker-publish-image:
    docker tag mate:$(cargo tag current) ghcr.io/leoborai/mate:$(cargo tag current)
    docker tag mate:$(cargo tag current) ghcr.io/leoborai/mate:latest
    docker push ghcr.io/leoborai/mate:$(cargo tag current)
    docker push ghcr.io/leoborai/mate:latest

# Runs the Docker image locally
docker-run-image: docker-build-image
    docker run mate:$(cargo tag current)

# Runs clippy and fmt on the entire workspace
fmt:
    cargo clippy --fix --workspace --allow-dirty --allow-staged && cargo fmt
