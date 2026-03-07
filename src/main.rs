use market_data_aggregator::{
    connector::{ExchangeConnector, bybit::BybitConnector},
    model::order_book::{Cup, OrderBook},
};
use tokio::sync::{mpsc, watch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);
    let (tx, mut rx) = mpsc::channel::<OrderBook>(200);

    let bybit = BybitConnector {};

    let mut bybit_cup = Cup::new();

    tokio::spawn(bybit.run(vec!["BTCUSDT".to_string()], tx, shutdown_receiver));

    loop {
        tokio::select! {
            Some(order) = rx.recv() => {
                bybit_cup.apply(order);
                println!("bid={}, ask={}", bybit_cup.best_bid().unwrap().0, bybit_cup.best_ask().unwrap().0);
            }
            _ = tokio::signal::ctrl_c() => {
                shutdown_sender.send(true)?;
                println!("Sent Ctrl+C!\nExiting!");
                break;
            }
        }
    }

    Ok(())
}

// fn parse_and_match(text: Utf8Bytes)
