use crate::metrics;
use prometheus::{self, Encoder, TextEncoder};
use std::net::SocketAddr;
use std::result::Result as StdResult;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use warp::{http::StatusCode, Filter, Rejection, Reply};

async fn about_handler() -> StdResult<impl Reply, Rejection> {
    metrics::API_COUNTER.inc();

    let timer = metrics::API_DURATION1_MS.start_timer();
    let timer2 = Instant::now();
    sleep(Duration::from_millis(3000)).await;
    let call_duration = timer.stop_and_record();
    let call_duration2 = timer2.elapsed().as_millis() as f64;
    println!("DURATION {} sec", call_duration);
    println!("DURATION2 {} ms", call_duration2);
    metrics::API_DURATION1_MS.observe(call_duration * 1000.0);

    metrics::API_DURATION2_MS
        .with_label_values(&[""])
        .observe(call_duration2);

    Ok(warp::reply::with_status(
        "REST API is running here!!!",
        StatusCode::OK,
    ))
}

async fn metrics_handler() -> StdResult<impl Reply, Rejection> {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();

    // Encode them to send.
    if let Err(encoding_err) = encoder.encode(&metric_families, &mut buffer) {
        println!("metrics_handler encoding error: {:?}", encoding_err);
        Ok(warp::reply::with_status(
            "Encoding error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        match String::from_utf8(buffer) {
            Err(from_utf8_err) => {
                println!("metrics_handler from_utf8 error: {:?}", from_utf8_err);
                Ok(warp::reply::with_status(
                    "UTF8 conversion error".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
            Ok(metrics_str) => Ok(warp::reply::with_status(metrics_str, StatusCode::OK)),
        }
    }
}

pub fn run_rest_api(socket_addr: SocketAddr) {
    let about_route = warp::get()
        .and(warp::path("about"))
        .and(warp::path::end())
        .and_then(about_handler);

    let metrics_route = warp::get()
        .and(warp::path("metrics"))
        .and(warp::path::end())
        .and_then(metrics_handler);

    let routes = about_route.or(metrics_route);

    tokio::spawn(async move {
        warp::serve(routes).run(socket_addr).await;
    });
}
