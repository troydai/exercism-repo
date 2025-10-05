pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    phantom: std::marker::PhantomData<T>,
    data: Vec<Option<T>>,
    ridx: usize, // read index
    widx: usize, // write index
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
    UnexpectedError,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: std::iter::repeat_with(|| None).take(capacity).collect(),
            ridx: 0,
            widx: 0,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.data.get(self.widx).unwrap().is_some() {
            return Err(Error::FullBuffer);
        }

        self.data[self.widx] = Some(_element);
        self.widx = (self.widx + 1) % self.data.capacity();

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.data.get(self.ridx).unwrap().is_none() {
            return Err(Error::EmptyBuffer);
        }

        let retval = self.data[self.ridx].take();
        self.ridx = (self.ridx + 1) % self.data.capacity();

        Ok(retval.unwrap())
    }

    pub fn clear(&mut self) {
        self.data = std::iter::repeat_with(|| None).take(self.data.capacity()).collect();
        self.ridx = 0;
        self.widx = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.data.get(self.widx).unwrap().is_some() {
            self.ridx = (self.ridx + 1) % self.data.capacity();
        }

        self.data[self.widx] = Some(_element);
        self.widx = (self.widx + 1) % self.data.capacity();
    }
}
