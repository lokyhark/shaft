use serde::{ser, Serialize};

use crate::{
    error::{Error, Result},
    write::{VecWriter, Writer},
};

/// Serialize a value into bytes.
///
/// # Examples
///
/// ```
/// // Bring serde Serialize/Deserialize derivable traits into scope.
/// use serde::Serialize;
///
/// // Define custom struct.
/// #[derive(Serialize)]
/// struct MyStruct {
///     name: String,
///     score: u32,
/// }
///
/// // Create value to serialize.
/// let value = MyStruct {
///    name: "Ferris".to_owned(),
///    score: 42,
/// };
///
/// // Serialize value into bytes.
/// let bytes = shaft::to_bytes(&value).expect("serialization error");
/// ```
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = VecWriter::new();
    to_writer(&mut writer, value)?;
    Ok(writer.into_vec())
}

pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: Writer,
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}

pub struct Serializer<W: Writer> {
    writer: W,
}

impl<W: Writer> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<'a, W: Writer> ser::Serializer for &'a mut Serializer<W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.writer.write_bool(value)
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.writer.write_i8(value)
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.writer.write_i16(value)
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.writer.write_i32(value)
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.writer.write_i64(value)
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.writer.write_u8(value)
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        self.writer.write_u16(value)
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        self.writer.write_u32(value)?;
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        self.writer.write_u64(value)
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        self.writer.write_f32(value)
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        self.writer.write_f64(value)
    }

    fn serialize_char(self, value: char) -> Result<()> {
        (value as u32).serialize(self)
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        let len = value.len();
        self.serialize_u64(len as u64)?;
        self.writer.write_bytes(value.as_bytes())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        let len = value.len();
        self.serialize_u64(len as u64)?;
        self.writer.write_bytes(value)
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_u8(0)
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u8(1)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(self, _: &'static str, index: u32, _: &'static str) -> Result<()> {
        self.serialize_u32(index)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        index: u32,
        _: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u32(index)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(len) => {
                self.serialize_u64(len as u64)?;
            }
            None => {
                return Err("serialize: unknown sequence length".into());
            }
        }
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        index: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_u32(index)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        match len {
            Some(len) => {
                self.serialize_u64(len as u64)?;
            }
            None => {
                return Err("serialize: unknown sequence length".into());
            }
        }
        Ok(self)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        index: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_u32(index)?;
        Ok(self)
    }
}

impl<'ser, W: Writer> ser::SerializeSeq for &'ser mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'ser, W: Writer> ser::SerializeTuple for &'ser mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'ser, W: Writer> ser::SerializeTupleStruct for &'ser mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'ser, W: Writer> ser::SerializeTupleVariant for &'ser mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'ser, W: Writer> ser::SerializeMap for &'ser mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'ser, W: Writer> ser::SerializeStruct for &'ser mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'ser, W: Writer> ser::SerializeStructVariant for &'ser mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
