use crypto_ws_client::{BinanceSpotWSClient, WSClient};

#[tokio::main]
async fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        let symbols = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
        let ws_client = BinanceSpotWSClient::new(tx, None).await;
        ws_client.subscribe_trade(&symbols).await;
        // run for 5 seconds
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ws_client.run()).await;
        ws_client.close();
    });

    for msg in rx {
        println!("{}", msg);
    }
}