use std::collections::HashMap;

use serde::Deserialize;
use shaft::Error;

#[test]
fn deserialize_bool_false() -> Result<(), Error> {
    let bytes = [0];
    let value: bool = shaft::from_bytes(&bytes)?;
    assert!(!value);
    Ok(())
}

#[test]
fn deserialize_bool_true() -> Result<(), Error> {
    let bytes = [1];
    let value: bool = shaft::from_bytes(&bytes)?;
    assert!(value);
    Ok(())
}

#[test]
fn deserialize_i8_zero() -> Result<(), Error> {
    let bytes = [0];
    let value: i8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_i8_one() -> Result<(), Error> {
    let bytes = [1];
    let value: i8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_i8_neg() -> Result<(), Error> {
    let bytes = [255];
    let value: i8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, -1);
    Ok(())
}

#[test]
fn deserialize_i8_min() -> Result<(), Error> {
    let bytes = [128];
    let value: i8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i8::MIN);
    Ok(())
}

#[test]
fn deserialize_i8_max() -> Result<(), Error> {
    let bytes = [127];
    let value: i8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i8::MAX);
    Ok(())
}

#[test]
fn deserialize_i16_zero() -> Result<(), Error> {
    let bytes = [0, 0];
    let value: i16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_i16_one() -> Result<(), Error> {
    let bytes = [1, 0];
    let value: i16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_i16_neg() -> Result<(), Error> {
    let bytes = [255, 255];
    let value: i16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, -1);
    Ok(())
}

#[test]
fn deserialize_i16_min() -> Result<(), Error> {
    let bytes = [0, 128];
    let value: i16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i16::MIN);
    Ok(())
}

#[test]
fn deserialize_i16_max() -> Result<(), Error> {
    let bytes = [255, 127];
    let value: i16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i16::MAX);
    Ok(())
}

#[test]
fn deserialize_i32_zero() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0];
    let value: i32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_i32_one() -> Result<(), Error> {
    let bytes = [1, 0, 0, 0];
    let value: i32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_i32_neg() -> Result<(), Error> {
    let bytes = [255, 255, 255, 255];
    let value: i32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, -1);
    Ok(())
}

#[test]
fn deserialize_i32_min() -> Result<(), Error> {
    let bytes = [0, 0, 0, 128];
    let value: i32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i32::MIN);
    Ok(())
}

#[test]
fn deserialize_i32_max() -> Result<(), Error> {
    let bytes = [255, 255, 255, 127];
    let value: i32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i32::MAX);
    Ok(())
}

#[test]
fn deserialize_i64_zero() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0, 0];
    let value: i64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_i64_one() -> Result<(), Error> {
    let bytes = [1, 0, 0, 0, 0, 0, 0, 0];
    let value: i64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_i64_neg() -> Result<(), Error> {
    let bytes = [255, 255, 255, 255, 255, 255, 255, 255];
    let value: i64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, -1);
    Ok(())
}

#[test]
fn deserialize_i64_min() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0, 128];
    let value: i64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i64::MIN);
    Ok(())
}

#[test]
fn deserialize_i64_max() -> Result<(), Error> {
    let bytes = [255, 255, 255, 255, 255, 255, 255, 127];
    let value: i64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, i64::MAX);
    Ok(())
}

#[test]
fn deserialize_u8_zero() -> Result<(), Error> {
    let bytes = [0];
    let value: u8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_u8_one() -> Result<(), Error> {
    let bytes = [1];
    let value: u8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_u8_max() -> Result<(), Error> {
    let bytes = [255];
    let value: u8 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, u8::MAX);
    Ok(())
}

#[test]
fn deserialize_u16_zero() -> Result<(), Error> {
    let bytes = [0, 0];
    let value: u16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_u16_one() -> Result<(), Error> {
    let bytes = [1, 0];
    let value: u16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_u16_max() -> Result<(), Error> {
    let bytes = [255, 255];
    let value: u16 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, u16::MAX);
    Ok(())
}

#[test]
fn deserialize_u32_zero() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0];
    let value: u32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_u32_one() -> Result<(), Error> {
    let bytes = [1, 0, 0, 0];
    let value: u32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_u32_max() -> Result<(), Error> {
    let bytes = [255, 255, 255, 255];
    let value: u32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, u32::MAX);
    Ok(())
}

#[test]
fn deserialize_u64_zero() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0, 0];
    let value: u64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0);
    Ok(())
}

#[test]
fn deserialize_u64_one() -> Result<(), Error> {
    let bytes = [1, 0, 0, 0, 0, 0, 0, 0];
    let value: u64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1);
    Ok(())
}

#[test]
fn deserialize_u64_max() -> Result<(), Error> {
    let bytes = [255, 255, 255, 255, 255, 255, 255, 255];
    let value: u64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, u64::MAX);
    Ok(())
}

#[test]
fn deserialize_f32_zero() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0];
    let value: f32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0.0);
    Ok(())
}

#[test]
fn deserialize_f32_one() -> Result<(), Error> {
    let bytes = [0, 0, 0x80, 0x3f];
    let value: f32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1.0);
    Ok(())
}

#[test]
fn deserialize_f32_pi() -> Result<(), Error> {
    let bytes = [0xdb, 0x0f, 0x49, 0x40];
    let value: f32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, std::f32::consts::PI);
    Ok(())
}

#[test]
fn deserialize_f32_inf() -> Result<(), Error> {
    let bytes = [0, 0, 0x80, 0x7f];
    let value: f32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, f32::INFINITY);
    Ok(())
}

#[test]
fn deserialize_f32_neg() -> Result<(), Error> {
    let bytes = [0, 0, 0x80, 0xff];
    let value: f32 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, -f32::INFINITY);
    Ok(())
}

#[test]
fn deserialize_f32_nan() -> Result<(), Error> {
    let bytes = [0, 0, 0xc0, 0x7f];
    let value: f32 = shaft::from_bytes(&bytes)?;
    assert!(value.is_nan());
    Ok(())
}

#[test]
fn deserialize_f64_zero() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0, 0];
    let value: f64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 0.0);
    Ok(())
}

#[test]
fn deserialize_f64_one() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0xf0, 0x3f];
    let value: f64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 1.0);
    Ok(())
}

#[test]
fn deserialize_f64_pi() -> Result<(), Error> {
    let bytes = [0x18, 0x2d, 0x44, 0x54, 0x0fb, 0x21, 0x09, 0x40];
    let value: f64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, std::f64::consts::PI);
    Ok(())
}

#[test]
fn deserialize_f64_inf() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0xf0, 0x7f];
    let value: f64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, f64::INFINITY);
    Ok(())
}

#[test]
fn deserialize_f64_neg() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0xf0, 0xff];
    let value: f64 = shaft::from_bytes(&bytes)?;
    assert_eq!(value, -f64::INFINITY);
    Ok(())
}

#[test]
fn deserialize_f64_nan() -> Result<(), Error> {
    let bytes = [0, 0, 0, 0, 0, 0, 0xf8, 0x7f];
    let value: f64 = shaft::from_bytes(&bytes)?;
    assert!(value.is_nan());
    Ok(())
}

#[test]
fn deserialize_char_one_byte() -> Result<(), Error> {
    let bytes = [0x41, 0, 0, 0];
    let value: char = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 'A');
    Ok(())
}

#[test]
fn deserialize_char_two_bytes() -> Result<(), Error> {
    let bytes = [0xdf, 0, 0, 0];
    let value: char = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 'ß');
    Ok(())
}

#[test]
fn deserialize_char_three_bytes() -> Result<(), Error> {
    let bytes = [0x01, 0x14, 0, 0];
    let value: char = shaft::from_bytes(&bytes)?;
    assert_eq!(value, 'ᐁ');
    Ok(())
}

#[test]
fn deserialize_char_four_bytes() -> Result<(), Error> {
    let bytes = [0xa3, 0xf4, 0x01, 0];
    let value: char = shaft::from_bytes(&bytes)?;
    assert_eq!(value, '💣');
    Ok(())
}

#[test]
fn deserialize_string() -> Result<(), Error> {
    let string = "Hello world!";
    let len = string.len().to_le_bytes();
    let mut bytes = Vec::new();
    bytes.extend(len);
    bytes.extend(string.as_bytes());
    let value: String = shaft::from_bytes(&bytes)?;
    assert_eq!(value, "Hello world!");
    Ok(())
}

#[test]
fn deserialize_bytes() -> Result<(), Error> {
    let string = b"Hello world!";
    let len = string.len().to_le_bytes();
    let mut bytes = Vec::new();
    bytes.extend(len);
    bytes.extend(string);
    let value: &[u8] = shaft::from_bytes(&bytes)?;
    assert_eq!(value, b"Hello world!");
    Ok(())
}

#[test]
fn deserialize_none() -> Result<(), Error> {
    let bytes = [0];
    let value: Option<()> = shaft::from_bytes(&bytes)?;
    assert_eq!(value, None);
    Ok(())
}

#[test]
fn deserialize_some() -> Result<(), Error> {
    let bytes = [1, 1];
    let value: Option<u8> = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Some(1));
    Ok(())
}

#[test]
fn deserialize_unit() -> Result<(), Error> {
    let bytes = [];
    let _: () = shaft::from_bytes(&bytes)?;
    Ok(())
}

#[test]
fn deserialize_unit_struct() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Unit;

    let bytes = [];
    let value: Unit = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Unit);
    Ok(())
}

#[test]
fn deserialize_unit_variant() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Enum {
        Variant,
    }

    let bytes = [0, 0, 0, 0];
    let value: Enum = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Enum::Variant);
    Ok(())
}

#[test]
fn deserialize_newtype_struct() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    struct NewType(u8);

    let bytes = [1];
    let value: NewType = shaft::from_bytes(&bytes)?;
    assert_eq!(value, NewType(1));
    Ok(())
}

#[test]
fn deserialize_newtype_variant() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Enum {
        Variant(u8),
    }

    let bytes = [0, 0, 0, 0, 1];
    let value: Enum = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Enum::Variant(1));
    Ok(())
}

#[test]
fn deserialize_seq() -> Result<(), Error> {
    let bytes = [3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3];
    let value: Vec<u8> = shaft::from_bytes(&bytes)?;
    assert_eq!(value, vec![1, 2, 3u8]);
    Ok(())
}

#[test]
fn deserialize_tuple() -> Result<(), Error> {
    let bytes = [1, 2, 3];
    let value: (u8, u8, u8) = shaft::from_bytes(&bytes)?;
    assert_eq!(value, (1, 2, 3));
    Ok(())
}

#[test]
fn deserialize_tuple_struct() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Struct(u8, u8, u8);

    let bytes = [1, 2, 3];
    let value: Struct = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Struct(1, 2, 3));
    Ok(())
}

#[test]
fn deserialize_tuple_variant() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Enum {
        Variant(u8, u8, u8),
    }

    let bytes = [0, 0, 0, 0, 1, 2, 3];
    let value: Enum = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Enum::Variant(1, 2, 3));
    Ok(())
}

#[test]
fn deserialize_map() -> Result<(), Error> {
    let mut map = HashMap::new();
    map.insert(1u8, 2u8);
    let bytes = [1, 0, 0, 0, 0, 0, 0, 0, 1, 2];
    let value: HashMap<u8, u8> = shaft::from_bytes(&bytes)?;
    assert_eq!(value, map);
    Ok(())
}

#[test]
fn deserialize_struct() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Struct {
        a: u8,
        b: u8,
        c: u8,
    }

    let bytes = [1, 2, 3];
    let value: Struct = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Struct { a: 1, b: 2, c: 3 });
    Ok(())
}

#[test]
fn deserialize_struct_variant() -> Result<(), Error> {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Enum {
        Variant { a: u8, b: u8, c: u8 },
    }

    let bytes = [0, 0, 0, 0, 1, 2, 3];
    let value: Enum = shaft::from_bytes(&bytes)?;
    assert_eq!(value, Enum::Variant { a: 1, b: 2, c: 3 });
    Ok(())
}
