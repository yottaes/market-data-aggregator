use market_data_aggregator::{
    connector::{ExchangeConnector, bybit::BybitConnector},
    model::Cup,
};
use tokio::sync::{mpsc, watch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);
    let (tx, mut rx) = mpsc::channel(200);

    let bybit = BybitConnector {};

    let mut cup = Cup::new();

    tokio::spawn(bybit.run(vec!["BTCUSDT".to_string()], tx, shutdown_receiver));

    loop {
        tokio::select! {
            Some(update) = rx.recv() => {
                cup.apply_update(update.is_snapshot, update.bids, update.asks);
                if let (Some(bid), Some(ask)) = (cup.best_bid(), cup.best_ask()) {
                    println!("[{}] {} bid={}, ask={}", update.exchange, update.symbol, bid.0, ask.0);
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
