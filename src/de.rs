use std::marker::PhantomData;

use crate::{
    error::{Error, Result},
    read::{Reader, SliceReader},
};
use serde::de::{self, value::U32Deserializer, IntoDeserializer};

/// Deserialize a value from bytes.
///
/// # Examples
///
/// ```
/// // Bring serde Serialize/Deserialize derivable traits into scope.
/// use serde::Deserialize;
///
/// // Define custom struct.
/// #[derive(Deserialize)]
/// struct MyStruct(u8, u8, u8);
///
/// // Create bytes to deserialize.
/// let bytes: [u8; 3] = [1, 2, 3];
///
/// // Deserialize value from bytes.
/// let value: MyStruct = shaft::from_bytes(&bytes).expect("deserialization error");
/// ```
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let reader = SliceReader::new(bytes);
    let mut deserializer = Deserializer::new(reader);
    let value = de::Deserialize::deserialize(&mut deserializer)?;
    Ok(value)
}

pub struct Deserializer<'a, R: Reader<'a>> {
    reader: R,
    phantom: PhantomData<&'a ()>,
}

impl<'a, R: Reader<'a>> Deserializer<'a, R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            phantom: PhantomData,
        }
    }
}

impl<'de: 'a, 'a, R: Reader<'de>> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_bool()?;
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_i8()?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_i16()?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_i32()?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_i64()?;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_u8()?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_u16()?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_u32()?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_u64()?;
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_f32()?;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_f64()?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.reader.read_u32()?;
        match char::from_u32(value) {
            Some(value) => visitor.visit_char(value),
            None => Err("deserialize: invalid char".into()),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = self.reader.read_u64()? as usize;
        let bytes = self.reader.read_bytes(len)?;
        let value = match std::str::from_utf8(bytes) {
            Ok(value) => value,
            Err(_) => return Err("deserialize: invalid str".into()),
        };
        visitor.visit_borrowed_str(value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = self.reader.read_u64()? as usize;
        let bytes = self.reader.read_bytes(len)?;
        visitor.visit_borrowed_bytes(bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let flag = self.reader.read_bool()?;
        if flag {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = self.reader.read_u64()? as usize;
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = self.reader.read_u64()? as usize;
        visitor.visit_map(MapAccess::new(self, len))
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(EnumAccess::new(self))
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }
}

struct SeqAccess<'a, 'b: 'a, R: Reader<'b>> {
    de: &'a mut Deserializer<'b, R>,
    len: usize,
}

impl<'a, 'b, R: Reader<'b>> SeqAccess<'a, 'b, R> {
    pub fn new(deserializer: &'a mut Deserializer<'b, R>, len: usize) -> Self {
        Self {
            de: deserializer,
            len,
        }
    }
}

impl<'a, 'b, R: Reader<'b>> de::SeqAccess<'b> for SeqAccess<'a, 'b, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'b>,
    {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(seed.deserialize(&mut *self.de)?))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapAccess<'a, 'b: 'a, R: Reader<'b>> {
    de: &'a mut Deserializer<'b, R>,
    len: usize,
}

impl<'a, 'b, R: Reader<'b>> MapAccess<'a, 'b, R> {
    pub fn new(deserializer: &'a mut Deserializer<'b, R>, len: usize) -> Self {
        Self {
            de: deserializer,
            len,
        }
    }
}

impl<'a, 'b, R: Reader<'b>> de::MapAccess<'b> for MapAccess<'a, 'b, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'b>,
    {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(seed.deserialize(&mut *self.de)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'b>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct EnumAccess<'a, 'b: 'a, R: Reader<'b>> {
    de: &'a mut Deserializer<'b, R>,
}

impl<'a, 'b: 'a, R: Reader<'b>> EnumAccess<'a, 'b, R> {
    fn new(de: &'a mut Deserializer<'b, R>) -> Self {
        Self { de }
    }
}

impl<'a, 'b: 'a, R: Reader<'b>> de::EnumAccess<'b> for EnumAccess<'a, 'b, R> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'b>,
    {
        let variant = self.de.reader.read_u32()?;
        let value = seed.deserialize::<U32Deserializer<Error>>(variant.into_deserializer())?;
        Ok((value, self))
    }
}

impl<'a, 'b: 'a, R: Reader<'b>> de::VariantAccess<'b> for EnumAccess<'a, 'b, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'b>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'b>,
    {
        de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'b>,
    {
        de::Deserializer::deserialize_tuple(self.de, fields.len(), visitor)
    }
}
