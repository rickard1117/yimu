#[derive(Debug)]
pub struct Buffer {
    data: [u8; BUFFER_SIZE],
    cap: usize,
}

const BUFFER_SIZE: usize = 32;

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            data: [0; BUFFER_SIZE],
            cap: BUFFER_SIZE,
        }
    }

    pub const fn len(&self) -> usize {
        BUFFER_SIZE - self.cap
    }

    pub const fn capacity(&self) -> usize {
        self.cap
    }

    pub fn append(&mut self, slice: &[u8]) {
        if self.cap < slice.len() {
            return;
        }
        let slen = slice.len();

        unsafe {
            std::ptr::copy_nonoverlapping(slice.as_ptr(), self.data.as_mut_ptr(), slen);
        }

        self.cap -= slen;
        return;
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
