# Basic usage

## Run

Run with logger and watcher
RUST_LOG=info,celestus=debug cargo-watch -c -x run -i *.json -w src

RUST_LOG=celestus=debug cargo-watch -c -x run -i *.json -w src

## Docker run

docker-compose --profile dev up -d --build
docker-compose --profile prod up -d --build

## VScode settings

  `
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.allTargets": false
  `
