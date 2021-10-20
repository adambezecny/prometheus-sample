use prometheus_sample::restapi;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    restapi::run_rest_api(socket);

    // https://blog.logrocket.com/using-prometheus-metrics-in-a-rust-web-service/

    println!("Listening 120 sec...");
    sleep(Duration::from_millis(120000)).await;
    println!("done!!!");
}
