# Indra - Cross-Compilation Setup

This project is configured to cross-compile for Raspberry Pi Zero 2 W using Docker.

## Initial Setup

1. Build the Docker image:
```bash
docker compose build
```

2. Install the ARM64 target (if building locally):
```bash
rustup target add aarch64-unknown-linux-gnu
```

## Building

### Using Docker (recommended)
```bash
# Release build
docker compose run --rm indra-builder cargo build --target aarch64-unknown-linux-gnu --release

# Debug build
docker compose run --rm indra-builder cargo build --target aarch64-unknown-linux-gnu

# Or use the build script
docker compose run --rm indra-builder ./build.sh
```

### Using VSCode
- Press `Ctrl+Shift+B` to build (uses the default build task)
- Or run tasks from the Command Palette (`Ctrl+Shift+P` > "Tasks: Run Task")

### Build output
The compiled binary will be at: `target/aarch64-unknown-linux-gnu/release/indra`

## Deploying to Pi

### Manual deployment
```bash
scp ./target/aarch64-unknown-linux-gnu/release/indra pi@raspberrypi.local:~/
```

### Using VSCode task
Run the "Deploy to Pi" task from the Command Palette

## Running on Pi

SSH into your Pi and run:
```bash
chmod +x ~/indra
sudo ./indra
```

Note: GPIO access requires root privileges.

## Development in Docker

To get an interactive shell in the Docker container:
```bash
docker compose run --rm indra-builder bash
```

Then you can run cargo commands directly:
```bash
cargo build --target aarch64-unknown-linux-gnu
cargo check --target aarch64-unknown-linux-gnu
```

## VSCode Integration

The `.vscode/settings.json` configures rust-analyzer to use the ARM64 target. However, for full IDE support, you may want to:

1. Keep rust-analyzer checking for your native target for better IDE experience
2. Build for ARM64 only when deploying

To adjust this, modify `.vscode/settings.json` and remove or comment out the `rust-analyzer.cargo.target` setting.

## Troubleshooting

### Docker volume permissions
If you get permission errors, ensure Docker has access to your project directory in Docker Desktop settings.

### Cargo cache
The Docker setup uses a named volume for cargo cache to speed up builds. To clear it:
```bash
docker volume rm indra_cargo-cache
```

### Pi connection
If `raspberrypi.local` doesn't resolve, use the Pi's IP address instead:
```bash
scp ./target/aarch64-unknown-linux-gnu/release/indra pi@192.168.1.x:~/
```
