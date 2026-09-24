// Expected: connect() after shutdown() fails with ClientShutdown on every transport.
// Actual: TCP refuses, WebSocket connects again.

use iggy::prelude::*;

#[tokio::main]
async fn main() {
    for url in ["iggy+tcp://iggy:iggy@127.0.0.1:8090", "iggy+ws://iggy:iggy@127.0.0.1:8092"] {
        let client = IggyClient::from_connection_string(url).unwrap();
        client.connect().await.unwrap();
        client.shutdown().await.unwrap();
        println!("{url}: connect after shutdown -> {:?}", client.connect().await);
    }
}
