## Ongoing

- [ ] CAPTCHA Integration (Phase 1B - Blocker)
  - [ ] Research browser automation or CAPTCHA solving strategies
  - [x] Integrate hCaptcha token provider (env/file based)
  - [x] Update `WebSocketClient::connect()` to obtain CAPTCHA token before session creation
  - [ ] Test full connection workflow once CAPTCHA is solved

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

## Pending

- [ ] CAPTCHA solving mechanism
- [ ] Argon2d mining engine (CPU, AVX2 SIMD)
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
- [x] Dependencies: tokio, tokio-tungstenite, serde, anyhow, reqwest, uuid
- [x] README.md
- [x] **Phase 1: Core Protocol (Days 1-2)**
  - [x] Define `ServerMessage` and `ClientMessage` enums in `src/messages.rs`
    - Strongly-typed protocol with all variants: Init, Job, Result, Verify, Update
    - Proper serde deserialization with rename attributes for API compatibility
  - [x] Implement `WebSocketClient::connect()` in `src/websocket.rs`
    - HTTP session creation via `/api/startSession`
    - WebSocket connection to `/ws/pow?session=<id>`
    - Proper error handling and logging
  - [x] Implement `WebSocketClient::run()` event loop
    - Message stream processing with futures
    - Ping/Pong handshake
    - Connection closure handling
  - [x] Handle message deserialization
    - Init, Job, Result messages logged and parsed
    - Type-safe enum dispatch (no string matching)
  - [x] Basic Argon2d hashing (no AVX2 yet) in `src/argon2.rs`
    - `hash_nonce()` function with configurable Argon2 parameters
    - `meets_target()` for share difficulty validation
    - Support for Argon2d, Argon2i, Argon2id variants
  - [x] Wired main.rs with WebSocket client
    - CLI wallet address from environment variable
    - Tokio async runtime with proper lifetime management

## Phase 1 Technical Notes

- **Blocking Issue:** pk910 faucet requires hCaptcha token for session creation. Browser-based miner solves this clientside.
- **Message Protocol:** Fully reverse-engineered from PoWFaucet source code (pk910/PoWFaucet repo).
- **WebSocket Endpoint:** `/ws/pow` with session ID query parameter. Server sends JSON messages with "action" field.
- **Argon2 Implementation:** Using standard `argon2` crate. Memory-heavy hashing (4MB per hash) must be offloaded to `spawn_blocking()` in Phase 2.
- **Architecture:** Clean module separation (messages.rs, websocket.rs, argon2.rs) following Rust idioms.
