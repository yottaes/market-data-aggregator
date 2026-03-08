use market_data_aggregator::{
    connector::{ExchangeConnector, bybit::BybitConnector},
    model::exchange_books::ExchangeBooks,
};
use tokio::sync::{mpsc, watch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);
    let (tx, mut rx) = mpsc::channel(200);

    let bybit = BybitConnector {};

    let mut cup = ExchangeBooks::new();

    tokio::spawn(bybit.run(
        vec![
            "BTCUSDT".to_string(),
            "SOLUSDT".to_string(),
            "ETHUSDT".to_string(),
            "XRPUSDT".to_string(),
        ],
        tx,
        shutdown_receiver,
    ));

    loop {
        tokio::select! {
            Some(update) = rx.recv() => {
                let (exchange, symbol) = cup.apply_update(update);
                if let (Some(bid), Some(ask)) = (cup.best_bid(&symbol), cup.best_ask(&symbol)) {
                    println!("[{}] {} bid={}, ask={}", exchange, symbol, bid, ask);
                }
            }
            _ = tokio::signal::ctrl_c() => {
                shutdown_sender.send(true)?;
                println!("Shutting down...");
                break;
            }
        }
    }

    Ok(())
}
