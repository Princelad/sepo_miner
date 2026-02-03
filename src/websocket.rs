use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use uuid::Uuid;

use crate::messages::{ClientMessage, ServerMessage};

pub struct WebSocketClient {
    wallet_address: String,
    socket_base_url: String,
    session_id: String,
}

impl WebSocketClient {
    /// Create a new WebSocket client for the pk910 faucet
    pub fn new(wallet_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            socket_base_url: "wss://sepolia-faucet.pk910.de/ws/pow".to_string(),
            session_id: Uuid::new_v4().to_string(),
        }
    }

    /// Connect to the faucet WebSocket endpoint
    pub async fn connect(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        // First, create a session via HTTP POST request
        let session_url = "https://sepolia-faucet.pk910.de/api/startSession";
        let client = reqwest::Client::new();

        let response = client
            .post(session_url)
            .json(&serde_json::json!({
                "addr": self.wallet_address,
            }))
            .send()
            .await
            .context("Failed to create session")?;

        let session_data: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse session response")?;

        eprintln!("[Debug] Session response: {}", session_data);

        let session_id = session_data
            .get("session")
            .or(session_data.get("data").and_then(|d| d.get("session")))
            .and_then(|v| v.as_str())
            .context("Missing 'session' field in response")?
            .to_string();

        eprintln!("[Session] Created session: {}", session_id);

        // Now connect to WebSocket with the valid session ID
        let url = format!("{}?session={}", self.socket_base_url, session_id);
        let (ws_stream, response) = connect_async(&url)
            .await
            .context("Failed to connect to pk910 faucet")?;

        eprintln!(
            "[WebSocket] Connected to {} (HTTP {})",
            self.socket_base_url,
            response.status()
        );

        Ok(ws_stream)
    }

    /// Run the main event loop
    pub async fn run(&self) -> Result<()> {
        let mut ws_stream = self.connect().await?;

        // Send initial Start message
        let start_msg = ClientMessage::start(&self.wallet_address);
        self.send_message(&mut ws_stream, &start_msg).await?;

        println!(
            "[Session] Started mining for wallet: {}",
            self.wallet_address
        );

        // Main event loop
        while let Some(msg_result) = ws_stream.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    self.handle_text_message(&text).await?;
                }
                Ok(Message::Binary(_)) => {
                    eprintln!("[Warning] Received unexpected binary message");
                }
                Ok(Message::Ping(payload)) => {
                    ws_stream
                        .send(Message::Pong(payload))
                        .await
                        .context("Failed to send Pong")?;
                }
                Ok(Message::Pong(_)) => {
                    // Server acknowledged our ping
                }
                Ok(Message::Close(frame)) => {
                    eprintln!("[WebSocket] Server closed connection: {:?}", frame);
                    break;
                }
                Err(e) => {
                    eprintln!("[Error] WebSocket error: {}", e);
                    return Err(e.into());
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Handle incoming text messages from the server
    async fn handle_text_message(&self, text: &str) -> Result<()> {
        // Deserialize into strongly-typed ServerMessage enum
        let server_msg: ServerMessage = serde_json::from_str(text)
            .with_context(|| format!("Failed to parse server message: {}", text))?;

        match server_msg {
            ServerMessage::Init {
                session,
                target_addr,
                difficulty,
                claimable_balance,
                ..
            } => {
                println!("[Init] Session: {}", session);
                println!("[Init] Target Address: {}", target_addr);
                println!("[Init] Difficulty: {}", difficulty);
                println!("[Init] Claimable Balance: {}", claimable_balance);
            }
            ServerMessage::Job {
                id,
                pre_image,
                target,
                algorithm,
                argon2_params,
            } => {
                println!("[Job] ID: {}", id);
                println!("[Job] Pre-Image: {}", pre_image);
                println!("[Job] Target: {}", target);
                println!("[Job] Algorithm: {}", algorithm);
                println!("[Job] Argon2 Params: {:?}", argon2_params);
                // TODO: Dispatch to mining engine
            }
            ServerMessage::Verify {
                share_id,
                nonce,
                pre_image,
                argon2_params,
            } => {
                println!("[Verify] Share ID: {}", share_id);
                println!("[Verify] Nonce: {}", nonce);
                println!("[Verify] Pre-Image: {}", pre_image);
                println!("[Verify] Argon2 Params: {:?}", argon2_params);
                // TODO: Handle verification job (high priority)
            }
            ServerMessage::Result {
                share_id,
                status,
                error_code,
                error_message,
                balance,
            } => {
                println!("[Result] Share ID: {} - Status: {:?}", share_id, status);
                if let Some(err) = error_message {
                    eprintln!(
                        "[Result] Error: {} ({})",
                        err,
                        error_code.unwrap_or_default()
                    );
                }
                if let Some(bal) = balance {
                    println!("[Result] New Balance: {}", bal);
                }
            }
            ServerMessage::Update {
                session,
                claimable_balance,
            } => {
                println!(
                    "[Update] Session: {} - Balance: {}",
                    session, claimable_balance
                );
            }
        }

        Ok(())
    }

    /// Send a message to the server
    async fn send_message(
        &self,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
        msg: &ClientMessage,
    ) -> Result<()> {
        let json = msg.to_json()?;
        ws_stream
            .send(Message::text(json))
            .await
            .context("Failed to send message")?;
        Ok(())
    }
}
