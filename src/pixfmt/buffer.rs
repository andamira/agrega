//! Rendering buffer

use alloc::{vec, vec::Vec};
use core::ops::{Index, IndexMut};

/// A rendering buffer for storing image pixel data in row-major order (C-format).
#[derive(Clone, Debug, Default)]
pub(crate) struct RenderingBuffer {
    /// Pixel/component-level data for the image.
    pub data: Vec<u8>,
    /// Width of the image in pixels.
    pub width: usize,
    /// Height of the image in pixels.
    pub height: usize,
    /// Bytes per pixel or the number of color components per pixel.
    pub bpp: usize,
}

impl RenderingBuffer {
    /// Creates a new `RenderingBuffer` with the given `width`, `height`, and `bpp`.
    ///
    /// Allocates the `data` buffer to hold `width * height * bpp` bytes, initialized to zero.
    #[inline]
    pub fn new(width: usize, height: usize, bpp: usize) -> Self {
        RenderingBuffer { width, height, bpp, data: vec![0; width * height * bpp] }
    }

    /// Returns the total size of the underlying data buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Clears the buffer, setting all pixel values to `255` (white or fully opaque).
    #[inline]
    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|v| *v = 255);
    }

    /// Creates a `RenderingBuffer` from an existing `data` vector.
    ///
    /// # Panics
    /// Panics if `data.len()` does not equal `width * height * bpp`.
    #[inline]
    pub fn from_vec(data: Vec<u8>, width: usize, height: usize, bpp: usize) -> Self {
        assert_eq!(data.len(), width * height * bpp);
        RenderingBuffer { width, height, bpp, data }
    }
}

impl Index<(usize, usize)> for RenderingBuffer {
    type Output = [u8];
    #[rustfmt::skip]
    fn index(&self, index: (usize, usize)) -> &[u8] {
        debug_assert!(index.0 < self.width, "request {} >= {} width :: index",
            index.0, self.width);
        debug_assert!(index.1 < self.height, "request {} >= {} height :: index",
            index.1, self.height);
        let i = ((index.1 * self.width) + index.0) * self.bpp;
        debug_assert!(i < self.data.len());
        &self.data[i..]
    }
}
impl IndexMut<(usize, usize)> for RenderingBuffer {
    #[rustfmt::skip]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut [u8] {
        debug_assert!(index.0 < self.width, "request {} >= {} width :: index_mut",
            index.0, self.width);
        debug_assert!(index.1 < self.height, "request {} >= {} height :: index_mut",
            index.1, self.height);
        let i = ((index.1 * self.width) + index.0) * self.bpp;
        debug_assert!(i < self.data.len());
        &mut self.data[i..]
    }
}
