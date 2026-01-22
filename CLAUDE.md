# WSL Setup for rust-analyzer

## Steps to use rust-analyzer via WSL:

### 1. Open your project in WSL
In VSCode, press `Ctrl+Shift+P` and select **"WSL: Reopen Folder in WSL"**

This will:
- Open the project in WSL context
- All tools (rust-analyzer, cargo, etc.) will run in the Linux environment
- Your files remain on Windows but are accessed via `/mnt/c/Users/leifw/Documents/Rust/indra`

### 2. Install Rust in WSL (if not already installed)
Open WSL terminal and run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-unknown-linux-gnu
```

### 3. Install libcamera development files in WSL
```bash
sudo apt update
sudo apt install -y libcamera-dev pkg-config
```

### 4. Install cross-compilation tools in WSL
```bash
sudo apt install -y gcc-aarch64-linux-gnu
```

### 5. Update your .cargo/config.toml for WSL
The file should contain:
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

That's it! rust-analyzer will now work properly because:
- It runs in Linux (WSL)
- pkg-config is available
- libcamera headers are available
- Cross-compilation toolchain is available

## Alternative: Keep using Windows with a workaround

If you don't want to use WSL for development, you can keep working on Windows but we need to either:
1. Install pkg-config for Windows and create fake .pc files (complex)
2. Patch the libcamera build script to not fail on Windows (requires forking the crate)
3. Use cargo features to make libcamera optional during development

Given your constraints, **using WSL is by far the cleanest solution**.
