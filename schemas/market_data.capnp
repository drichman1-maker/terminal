@0xf8a3b2c1d4e5f6a7;
# Cap'n Proto Schema for Kinetic Trading Terminal
# Zero-copy serialization for high-frequency market data

struct PriceLevel {
  price @0 :Float64;
  quantity @1 :Float64;
  timestamp @2 :Int64;  # Unix timestamp in microseconds
}

struct OrderBook {
  symbol @0 :Text;
  exchange @1 :Text;  # "binance", "okx", "bybit", "coinbase"
  bids @2 :List(PriceLevel);
  asks @3 :List(PriceLevel);
  timestamp @4 :Int64;
  sequence @5 :UInt64;  # Sequence number for ordering updates
}

struct UnifiedOrderBook {
  symbol @0 :Text;
  aggregatedBids @1 :List(PriceLevel);
  aggregatedAsks @2 :List(PriceLevel);
  exchangeBooks @3 :List(OrderBook);
  timestamp @4 :Int64;
  totalBidLiquidity @5 :Float64;
  totalAskLiquidity @6 :Float64;
}

struct Trade {
  symbol @0 :Text;
  exchange @1 :Text;
  price @2 :Float64;
  quantity @3 :Float64;
  timestamp @4 :Int64;
  side @5 :Side;
  tradeId @6 :Text;
  
  enum Side {
    buy @0;
    sell @1;
  }
}

struct Ticker {
  symbol @0 :Text;
  exchange @1 :Text;
  lastPrice @2 :Float64;
  volume24h @3 :Float64;
  high24h @4 :Float64;
  low24h @5 :Float64;
  priceChange24h @6 :Float64;
  priceChangePercent24h @7 :Float64;
  timestamp @8 :Int64;
}

struct LiquidationLevel {
  price @0 :Float64;
  quantity @1 :Float64;
  side @2 :Trade.Side;
  leverage @3 :Float64;
}

struct LiquidationMap {
  symbol @0 :Text;
  levels @1 :List(LiquidationLevel);
  timestamp @2 :Int64;
}

struct CorrelationMatrix {
  symbols @0 :List(Text);
  matrix @1 :List(Float64);  # Flattened 2D matrix (row-major)
  timestamp @2 :Int64;
}

struct NewsItem {
  id @0 :Text;
  title @1 :Text;
  source @2 :Text;  # "twitter", "telegram", "binance", etc.
  content @3 :Text;
  timestamp @4 :Int64;
  impactScore @5 :UInt8;  # 1-10
  relatedSymbols @6 :List(Text);
  url @7 :Text;
}

struct Position {
  symbol @0 :Text;
  exchange @1 :Text;
  side @2 :Trade.Side;
  quantity @3 :Float64;
  entryPrice @4 :Float64;
  currentPrice @5 :Float64;
  unrealizedPnl @6 :Float64;
  unrealizedPnlPercent @7 :Float64;
  liquidationPrice @8 :Float64;
  leverage @9 :Float64;
  margin @10 :Float64;
}

struct Greeks {
  delta @0 :Float64;
  gamma @1 :Float64;
  vega @2 :Float64;
  theta @3 :Float64;
  rho @4 :Float64;
}

struct RiskMetrics {
  totalMargin @0 :Float64;
  totalUnrealizedPnl @1 :Float64;
  totalRealizedPnl @2 :Float64;
  portfolioGreeks @3 :Greeks;
  maxDrawdown @4 :Float64;
  sharpeRatio @5 :Float64;
}

# Event types for IPC streaming
struct MarketDataEvent {
  union {
    orderBookUpdate @0 :UnifiedOrderBook;
    trade @1 :Trade;
    ticker @2 :Ticker;
    liquidationMap @3 :LiquidationMap;
    correlationMatrix @4 :CorrelationMatrix;
    newsItem @5 :NewsItem;
    position @6 :Position;
    riskMetrics @7 :RiskMetrics;
  }
}
