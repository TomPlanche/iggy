// Expected: after an explicit disconnect(), requests fail until connect().
// Actual: with auto-login, the heartbeat (5 s default) reconnects and signs in again.

use iggy::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = IggyClient::from_connection_string("iggy+tcp://iggy:iggy@127.0.0.1:8090").unwrap();
    client.connect().await.unwrap();
    client.disconnect().await.unwrap();
    println!("right after disconnect: get_me -> {:?}", client.get_me().await.map(|_| ()));

    tokio::time::sleep(Duration::from_secs(7)).await;
    println!("7 s after disconnect:   get_me -> {:?}", client.get_me().await.map(|_| ()));
}
