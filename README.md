# Basic usage

## Run

Run with logger and watcher
RUST_LOG=info cargo-watch -x run -i *.json -w src

## Docker run

docker-compose --profile dev up -d --build
docker-compose --profile prod up -d --build
