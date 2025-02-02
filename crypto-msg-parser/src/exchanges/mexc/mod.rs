mod mexc_spot;
mod mexc_swap;

use std::collections::HashMap;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

pub(super) const EXCHANGE_NAME: &str = "mexc";

pub(crate) fn extract_symbol(_market_type_: MarketType, msg: &str) -> Result<String, SimpleError> {
    if let Ok(arr) = serde_json::from_str::<Vec<Value>>(msg) {
        Ok(arr[1]["symbol"].as_str().unwrap().to_string())
    } else if let Ok(json_obj) = serde_json::from_str::<HashMap<String, Value>>(msg) {
        Ok(json_obj
            .get("symbol")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        mexc_spot::parse_trade(msg)
    } else {
        mexc_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        mexc_spot::parse_l2(
            msg,
            timestamp.expect("MEXC Spot orderbook messages don't have timestamp"),
        )
    } else {
        mexc_swap::parse_l2(market_type, msg)
    }
}
