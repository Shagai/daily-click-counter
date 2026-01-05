# Daily Click Counter (Rust)

A small Rust web app with two buttons (Add / Subtract). It tracks totals per calendar day and persists them in a local JSON file so you can add multi-day stats later.

## Run locally

```bash
cargo run
```

Open http://localhost:8080

## Run in a container

```bash
docker build -t daily-click-counter .

docker run --rm -p 8080:8080 \
  -v "$(pwd)/data:/app/data" \
  daily-click-counter
```

Open http://localhost:8080

## Configuration

- `PORT` (default: `8080`)
- `APP_DATA_PATH` (default: `data/state.json`)

The counters are based on the server's local date. If you need a specific timezone, set the container's `TZ` environment variable.
