use crate::error::Result;

pub trait Writer {
    fn write_bool(&mut self, value: bool) -> Result<()>;
    fn write_i8(&mut self, value: i8) -> Result<()>;
    fn write_i16(&mut self, value: i16) -> Result<()>;
    fn write_i32(&mut self, value: i32) -> Result<()>;
    fn write_i64(&mut self, value: i64) -> Result<()>;
    fn write_u8(&mut self, value: u8) -> Result<()>;
    fn write_u16(&mut self, value: u16) -> Result<()>;
    fn write_u32(&mut self, value: u32) -> Result<()>;
    fn write_u64(&mut self, value: u64) -> Result<()>;
    fn write_f32(&mut self, value: f32) -> Result<()>;
    fn write_f64(&mut self, value: f64) -> Result<()>;
    fn write_bytes(&mut self, value: &[u8]) -> Result<()>;
}

impl<W: Writer + ?Sized> Writer for &mut W {
    fn write_bool(&mut self, value: bool) -> Result<()> {
        (**self).write_bool(value)
    }

    fn write_i8(&mut self, value: i8) -> Result<()> {
        (**self).write_i8(value)
    }

    fn write_i16(&mut self, value: i16) -> Result<()> {
        (**self).write_i16(value)
    }

    fn write_i32(&mut self, value: i32) -> Result<()> {
        (**self).write_i32(value)
    }

    fn write_i64(&mut self, value: i64) -> Result<()> {
        (**self).write_i64(value)
    }

    fn write_u8(&mut self, value: u8) -> Result<()> {
        (**self).write_u8(value)
    }

    fn write_u16(&mut self, value: u16) -> Result<()> {
        (**self).write_u16(value)
    }

    fn write_u32(&mut self, value: u32) -> Result<()> {
        (**self).write_u32(value)
    }

    fn write_u64(&mut self, value: u64) -> Result<()> {
        (**self).write_u64(value)
    }

    fn write_f32(&mut self, value: f32) -> Result<()> {
        (**self).write_f32(value)
    }

    fn write_f64(&mut self, value: f64) -> Result<()> {
        (**self).write_f64(value)
    }

    fn write_bytes(&mut self, value: &[u8]) -> Result<()> {
        (**self).write_bytes(value)
    }
}

pub struct VecWriter {
    buf: Vec<u8>,
}

impl VecWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }
}

impl Writer for VecWriter {
    fn write_bool(&mut self, value: bool) -> Result<()> {
        if value {
            self.buf.push(1);
        } else {
            self.buf.push(0);
        }
        Ok(())
    }

    fn write_i8(&mut self, value: i8) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_i16(&mut self, value: i16) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_i32(&mut self, value: i32) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_i64(&mut self, value: i64) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_u16(&mut self, value: u16) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_u32(&mut self, value: u32) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_u64(&mut self, value: u64) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_f32(&mut self, value: f32) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_f64(&mut self, value: f64) -> Result<()> {
        self.buf.extend(value.to_le_bytes());
        Ok(())
    }

    fn write_bytes(&mut self, value: &[u8]) -> Result<()> {
        self.buf.extend(value);
        Ok(())
    }
}
