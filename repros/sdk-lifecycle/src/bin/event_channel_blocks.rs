use iggy::prelude::*;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

const CYCLES: u32 = 600;
const STEP_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() {
    let config = TcpClientConfig {
        server_address: "127.0.0.1:8090".to_string(),
        reconnection: TcpClientReconnectionConfig {
            reestablish_after: IggyDuration::from_str("0s").unwrap(),
            ..TcpClientReconnectionConfig::default()
        },
        ..TcpClientConfig::default()
    };
    let client = TcpClient::create(Arc::new(config)).unwrap();

    for cycle in 1..=CYCLES {
        if tokio::time::timeout(STEP_TIMEOUT, client.connect()).await.is_err() {
            println!("connect {cycle} blocked");
            return;
        }
        if tokio::time::timeout(STEP_TIMEOUT, client.disconnect()).await.is_err() {
            println!("disconnect {cycle} blocked");
            return;
        }
    }

    println!("{CYCLES} cycles completed");
}
