// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::Map;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

impl<V> Drop for Map<V> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.head.as_ptr().cast(), self.layout);
        }
    }
}

impl<V: Clone> Map<V> {
    /// Make it.
    ///
    /// # Panics
    ///
    /// May panic if out of memory.
    #[inline]
    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        unsafe {
            let layout = Layout::array::<V>(cap).unwrap();
            let ptr = alloc(layout);
            Self {
                max: 0,
                layout,
                head: NonNull::new_unchecked(ptr.cast()),
            }
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn makes_new_map() -> Result<()> {
    let m: Map<&str> = Map::with_capacity(16);
    assert_eq!(0, m.len());
    Ok(())
}
