## Ongoing

- [ ] WebSocket protocol implementation (`wss://sepolia-faucet.pk910.de/api/socket`)
- [ ] Message type deserialization (strongly-typed enums)

## Pending

- [ ] Argon2d hashing engine (CPU, AVX2 SIMD)
- [ ] Nonce generation & share submission logic
- [ ] Mining job prioritization (verify > mining)
- [ ] Thread pool for `spawn_blocking` offload
- [ ] CUDA kernel integration (Phase 3)
- [ ] GPU memory buffer management (4MB per hash)
- [ ] Rate limit handling & backoff strategy
- [ ] CLI argument parsing (wallet address, threads, GPU toggle)
- [ ] Real-time hashrate monitoring

## Completed

- [x] Project scaffold (Cargo.toml, src/main.rs)
- [x] Dependencies: tokio, tokio-tungstenite, serde, anyhow
- [x] README.md
