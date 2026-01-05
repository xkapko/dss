use std::boxed::Box;

#[derive(Debug, Clone)]
/// GapBuffer is a data structure for efficient work with strings. It allows very fast insertions
/// and deletions from any part of the string. It is represented as a buffer of bytes with an empty gap
/// in the middle. The insertion and deletion happens at the start and end of the gap. GapBuffer is
/// dynamic, meaning whenever the internal buffer runs out of space a new and bigger buffer is
/// allocated.
pub struct GapBuffer {
    left: usize,
    right: usize,
    pub capacity: usize,
    buffer: Box<[u8]>,
}

pub const GROW_BY: usize = 32;

impl Default for GapBuffer {
    fn default() -> Self {
        let mut vec = Vec::with_capacity(GROW_BY);
        vec.extend_from_slice(&[0; GROW_BY]);
        Self {
            left: 0,
            right: GROW_BY - 1,
            capacity: GROW_BY,
            buffer: vec.into_boxed_slice(),
        }
    }
}

impl ToString for GapBuffer {
    fn to_string(&self) -> String {
        let mut out = Vec::with_capacity(self.capacity);

        out.extend(
            self.buffer[0..self.left]
                .iter()
                .chain(&self.buffer[self.right + 1..]),
        );

        let string = String::from_utf8(out).expect("Unable to construct string from a GapBuffer.");
        string.chars().filter(|&c| c != '\0').collect()
    }
}

#[allow(unused)]
impl GapBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.fill(0);
        Self {
            left: 0,
            right: capacity - 1,
            capacity,
            buffer: vec.into_boxed_slice(),
        }
    }

    /// Grow the `GapBuffer` by `GROW_STEP` bytes.
    fn grow(&mut self) {
        let mut new_buff: Vec<u8> = Vec::with_capacity(self.capacity + GROW_BY);

        new_buff.extend_from_slice(&self.buffer[0..self.left]);
        new_buff.extend_from_slice(&vec![0; self.right + GROW_BY - self.left]);
        new_buff.extend_from_slice(&self.buffer[self.right..]);

        self.right += GROW_BY;
        self.capacity += GROW_BY;
        self.buffer = new_buff.into_boxed_slice();
    }

    /// Insert one byte at the current cursor position.
    /// If the gap is empty, grow the buffer as needed.
    pub fn insert_byte(&mut self, c: u8) {
        if self.left + 1 == self.right {
            self.grow()
        }

        // insert char at the start of the gap
        self.buffer[self.left] = c;
        self.left += 1;
    }

    /// Insert one char at the current cursor position.
    /// If the gap is empty, grow the buffer as needed.
    pub fn insert_char(&mut self, c: char) {
        self.insert(c.to_string().as_bytes())
    }

    /// Insert a slice of bytes on the current cursor position.
    /// If the buffer or gap is too small, grow the buffer as needed.
    pub fn insert(&mut self, slice: &[u8]) {
        // grow enough to accommodate the new slice
        let len = slice.len();
        if len == 0 {
            return;
        }

        while len > self.right - self.left {
            self.grow();
        }

        // insert the slice into the gap
        for (si, i) in (self.left..self.left + len).enumerate() {
            self.buffer[i] = slice[si];
        }

        self.left += len;
    }

    /// Insert a sting slice on the current cursor position.
    /// If the buffer or gap is too small, grow the buffer as needed.
    pub fn insert_str(&mut self, str: impl AsRef<str>) {
        self.insert(str.as_ref().as_bytes())
    }

    /// Move cursor to the left by `n` bytes. If `n` is too large the `left` gap index is set to
    /// zero.
    pub fn left_by(&mut self, n: usize) {
        let new_left = if n > self.left { 0 } else { self.left - n };
        let new_right = self.right - (self.left - new_left);

        for (l, r) in (new_left..self.left).zip(new_right + 1..) {
            self.buffer.swap(l, r);
        }

        self.left = new_left;
        self.right = new_right;
    }

    /// Move the cursor to the right by `n` bytes. If `n` is too large the `right` gap index is set
    /// to the last element.
    pub fn right_by(&mut self, n: usize) {
        let new_right = if self.right + n >= self.capacity {
            self.capacity - 1
        } else {
            self.right + n
        };
        let new_left = self.left + (new_right - self.right);

        for (r, l) in (self.right + 1..=new_right).zip(self.left..) {
            self.buffer.swap(r, l);
        }

        self.left = new_left;
        self.right = new_right;
    }

    /// Delete `n` bytes from the GapBuffer. Does nothing if the buffer is empty. The memory is
    /// not actually deleted or freed, the gap simply grows larger with each byte deleted.
    /// This funcion grows the buffer from the `left` side.
    pub fn delete_left(&mut self, n: usize) {
        self.left = if n > self.left { 0 } else { self.left - n }
    }

    /// Delete `n` bytes from the GapBuffer. Does nothing if the buffer is empty. The memory is
    /// not actually deleted or freed, the gap simply grows larger with each byte deleted.
    /// This funcion grows the buffer from the `right` side.
    pub fn delete_right(&mut self, n: usize) {
        self.right = if self.right + n > self.capacity - 1 {
            self.capacity - 1
        } else {
            self.right + n
        }
    }

    /// Clears the gap buffer.
    pub fn clear(&mut self) {
        self.right = self.capacity - 1;
        self.left = 0;
        self.buffer.fill(0);
    }

    /// Return the start and end indecies of the gap.
    pub fn gap(&self) -> (usize, usize) {
        (self.left, self.right)
    }

    #[cfg(test)]
    pub(crate) fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
