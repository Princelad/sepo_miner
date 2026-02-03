use serde::{Deserialize, Serialize};

/// Messages sent from the pk910 faucet server to the client
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ServerMessage {
    /// Initial session parameters from the server
    Init {
        session: String,
        #[serde(rename = "targetAddr")]
        target_addr: String,
        #[serde(rename = "hashrate")]
        hashrate_limit: Option<u64>,
        difficulty: u32,
        #[serde(rename = "claimable")]
        claimable_balance: String,
    },
    /// Mining job assignment
    Job {
        id: String,
        #[serde(rename = "preImage")]
        pre_image: String,
        target: String,
        algorithm: String,
        #[serde(rename = "argon2")]
        argon2_params: Argon2Params,
    },
    /// Verification job (high priority - interrupts mining)
    Verify {
        #[serde(rename = "shareId")]
        share_id: String,
        nonce: String,
        #[serde(rename = "preImage")]
        pre_image: String,
        #[serde(rename = "argon2")]
        argon2_params: Argon2Params,
    },
    /// Server's response to a submitted share
    Result {
        #[serde(rename = "shareId")]
        share_id: String,
        status: ShareStatus,
        #[serde(rename = "errorCode")]
        error_code: Option<String>,
        #[serde(rename = "errorMessage")]
        error_message: Option<String>,
        balance: Option<String>,
    },
    /// Session update (e.g., balance change)
    Update {
        session: String,
        #[serde(rename = "claimable")]
        claimable_balance: String,
    },
}

/// Argon2d parameters from the server
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Argon2Params {
    #[serde(rename = "type")]
    pub variant: u8, // 0 = Argon2d, 1 = Argon2i, 2 = Argon2id
    pub version: u32,
    pub time_cost: u32,
    pub memory_cost: u32,
    pub parallelism: u32,
    pub key_length: u32,
}

/// Share submission status
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShareStatus {
    Valid,
    Invalid,
    Duplicate,
    Stale,
}

/// Messages sent from the client to the pk910 faucet server
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ClientMessage {
    /// Start a new mining session
    Start {
        wallet: String,
        #[serde(rename = "minerVersion")]
        miner_version: String,
    },
    /// Submit a valid nonce (share)
    Submit {
        #[serde(rename = "shareId")]
        share_id: String,
        nonce: String,
        #[serde(rename = "hashrate")]
        hashrate: Option<u64>,
    },
    /// Respond to a verify job
    Verify {
        #[serde(rename = "shareId")]
        share_id: String,
        result: String,
    },
    /// Ping to keep connection alive
    Ping,
}

impl ClientMessage {
    /// Create a Start message with the given wallet address
    pub fn start(wallet: impl Into<String>) -> Self {
        Self::Start {
            wallet: wallet.into(),
            miner_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Create a Submit message
    pub fn submit(
        share_id: impl Into<String>,
        nonce: impl Into<String>,
        hashrate: Option<u64>,
    ) -> Self {
        Self::Submit {
            share_id: share_id.into(),
            nonce: nonce.into(),
            hashrate,
        }
    }

    /// Create a Verify response message
    pub fn verify_response(share_id: impl Into<String>, result: impl Into<String>) -> Self {
        Self::Verify {
            share_id: share_id.into(),
            result: result.into(),
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}
