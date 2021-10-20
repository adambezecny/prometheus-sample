use lazy_static::lazy_static;
use prometheus::{register_int_counter, Histogram, HistogramOpts, HistogramVec, IntCounter, Opts};

lazy_static! {
    pub static ref API_COUNTER: IntCounter =
        register_int_counter!("api_counter", "Counter of all API calls").unwrap();
    pub static ref API_DURATION1_MS: Histogram = Histogram::with_opts(HistogramOpts {
        common_opts: Opts::new("api_duration1_ms", "Histogram of call duration in ms"),
        buckets: vec![200f64, 500f64, 1000f64, 2000f64, 4000f64, 7000f64],
    })
    .unwrap();
    pub static ref API_DURATION2_MS: HistogramVec = prometheus::register_histogram_vec!(
        "api_duration2_ms",
        "Histogram of call duration in ms",
        &["dummy_dimension"],
        vec![200f64, 500f64, 1000f64, 2000f64, 4000f64, 7000f64]
    )
    .unwrap();
}
