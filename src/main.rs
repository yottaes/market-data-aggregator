use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "wss://stream.bybit.com/v5/public/spot";

    // println!("connecting to {url}...");
    let (mut ws, _) = connect_async(url).await?;
    // println!("connected!");

    // Subscribe to BTC/USDT order book (25 levels) and trades
    let sub = r#"{"op":"subscribe","args":["orderbook.50.BTCUSDT","publicTrade.BTCUSDT"]}"#;
    ws.send(Message::Text(sub.into())).await?;

    while let Some(msg) = ws.next().await {
        match msg? {
            Message::Text(text) => {
                if let Ok(_data) = serde_json::from_slice::<OrderBook>(text.as_bytes()) {
                    // println!("{:?}", data);
                    continue;
                };

                println!("{}", text);
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

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct OrderBook {
    pub topic: String,
    pub ts: u64,
    #[serde(rename = "type")]
    pub order_type: String,
    pub data: OrderBookData,
    pub cts: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    pub asks: Vec<(String, String)>,
    #[serde(rename = "u")]
    pub update_id: u64,
    pub seq: u64,
}

// fn parse_and_match(text: Utf8Bytes)
