use crate::MarketType;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
pub struct Fees {
    pub maker: f64,
    pub taker: f64,
    pub percentage: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Precision {
    pub price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct MinQuantity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<f64>,
}

/// Market contains all information about a market
#[derive(Serialize, Deserialize)]
pub struct Market {
    /// exchange name
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// exchange-specific trading symbol, recognized by RESTful API, equivalent to ccxt's Market.id.
    pub symbol: String,
    /// exchange-specific base currency
    pub base_id: String,
    /// exchange-specific quote currency
    pub quote_id: String,
    /// unified uppercase string of base fiat or crypto currency
    pub base: String,
    /// unified uppercase string of quote fiat or crypto currency
    pub quote: String,
    /// market status
    pub active: bool,
    /// Margin enabled.
    ///
    /// * All contract markets are margin enabled, including future, swap and option.
    /// * Only a few exchanges have spot market with margin enabled.
    pub margin: bool,
    pub fees: Fees,
    /// number of decimal digits after the dot
    pub precision: Precision,
    /// minimum quantity when placing orders
    pub min_quantity: MinQuantity,
    // The value of one contract, not applicable to sport markets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_value: Option<f64>,
    /// Delivery date, unix timestamp in milliseconds, only applicable for future and option markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<u64>,
    /// the original JSON string retrieved from the exchange
    pub info: Map<String, Value>,
}
