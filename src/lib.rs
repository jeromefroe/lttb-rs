// MIT License

// Copyright (c) 2017 Jerome Froelich

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! An implementation of the largest triangle three buckets downsampling algorithm
//! as described in [Downsampling Time Series for Visual Representation]
//! (https://skemman.is/bitstream/1946/15343/3/SS_MSthesis.pdf). This is a Rust port
//! of [the original Javascript implementation]
//! (https://github.com/sveinn-steinarsson/flot-downsample).
//!
//! ## Example
//!
//! ``` rust,no_run
//! extern crate lttb;
//!
//! use lttb::{DataPoint,lttb};
//!
//! fn main() {
//!   let mut raw = vec!();
//!   raw.push(DataPoint::new(0.0, 10.0));
//!   raw.push(DataPoint::new(1.0, 12.0));
//!   raw.push(DataPoint::new(2.0, 8.0));
//!   raw.push(DataPoint::new(3.0, 10.0));
//!   raw.push(DataPoint::new(4.0, 12.0));
//!
//!   // Downsample the raw data to use just three datapoints.
//!   let downsampled = lttb(&raw[..], 3);
//! }
//! ```

extern crate num;

use std::error::Error;
use std::fmt;

use num::Float;
use num::cast::cast;

/// DataPoint
///
/// Struct used to represent a single datapoint in a time series.
#[derive(Debug, PartialEq, Copy)]
pub struct DataPoint<T: Float> {
    x: T,
    y: T,
}

impl<T: Float> Clone for DataPoint<T> {
    fn clone(&self) -> DataPoint<T> {
        *self
    }
}

impl<T: Float> DataPoint<T> {
    pub fn new(x: T, y: T) -> Self {
        DataPoint { x: x, y: y }
    }
}

#[derive(Debug)]
pub struct ConversionError;

// TODO: Improve error messages.
impl Error for ConversionError {
    fn description(&self) -> &str {
        "type conversion failed"
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type conversion failed")
    }
}

const CONVERSION_ERROR: ConversionError = ConversionError{};

pub fn lttb<T: Float>(data: &[DataPoint<T>], threshold: usize) -> Result<Option<Vec<DataPoint<T>>>, ConversionError> {
    if threshold >= data.len() || threshold == 0 {
        // Nothing to do.
        return Ok(None);
    }

    let mut sampled = Vec::with_capacity(threshold);

    // Bucket size. Leave room for start and end data points.
    let every = T::from(data.len() - 2).ok_or(CONVERSION_ERROR)? / T::from(threshold - 2).ok_or(CONVERSION_ERROR)?;

    // Initially a is the first point in the triangle.
    let mut a = 0;

    // Always add the first point.
    sampled.push(data[a]);

    for i in 0..threshold - 2 {
        // Calculate point average for next bucket (containing c).
        let mut avg_x = T::zero();
        let mut avg_y = T::zero();

        // TODO: Investigate further why the compiler is unable to infer the type properly
        // if we combine the following two lines into a single expression.
        let temp_cast: Option<usize> = cast(T::from(i + 1).ok_or(CONVERSION_ERROR)? * every);
        let avg_range_start = temp_cast.ok_or(CONVERSION_ERROR)? + 1;

        let temp_cast: Option<usize> = cast(T::from(i + 2).ok_or(CONVERSION_ERROR)? * every);
        let mut end = temp_cast.ok_or(CONVERSION_ERROR)? + 1;
        if end >= data.len() {
            end = data.len();
        }
        let avg_range_end = end;

        let avg_range_length = T::from(avg_range_end - avg_range_start).ok_or(CONVERSION_ERROR)?;

        for i in 0..(avg_range_end - avg_range_start) {
            let idx = (avg_range_start + i) as usize;
            avg_x = avg_x + data[idx].x;
            avg_y = avg_y + data[idx].y;
        }
        avg_x = avg_x / avg_range_length;
        avg_y = avg_y / avg_range_length;

        // Get the range for this bucket.
        let temp_cast: Option<usize> = cast(T::from(i + 0).ok_or(CONVERSION_ERROR)? * every);
        let range_offs = temp_cast.ok_or(CONVERSION_ERROR)? + 1;
        let temp_cast: Option<usize> = cast(T::from(i + 1).ok_or(CONVERSION_ERROR)? * every);
        let range_to =  temp_cast.ok_or(CONVERSION_ERROR)? + 1;

        // Point a.
        let point_a_x = data[a].x;
        let point_a_y = data[a].y;

        let mut max_area = T::from(-1).ok_or(CONVERSION_ERROR)?;
        let mut next_a = range_offs;
        for i in 0..(range_to - range_offs) {
            let idx = (range_offs + i) as usize;

            // Calculate triangle area over three buckets.
            let area = ((point_a_x - avg_x) * (data[idx].y - point_a_y) -
                        (point_a_x - data[idx].x) * (avg_y - point_a_y))
                .abs() * T::from(0.5).ok_or(CONVERSION_ERROR)?;
            if area > max_area {
                max_area = area;
                next_a = idx; // Next a is this b.
            }
        }

        sampled.push(data[next_a]);  // Pick this point from the bucket.
        a = next_a; // This a is the next a (chosen b).
    }

    // Always add the last point.
    sampled.push(data[data.len() - 1]);

    Ok(Some(sampled))
}

#[cfg(test)]
mod tests {
    use super::{DataPoint,lttb};

    #[test]
    fn lttb_test() {
        let mut dps = vec!();
        dps.push(DataPoint::new(0.0, 10.0));
        dps.push(DataPoint::new(1.0, 12.0));
        dps.push(DataPoint::new(2.0, 8.0));
        dps.push(DataPoint::new(3.0, 10.0));
        dps.push(DataPoint::new(4.0, 12.0));

        let mut expected = vec!();
        expected.push(DataPoint::new(0.0, 10.0));
        expected.push(DataPoint::new(2.0, 8.0));
        expected.push(DataPoint::new(4.0, 12.0));

        assert_eq!(expected, lttb(&dps[..], 3).unwrap().unwrap());
    }
}
