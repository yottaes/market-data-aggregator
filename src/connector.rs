use tokio::sync::mpsc;

use crate::model::order_book::OrderBook;

pub mod bybit;

#[allow(async_fn_in_trait)]
pub trait ExchangeConnector: Send + Sync + 'static {
    fn exchange_name(&self) -> &'static str;

    async fn run(
        self,
        symbols: Vec<String>,
        sender: mpsc::Sender<OrderBook>,
        shutdown: tokio::sync::watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error>;
}
