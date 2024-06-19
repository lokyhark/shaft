use std::collections::HashMap;

use serde::Serialize;
use shaft::Error;

#[test]
fn serialize_bool_false() -> Result<(), Error> {
    let value = false;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0]);
    Ok(())
}

#[test]
fn serialize_bool_true() -> Result<(), Error> {
    let value = true;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1]);
    Ok(())
}

#[test]
fn serialize_i8_zero() -> Result<(), Error> {
    let value = 0i8;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0]);
    Ok(())
}

#[test]
fn serialize_i8_one() -> Result<(), Error> {
    let value = 1i8;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1]);
    Ok(())
}

#[test]
fn serialize_i8_neg() -> Result<(), Error> {
    let value = -1i8;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255]);
    Ok(())
}

#[test]
fn serialize_i8_min() -> Result<(), Error> {
    let value = i8::MIN;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [128]);
    Ok(())
}

#[test]
fn serialize_i8_max() -> Result<(), Error> {
    let value = i8::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [127]);
    Ok(())
}

#[test]
fn serialize_i16_zero() -> Result<(), Error> {
    let value = 0i16;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0]);
    Ok(())
}

#[test]
fn serialize_i16_one() -> Result<(), Error> {
    let value = 1i16;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 0]);
    Ok(())
}

#[test]
fn serialize_i16_neg() -> Result<(), Error> {
    let value = -1i16;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255]);
    Ok(())
}

#[test]
fn serialize_i16_min() -> Result<(), Error> {
    let value = i16::MIN;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 128]);
    Ok(())
}

#[test]
fn serialize_i16_max() -> Result<(), Error> {
    let value = i16::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 127]);
    Ok(())
}

#[test]
fn serialize_i32_zero() -> Result<(), Error> {
    let value = 0i32;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_i32_one() -> Result<(), Error> {
    let value = 1i32;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_i32_neg() -> Result<(), Error> {
    let value = -1i32;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255, 255, 255]);
    Ok(())
}

#[test]
fn serialize_i32_min() -> Result<(), Error> {
    let value = i32::MIN;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 128]);
    Ok(())
}

#[test]
fn serialize_i32_max() -> Result<(), Error> {
    let value = i32::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255, 255, 127]);
    Ok(())
}

#[test]
fn serialize_i64_zero() -> Result<(), Error> {
    let value = 0i64;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_i64_one() -> Result<(), Error> {
    let value = 1i64;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 0, 0, 0, 0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_i64_neg() -> Result<(), Error> {
    let value = -1i64;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255, 255, 255, 255, 255, 255, 255]);
    Ok(())
}

#[test]
fn serialize_i64_min() -> Result<(), Error> {
    let value = i64::MIN;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0, 128]);
    Ok(())
}

#[test]
fn serialize_i64_max() -> Result<(), Error> {
    let value = i64::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255, 255, 255, 255, 255, 255, 127]);
    Ok(())
}

#[test]
fn serialize_u8_zero() -> Result<(), Error> {
    let value = 0u8;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0]);
    Ok(())
}

#[test]
fn serialize_u8_one() -> Result<(), Error> {
    let value = 1u8;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1]);
    Ok(())
}

#[test]
fn serialize_u8_max() -> Result<(), Error> {
    let value = u8::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255]);
    Ok(())
}

#[test]
fn serialize_u16_zero() -> Result<(), Error> {
    let value = 0u16;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0]);
    Ok(())
}

#[test]
fn serialize_u16_one() -> Result<(), Error> {
    let value = 1u16;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 0]);
    Ok(())
}

#[test]
fn serialize_u16_max() -> Result<(), Error> {
    let value = u16::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255]);
    Ok(())
}

#[test]
fn serialize_u32_zero() -> Result<(), Error> {
    let value = 0u32;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_u32_one() -> Result<(), Error> {
    let value = 1u32;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_u32_max() -> Result<(), Error> {
    let value = u32::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255, 255, 255]);
    Ok(())
}

#[test]
fn serialize_u64_zero() -> Result<(), Error> {
    let value = 0u64;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_u64_one() -> Result<(), Error> {
    let value = 1u64;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 0, 0, 0, 0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_u64_max() -> Result<(), Error> {
    let value = u64::MAX;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [255, 255, 255, 255, 255, 255, 255, 255]);
    Ok(())
}

#[test]
fn serialize_f32_zero() -> Result<(), Error> {
    let value = 0.0f32;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_f32_one() -> Result<(), Error> {
    let value = 1.0f32;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0x80, 0x3f]);
    Ok(())
}

#[test]
fn serialize_f32_pi() -> Result<(), Error> {
    let value = std::f32::consts::PI;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0xdb, 0x0f, 0x49, 0x40]);
    Ok(())
}

#[test]
fn serialize_f32_inf() -> Result<(), Error> {
    let value = f32::INFINITY;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0x80, 0x7f]);
    Ok(())
}

#[test]
fn serialize_f32_neg() -> Result<(), Error> {
    let value = -f32::INFINITY;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0x80, 0xff]);
    Ok(())
}

#[test]
fn serialize_f32_nan() -> Result<(), Error> {
    let value = f32::NAN;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0xc0, 0x7f]);
    Ok(())
}

#[test]
fn serialize_f64_zero() -> Result<(), Error> {
    let value = 0.0f64;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_f64_one() -> Result<(), Error> {
    let value = 1.0f64;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0xf0, 0x3f]);
    Ok(())
}

#[test]
fn serialize_f64_pi() -> Result<(), Error> {
    let value = std::f64::consts::PI;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0x18, 0x2d, 0x44, 0x54, 0x0fb, 0x21, 0x09, 0x40]);
    Ok(())
}

#[test]
fn serialize_f64_inf() -> Result<(), Error> {
    let value = f64::INFINITY;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0xf0, 0x7f]);
    Ok(())
}

#[test]
fn serialize_f64_neg() -> Result<(), Error> {
    let value = -f64::INFINITY;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0xf0, 0xff]);
    Ok(())
}

#[test]
fn serialize_f64_nan() -> Result<(), Error> {
    let value = f64::NAN;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0xf8, 0x7f]);
    Ok(())
}

#[test]
fn serialize_char_one_byte() -> Result<(), Error> {
    let value = 'A';
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0x41, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_char_two_bytes() -> Result<(), Error> {
    let value = 'ß';
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0xdf, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_char_three_bytes() -> Result<(), Error> {
    let value = 'ᐁ';
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0x01, 0x14, 0, 0]);
    Ok(())
}

#[test]
fn serialize_char_four_bytes() -> Result<(), Error> {
    let value = '💣';
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0xa3, 0xf4, 0x01, 0]);
    Ok(())
}

#[test]
fn serialize_string() -> Result<(), Error> {
    let value = "Hello world!";
    let bytes = shaft::to_bytes(&value)?;
    let mut vec = vec![0u8; 8];
    vec[0] = value.len() as u8;
    vec.extend(b"Hello world!");
    assert_eq!(bytes, vec);
    Ok(())
}

#[test]
fn serialize_bytes() -> Result<(), Error> {
    let value = b"Hello world!".as_slice();
    let bytes = shaft::to_bytes(&value)?;
    let mut vec = vec![0u8; 8];
    vec[0] = value.len() as u8;
    vec.extend(b"Hello world!");
    assert_eq!(bytes, vec);
    Ok(())
}

#[test]
fn serialize_none() -> Result<(), Error> {
    let value: Option<()> = None;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0]);
    Ok(())
}

#[test]
fn serialize_some() -> Result<(), Error> {
    let value = Some(1u8);
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 1]);
    Ok(())
}

#[test]
fn serialize_unit() -> Result<(), Error> {
    let value = ();
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, []);
    Ok(())
}

#[test]
fn serialize_unit_struct() -> Result<(), Error> {
    #[derive(Serialize)]
    struct Unit;

    let value = Unit;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, []);
    Ok(())
}

#[test]
fn serialize_unit_variant() -> Result<(), Error> {
    #[derive(Serialize)]
    enum Enum {
        Variant,
    }

    let value = Enum::Variant;
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0]);
    Ok(())
}

#[test]
fn serialize_newtype_struct() -> Result<(), Error> {
    #[derive(Serialize)]
    struct NewType(u8);

    let value = NewType(1);
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1]);
    Ok(())
}

#[test]
fn serialize_newtype_variant() -> Result<(), Error> {
    #[derive(Serialize)]
    enum Enum {
        Variant(u8),
    }

    let value = Enum::Variant(1);
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 1]);
    Ok(())
}

#[test]
fn serialize_seq() -> Result<(), Error> {
    let value = vec![1, 2, 3u8];
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]);
    Ok(())
}

#[test]
fn serialize_tuple() -> Result<(), Error> {
    let value = (1u8, 2u8, 3u8);
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 2, 3]);
    Ok(())
}

#[test]
fn serialize_tuple_struct() -> Result<(), Error> {
    #[derive(Serialize)]
    struct Struct(u8, u8, u8);

    let value = Struct(1u8, 2u8, 3u8);
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 2, 3]);
    Ok(())
}

#[test]
fn serialize_tuple_variant() -> Result<(), Error> {
    #[derive(Serialize)]
    enum Enum {
        Variant(u8, u8, u8),
    }

    let value = Enum::Variant(1u8, 2u8, 3u8);
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 1, 2, 3]);
    Ok(())
}

#[test]
fn serialize_map() -> Result<(), Error> {
    let mut value = HashMap::new();
    value.insert(1u8, 2u8);
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 0, 0, 0, 0, 0, 0, 0, 1, 2]);
    Ok(())
}

#[test]
fn serialize_struct() -> Result<(), Error> {
    #[derive(Serialize)]
    struct Struct {
        a: u8,
        b: u8,
        c: u8,
    }

    let value = Struct { a: 1, b: 2, c: 3 };
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [1, 2, 3]);
    Ok(())
}

#[test]
fn serialize_struct_variant() -> Result<(), Error> {
    #[derive(Serialize)]
    enum Enum {
        Variant { a: u8, b: u8, c: u8 },
    }

    let value = Enum::Variant { a: 1, b: 2, c: 3 };
    let bytes = shaft::to_bytes(&value)?;
    assert_eq!(bytes, [0, 0, 0, 0, 1, 2, 3]);
    Ok(())
}
