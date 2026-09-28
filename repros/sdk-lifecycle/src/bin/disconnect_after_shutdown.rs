// Expected: after shutdown(), disconnect() keeps the client shut down and connect() fails.
// Actual: disconnect() resets the state to Disconnected, so connect() works again.

use iggy::prelude::*;

#[tokio::main]
async fn main() {
    for url in [
        "iggy+tcp://iggy:iggy@127.0.0.1:8090",
        "iggy+ws://iggy:iggy@127.0.0.1:8092",
        "iggy+quic://iggy:iggy@127.0.0.1:8080",
    ] {
        let client = IggyClient::from_connection_string(url).unwrap();
        client.connect().await.unwrap();
        client.shutdown().await.unwrap();
        client.disconnect().await.unwrap();
        println!("{url}: connect after shutdown + disconnect -> {:?}", client.connect().await);
    }
}
