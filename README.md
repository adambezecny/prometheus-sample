# Prometheus Histogram not working - minimal reproducible example

This sample demonstrates that while *HistogramVec* works fine *Histogram* does not report observed values properly.

How to use:

```
cargo run
```

This will create warp server with two APIs:

* http://localhost:8080/about
* http://localhost:8080/metrics

Server will run for 2 minutes and then exit.

Running *About API* (http://localhost:8080/about) in browser returns static response *REST API is running here!!!*.

At the same time it updates following metrics:

* *API_COUNTER* (invocation counter)
* *API_DURATION1_MS* (duration via Histogram)
* *API_DURATION2_MS* (duration via HistogramVec)

Each api invocation is doing fixed delay for 3 seconds.

*Metrics API* (http://localhost:8080/metrics) will always return just *API_COUNTER* and *API_DURATION2_MS*. **Histogram metric *API_DURATION1_MS* is never reported properly.**

Example:

```
# HELP api_counter Counter of all API calls
# TYPE api_counter counter
api_counter 4
# HELP api_duration2_ms Histogram of call duration in ms
# TYPE api_duration2_ms histogram
api_duration2_ms_bucket{dummy_dimension="",le="200"} 0
api_duration2_ms_bucket{dummy_dimension="",le="500"} 0
api_duration2_ms_bucket{dummy_dimension="",le="1000"} 0
api_duration2_ms_bucket{dummy_dimension="",le="2000"} 0
api_duration2_ms_bucket{dummy_dimension="",le="4000"} 4
api_duration2_ms_bucket{dummy_dimension="",le="7000"} 4
api_duration2_ms_bucket{dummy_dimension="",le="+Inf"} 4
api_duration2_ms_sum{dummy_dimension=""} 12009.1373
api_duration2_ms_count{dummy_dimension=""} 4
```

