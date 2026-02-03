# SepoMiner

High-performance headless client for the [pk910 Sepolia Faucet](https://sepolia-faucet.pk910.de/).

Replaces the inefficient browser-based WASM miner with a native Rust executable utilizing AVX2 (CPU) and CUDA (GPU) instructions.

## Why

- **10x faster** than browser mining (no V8 overhead, no GC pauses)
- **Native Argon2d** implementation with SIMD optimizations
- **GPU acceleration** via CUDA for maximum hashrate
- **Zero thermal throttling** from browser bloat

## Usage

```bash
HCAPTCHA_TOKEN="<token>" WALLET_ADDRESS=0xYourAddress cargo run --release
```

## Requirements

- Rust 1.70+
- CUDA Toolkit (for GPU mining)
- AVX2-capable CPU (Intel 4th gen+ / Ryzen 1st gen+)
- hCaptcha token (set HCAPTCHA_TOKEN or HCAPTCHA_TOKEN_FILE)

## Protocol

- **Endpoint:** `wss://sepolia-faucet.pk910.de/api/socket`
- **Algorithm:** Argon2d (Memory-Hard PoW)
- **Mining Strategy:** Prioritizes `verify` jobs over mining jobs (higher ROI)

## Performance

Optimized for:

- Intel i5-12500H (12 threads)
- NVIDIA RTX 3050 (2560 CUDA cores)

Expected hashrate: TBD (depends on difficulty)

## License

MIT
