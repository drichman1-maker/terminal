/// Cap'n Proto-based order book models
// TODO: Import generated Cap'n Proto code

pub struct PriceLevel {
    pub price: f64,
    pub quantity: f64,
    pub timestamp: i64,
}

pub struct OrderBook {
    pub symbol: String,
    pub exchange: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub timestamp: i64,
    pub sequence: u64,
}
