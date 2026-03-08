use crate::model::normalized_event::NormalizedUpdate;

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
