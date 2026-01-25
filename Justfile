default:
    @echo "No default task defined."
    just --list

# Builds a example task into a WASM file
build-task task:
    rm ./{{task}}.wasm || true
    cd ./task/{{task}} && cargo +nightly build --release --target wasm32-wasip2
    mv ./target/wasm32-wasip2/release/{{task}}.wasm ./{{task}}.wasm
