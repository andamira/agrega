// agrega::scanline::scan
//
//! Scanlines.
//!
//! This module provides structures and functions for working with scanlines,
//! representing contiguous rows of image data in the form of spans. Each span
//! defines a continuous region along a scanline with its own coverage values.
//

use alloc::{vec, vec::Vec};

const LAST_X: i64 = 0x7FFF_FFF0;

/// Represents a contiguous area of data along a scanline.
///
/// A `Span` includes a starting `x` position, a `len` specifying the
/// number of pixels covered, and a `covers` vector containing coverage
/// values for each pixel in the span.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct Span {
    /// Starting x-coordinate of the span.
    pub x: i64,
    /// Length of the span in pixels.
    pub len: i64,
    /// Coverage values for each pixel in the span.
    pub covers: Vec<u64>,
}

/// Represents an unpacked scanline for a single row of an image.
///
/// `ScanlineU8` is used to store spans and manage their properties within
/// an image row. The scanline maintains state variables to track
/// horizontal and vertical positions, as well as a collection of spans.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct ScanlineU8 {
    /// Last x-coordinate used in the scanline, acting as a state variable.
    last_x: i64,
    /// Minimum x-coordinate for this scanline. This is optional.
    min_x: i64,
    /// Current y-coordinate for the scanline, representing the row being processed.
    pub y: i64,
    /// Collection of spans that make up the scanline.
    pub spans: Vec<Span>,
    // /// Collection of covers (RETHINK: needed?)
    // covers: HashMap<i64, u64>,
}

impl ScanlineU8 {
    /// Creates a new, empty scanline with a pre-allocated capacity for spans.
    #[inline]
    pub fn new() -> Self {
        Self { last_x: LAST_X, min_x: 0, y: 0, spans: Vec::with_capacity(256) }
        //covers: HashMap::new() }
    }

    /// Resets the scanline by clearing all spans and setting the x-coordinate state variable.
    #[inline]
    pub fn reset_spans(&mut self) {
        self.last_x = LAST_X;
        self.spans.clear();
        //self.covers.clear();
    }

    /// Reset values and clear spans, setting min value
    #[inline]
    pub fn reset(&mut self, min_x: i64, _max_x: i64) {
        self.last_x = LAST_X;
        self.min_x = min_x;
        self.spans.clear();
        //self.covers = HashMap::new()
    }

    /// Sets the current row (y-coordinate) to the specified value.
    #[inline]
    pub fn finalize(&mut self, y: i64) {
        self.y = y;
    }

    /// Returns the total number of spans within the scanline.
    #[inline]
    #[must_use]
    pub fn num_spans(&self) -> usize {
        self.spans.len()
    }

    /// Adds a span to the scanline.
    ///
    /// Adds a span starting at `x`, with the specified `len` (length in pixels)
    /// and `cover` (coverage value for each pixel).
    ///
    /// If the `x` value is contiguous with the last span, the last span's length
    /// is increased instead of creating a new one.
    pub fn add_span(&mut self, x: i64, len: i64, cover: u64) {
        let x = x - self.min_x;
        //self.covers.insert( x, cover );
        if x == self.last_x + 1 {
            let cur = self.spans.last_mut().unwrap();
            cur.len += len;
            cur.covers.extend(vec![cover; len as usize]);
        } else {
            let span = Span { x: x + self.min_x, len, covers: vec![cover; len as usize] };
            self.spans.push(span);
        }
        self.last_x = x + len - 1;
    }

    /// Adds a single-pixel span (cell) with a specified coverage value.
    ///
    /// If the new cell is contiguous with the last span, it extends that span
    /// instead of creating a new one.
    pub fn add_cell(&mut self, x: i64, cover: u64) {
        let x = x - self.min_x;
        //self.covers.insert( x, cover );
        if x == self.last_x + 1 {
            let cur = self.spans.last_mut().unwrap();
            cur.len += 1;
            cur.covers.push(cover);
        } else {
            //let cover = self.covers.get(&x).unwrap().clone();
            let span = Span { x: x + self.min_x, len: 1, covers: vec![cover] };
            self.spans.push(span);
        }
        self.last_x = x;
    }
}
