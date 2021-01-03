use crate::WSClient;
use std::collections::HashMap;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Ticker, Trade,
};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "CoinbasePro";

const WEBSOCKET_URL: &str = "wss://ws-feed.pro.coinbase.com";

/// The WebSocket client for CoinbasePro.
///
/// CoinbasePro has only Spot market.
///
///   * WebSocket API doc: <https://docs.pro.coinbase.com/#websocket-feed>
///   * Trading at: <https://pro.coinbase.com/>
pub struct CoinbaseProWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_pairs_to_command(channel: &str, pairs: &[String]) -> String {
    format!(
        r#"{{"name":"{}","product_ids":{}}}"#,
        channel,
        serde_json::to_string(pairs).unwrap(),
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut channel_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let channel = v[0];
        let pair = v[1];
        match channel_pairs.get_mut(channel) {
            Some(pairs) => pairs.push(pair.to_string()),
            None => {
                channel_pairs.insert(channel.to_string(), vec![pair.to_string()]);
            }
        }
    }

    let mut command = String::new();
    command.push_str(
        format!(
            r#"{{"type":"{}","channels": ["#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            }
        )
        .as_str(),
    );
    for (channel, pairs) in channel_pairs.iter() {
        command.push_str(channel_pairs_to_command(channel, pairs).as_str());
        command.push(',')
    }
    command.pop();
    command.push_str("]}");

    vec![command]
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    match obj.get("type").unwrap().as_str().unwrap() {
        "error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "subscriptions" => {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "heartbeat" => {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        _ => MiscMessage::Normal,
    }
}

impl<'a> Trade for CoinbaseProWSClient<'a> {
    fn subscribe_trade(&mut self, pairs: &[String]) {
        let pair_to_raw_channel =
            |pair: &String| format!("matches{}{}", CHANNEL_PAIR_DELIMITER, pair);

        let channels = pairs
            .iter()
            .map(pair_to_raw_channel)
            .collect::<Vec<String>>();
        self.client.subscribe(&channels);
    }
}

impl<'a> Ticker for CoinbaseProWSClient<'a> {
    fn subscribe_ticker(&mut self, pairs: &[String]) {
        let pair_to_raw_channel =
            |pair: &String| format!("ticker{}{}", CHANNEL_PAIR_DELIMITER, pair);

        let channels = pairs
            .iter()
            .map(pair_to_raw_channel)
            .collect::<Vec<String>>();
        self.client.subscribe(&channels);
    }
}

define_client!(
    CoinbaseProWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_two_pairs() {
        assert_eq!(
            r#"{"name":"matches","product_ids":["BTC-USD","ETH-USD"]}"#,
            super::channel_pairs_to_command(
                "matches",
                &vec!["BTC-USD".to_string(), "ETH-USD".to_string()],
            )
        );
    }
}
