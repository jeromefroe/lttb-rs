# Largest Triangle Three Buckets

An implementation of the largest triangle three buckets (lttb)
algorithm for time series downsampling as described in
[Downsampling Time Series for Visual Representation](https://skemman.is/bitstream/1946/15343/3/SS_MSthesis.pdf).
This is a Rust port of
[the original Javascript implementation](https://github.com/sveinn-steinarsson/flot-downsample).

## Example

``` rust
extern crate lttb;

use lttb::{DataPoint,lttb};

let mut raw = vec!();
raw.push(DataPoint::new(0.0, 10.0));
raw.push(DataPoint::new(1.0, 12.0));
raw.push(DataPoint::new(2.0, 8.0));
raw.push(DataPoint::new(3.0, 10.0));
raw.push(DataPoint::new(4.0, 12.0));

// Downsample the raw data to use just three datapoints.
let downsampled = lttb(raw, 3);
```
