use crate::connector::{ExchangeConnector, NormalizedUpdate};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct BybitConnector {}

impl ExchangeConnector for BybitConnector {
    fn exchange_name(&self) -> &'static str {
        "bybit"
    }

    async fn run(
        self,
        symbols: Vec<String>,
        sender: tokio::sync::mpsc::Sender<NormalizedUpdate>,
        mut shutdown: tokio::sync::watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error> {
        let url = "wss://stream.bybit.com/v5/public/spot";

        let (mut ws, _) = connect_async(url).await?;
        let args_vec = symbols
            .iter()
            .map(|s| format!("orderbook.50.{}", s))
            .collect::<Vec<_>>();

        let sub = json!({"op": "subscribe", "args": args_vec}).to_string();

        ws.send(Message::Text(sub.into())).await?;

        loop {
            tokio::select! {
                Some(msg) = ws.next() => {
                    match msg? {
                        Message::Text(text) => {
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
                        Message::Ping(data) => {
                            ws.send(Message::Pong(data)).await?;
                        }
                        Message::Close(frame) => {
                            println!("connection closed: {frame:?}");
                            break;
                        }
                        _ => {}
                    }
                }
                _ = shutdown.changed() => {
                    break;
                }
            }
        }

        Ok(())
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
