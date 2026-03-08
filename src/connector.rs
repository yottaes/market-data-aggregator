pub mod bybit;

#[allow(async_fn_in_trait)]
pub trait ExchangeConnector: Send + Sync + 'static {
    fn exchange_name(&self) -> &'static str;

    async fn run(
        self,
        symbols: Vec<String>,
        sender: tokio::sync::mpsc::Sender<NormalizedUpdate>,
        shutdown: tokio::sync::watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error>;
}

/// Normalized order book update sent through the channel
pub struct NormalizedUpdate {
    pub exchange: &'static str,
    pub symbol: String,
    pub is_snapshot: bool,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}
