/// Normalized order book update sent through the channel
pub struct NormalizedUpdate {
    pub exchange: &'static str,
    pub symbol: String,
    pub is_snapshot: bool,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}
