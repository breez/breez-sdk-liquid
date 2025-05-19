use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use url::Url;
use tokio::sync::mpsc;

pub trait RelayService {
  async fn connect_to_relay(&self) -> Result<()>;
  async fn relay_to_sdk(&self, message: String) -> Result<()>;
  async fn sdk_to_relay(&self, message: String) -> Result<()>;
}

pub struct BreezRelayService {
  relay_url: String,
  pubkey: String,
  secret_key: String,
  handler: Arc<dyn RelayMessageHandler>,
  outgoing_receiver: mpsc::Receiver<String>,
}

impl RelayService for BreezRelayService {
  pub fn new(
    relay_url: String, 
    pubkey: String, 
    secret_key: String
  ) -> Self {
    let (outgoing_sender, outgoing_receiver) = mpsc::channel::<String>::new();
    let handler = BreezRelayMessageHandler::new(outgoing_sender);
    Self {
      relay_url,
      pubkey,
      secret_key,
      handler,
      outgoing_receiver,
    }
  }

  pub async fn connect_to_relay(&self) -> Result<()> {
    let url = Url::parse(&self.relay_url)?;
    
    // Connect to the WebSocket server
    let (ws_stream, response) = connect_async(url).await?;
    
    if !response.status().is_success() {
      return Err(anyhow::anyhow!("Failed to connect to relay: {}", response.status()));
    }
    
    log::info!("Successfully connected to relay");
    
    // Split the WebSocket stream into sender and receiver
    let (mut write, mut read) = ws_stream.split();

    write.send(Message::Ping(vec![])).await?;
    log::debug!("Sent initial ping");

    // Handle incoming messages
    while let Some(message) = read.next().await {
      match message? {
        Message::Text(text) => {
          log::debug!("Received text message: {}", text);
          self.recv(text).await?;
        }
        Message::Ping(data) => {
          // Respond to ping with pong to keep connection alive
          write.send(Message::Pong(data)).await?;
          log::debug!("Responded to ping with pong");
        }
        Message::Pong(_) => {
          log::debug!("Received pong, connection is alive");
        }
        Message::Close(frame) => {
          if let Some(reason) = frame.reason {
            log::info!("Connection closed: {}", reason);
          } else {
            log::info!("Connection closed without reason");
          }
          write.send(Message::Close(None)).await?;
          break;
        }
      }
    }

    log::info!("WebSocket connection closed");
    Ok(())
  }

  async fn recv(&self, message: String) -> Result<()> {
    // Decode and verify origin (pubkey)
    match message {
      PayInvoice => self.handler.pay_invoice(),
      ListTransactions => self.handler.list_transactions(),
      ... => // Debug unhandled messages
    }
  }

  pub async fn sdk_to_relay(&self, message: String) -> Result<()> {
    // TODO: Implement channel to receive message from SDK and send to relay
    Ok(())
  }
}
