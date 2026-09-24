// With auto-login, after an explicit disconnect(): does a direct ping() reconnect?
// The heartbeat pings once right after connect(), so wait for that first tick to
// finish before disconnect(). The next tick comes after 5 s, well after this run.

use iggy::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = IggyClient::from_connection_string("iggy+tcp://iggy:iggy@127.0.0.1:8090").unwrap();
    client.connect().await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    client.disconnect().await.unwrap();

    println!("get_me -> {:?}", client.get_me().await.map(|_| ()));
    println!("ping   -> {:?}", client.ping().await);
    println!("get_me -> {:?}", client.get_me().await.map(|_| ()));
}
