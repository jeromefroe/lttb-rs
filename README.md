# Largest Triangle Three Buckets

[![Build Status](https://travis-ci.org/jeromefroe/lttb-rs.svg?branch=master)](https://travis-ci.org/jeromefroe/lttb-rs)
[![codecov](https://codecov.io/gh/jeromefroe/lttb-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jeromefroe/lttb-rs)
[![crates.io](https://img.shields.io/crates/v/lttb.svg)](https://crates.io/crates/lttb/)
[![docs.rs](https://docs.rs/lttb/badge.svg)](https://docs.rs/lttb/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/jeromefroe/lttb-rs/master/LICENSE)

[Documentation](https://docs.rs/lttb/)

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
