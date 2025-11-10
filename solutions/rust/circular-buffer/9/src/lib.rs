pub struct CircularBuffer<T: Clone + Default> {
    data: Vec<T>,
    idx: usize, // the current position to read from
    len: usize, // the length of the buffer
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![Default::default(); capacity],
            idx: 0,
            len: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.data.capacity() {
            return Err(Error::FullBuffer);
        }

        let write_idx = (self.idx + self.len) % self.data.capacity();
        self.data[write_idx] = element;
        self.len += 1;

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        }

        let retval = std::mem::take(&mut self.data[self.idx]);
        self.idx = (self.idx + 1) % self.data.capacity();
        self.len -= 1;

        Ok(retval)
    }

    pub fn clear(&mut self) {
        self.data = vec![Default::default(); self.data.capacity()];
        self.idx = 0;
        self.len = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.len != self.data.capacity() {
            self.write(element).unwrap();
            return;
        }

        self.data[self.idx] = element;
        self.idx = (self.idx + 1) % self.data.capacity();
    }
}
