use crate::error::Result;

pub trait Reader<'a> {
    fn read_bool(&mut self) -> Result<bool>;
    fn read_i8(&mut self) -> Result<i8>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;
    fn read_f32(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;
    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]>;
}

pub struct SliceReader<'a> {
    slice: &'a [u8],
    pos: usize,
}

impl<'a> SliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice, pos: 0 }
    }
}

impl<'a> Reader<'a> for SliceReader<'a> {
    fn read_bool(&mut self) -> Result<bool> {
        if self.pos >= self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let value = match self.slice[self.pos] {
            0 => false,
            1 => true,
            _ => return Err("reader: invalid bool".into()),
        };
        self.pos += 1;
        Ok(value)
    }

    fn read_i8(&mut self) -> Result<i8> {
        if self.pos >= self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let byte = self.slice[self.pos];
        let value = byte as i8;
        self.pos += 1;
        Ok(value)
    }

    fn read_i16(&mut self) -> Result<i16> {
        let end = self.pos + 2;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = i16::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 2;
        Ok(value)
    }

    fn read_i32(&mut self) -> Result<i32> {
        let end = self.pos + 4;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = i32::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 4;
        Ok(value)
    }

    fn read_i64(&mut self) -> Result<i64> {
        let end = self.pos + 8;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = i64::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 8;
        Ok(value)
    }

    fn read_u8(&mut self) -> Result<u8> {
        if self.pos >= self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let value = self.slice[self.pos];
        self.pos += 1;
        Ok(value)
    }

    fn read_u16(&mut self) -> Result<u16> {
        let end = self.pos + 2;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = u16::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 2;
        Ok(value)
    }

    fn read_u32(&mut self) -> Result<u32> {
        let end = self.pos + 4;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = u32::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 4;
        Ok(value)
    }

    fn read_u64(&mut self) -> Result<u64> {
        let end = self.pos + 8;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = u64::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 8;
        Ok(value)
    }

    fn read_f32(&mut self) -> Result<f32> {
        let end = self.pos + 4;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = f32::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 4;
        Ok(value)
    }

    fn read_f64(&mut self) -> Result<f64> {
        let end = self.pos + 8;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let bytes = &self.slice[self.pos..end];
        let value = f64::from_ne_bytes(bytes.try_into().unwrap());
        self.pos += 8;
        Ok(value)
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self.pos + len;
        if end > self.slice.len() {
            return Err("reader: reached end of slice".into());
        }
        let value = &self.slice[self.pos..end];
        self.pos += len;
        Ok(value)
    }
}
