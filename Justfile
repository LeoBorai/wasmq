set positional-arguments

latest_tag := `cargo tag current`
target_release := "x86_64-unknown-linux-musl"

default:
    @echo "No default task defined."
    just --list

# Builds a example task into a WASM file
build-task task:
    rm ./{{task}}.wasm || true
    cd ./task/{{task}} && cargo +nightly build --release --target wasm32-wasip2
    mv ./target/wasm32-wasip2/release/{{task}}.wasm ./{{task}}.wasm

# Builds the Server binary used in the Docker Image
docker-build:
    cargo zigbuild --target {{target_release}} --release --bin mate

# Builds the Docker image
docker-build-image: docker-build
    mkdir -p ./docker/tmp/
    cp ./docs/config/local.toml ./docker/tmp/local.toml
    cp ./target/{{target_release}}/release/mate ./docker/tmp/mate
    chmod +x ./docker/tmp/mate
    docker build -t "mate:{{latest_tag}}" ./docker

# Publishes the Docker image to the GitHub Container Registry
docker-publish-image:
    docker tag mate:{{latest_tag}} ghcr.io/leoborai/mate:{{latest_tag}}
    docker tag mate:{{latest_tag}} ghcr.io/leoborai/mate:latest
    docker push ghcr.io/leoborai/mate:{{latest_tag}}
    docker push ghcr.io/leoborai/mate:latest

# Runs the Docker image locally
docker-run-image: docker-build-image
    docker run mate:{{latest_tag}}

# Echo latest tag
latest-tag:
    echo {{latest_tag}}
