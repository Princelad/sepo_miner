## Ongoing

- [ ] WebSocket protocol implementation (`wss://sepolia-faucet.pk910.de/api/socket`)
  - [ ] Phase 1: Core Protocol (Days 1-2)
    - [ ] Define `ServerMessage` and `ClientMessage` enums in `src/messages.rs`
    - [ ] Implement `WebSocketClient::connect()` in `src/websocket.rs`
    - [ ] Implement `WebSocketClient::run()` event loop
    - [ ] Handle `Init`, `Job`, `Result` messages
    - [ ] Basic Argon2d hashing (no AVX2 yet) in `src/argon2.rs`
  - [ ] Phase 2: Job Management (Day 3)
    - [ ] Implement `MiningEngine` with priority queue in `src/miner.rs`
    - [ ] Handle `Verify` jobs (interrupt mechanism)
    - [ ] Nonce generation with randomization
    - [ ] Share submission logic
  - [ ] Phase 3: Robustness (Day 4)
    - [ ] Reconnection with exponential backoff
    - [ ] Ping/Pong heartbeat
    - [ ] Rate limit detection (429 status code)
    - [ ] Graceful shutdown (flush pending jobs)
  - [ ] Phase 4: Optimization (Day 5+)
    - [ ] AVX2 SIMD for Blake2b (Argon2 internals)
    - [ ] Thread pool tuning (match CPU core count)
    - [ ] Memory buffer re-use (avoid allocations)
    - [ ] Hashrate monitoring (hashes/sec)
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
