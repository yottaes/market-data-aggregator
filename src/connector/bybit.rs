use std::time::Duration;

use crate::{connector::ExchangeConnector, model::normalized_update::NormalizedUpdate};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::{mpsc, watch};
use tokio::time::sleep;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

pub struct BybitConnector {}

impl BybitConnector {
    //=======================================================================V//TODO: Implement Error set for connector
    async fn connect_and_subscribe(url: &str, sub: &str) -> Result<WsStream, anyhow::Error> {
        let (mut ws, _) = connect_async(url).await?;
        ws.send(Message::Text(sub.to_string().into())).await?;
        Ok(ws)
    }

    /// Returns true if shutdown was requested, false if connection was lost
    async fn read_messages(
        ws: &mut WsStream,
        sender: &mpsc::Sender<NormalizedUpdate>,
        shutdown: &mut watch::Receiver<bool>,
    ) -> bool {
        loop {
            tokio::select! {
                Some(msg) = ws.next() => {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Ok(data) = serde_json::from_str::<OrderBook>(&text) {
                                let update = NormalizedUpdate {
                                    exchange: "bybit",
                                    symbol: data.data.symbol,
                                    is_snapshot: data.order_type == "snapshot",
                                    bids: data.data.bids,
                                    asks: data.data.asks,
                                };
                                let _ = sender.send(update).await;
                            }
                        }
                        Ok(Message::Ping(data)) => {
                            if let Err(e) = ws.send(Message::Pong(data)).await {
                                println!("pong failed: {}", e);
                                return false;
                            }
                        }
                        Ok(Message::Close(frame)) => {
                            println!("connection closed: {frame:?}");
                            return false;
                        }
                        Ok(_) => {}
                        Err(e) => {
                            println!("ws error: {}", e);
                            return false;
                        }
                    }
                }
                _ = shutdown.changed() => {
                    return true;
                }
            }
        }
    }
}

impl ExchangeConnector for BybitConnector {
    fn exchange_name(&self) -> &'static str {
        "bybit"
    }

    async fn run(
        self,
        symbols: Vec<String>,
        sender: mpsc::Sender<NormalizedUpdate>,
        mut shutdown: watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error> {
        let url = "wss://stream.bybit.com/v5/public/spot";
        let mut backoff = 1u64;

        let args_vec: Vec<String> = symbols
            .iter()
            .map(|s| format!("orderbook.50.{}", s))
            .collect();
        let sub = json!({"op": "subscribe", "args": args_vec}).to_string();

        loop {
            match Self::connect_and_subscribe(url, &sub).await {
                Ok(mut ws) => {
                    backoff = 1;
                    if Self::read_messages(&mut ws, &sender, &mut shutdown).await {
                        return Ok(()); // shutdown requested
                    }
                }
                Err(e) => {
                    println!("connection failed: {}", e);
                }
            }

            backoff = (backoff * 2).min(64);
            sleep(Duration::from_secs(backoff)).await;
        }
    }
}

// Bybit-specific deserialization structures (private)
#[derive(Deserialize)]
struct OrderBook {
    #[serde(rename = "type")]
    order_type: String,
    data: OrderBookData,
}

#[derive(Deserialize)]
struct OrderBookData {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    asks: Vec<(String, String)>,
}
