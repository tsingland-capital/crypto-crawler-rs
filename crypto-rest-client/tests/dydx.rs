use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::LinearSwap, "BTC-USD")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("dydx", market_type, symbol, Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    let asks = obj.get("asks").unwrap().as_array().unwrap();
    let bids = obj.get("bids").unwrap().as_array().unwrap();
    assert!(!asks.is_empty());
    assert!(!bids.is_empty());
}
