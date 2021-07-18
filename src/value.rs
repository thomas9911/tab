use decorum::R64;
use derive_more::{From, IsVariant, Unwrap};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, From, IsVariant, Unwrap)]
/// simple non-recursive values
pub enum Value {
    String(String),
    Byte(u8),
    Unsigned(u128),
    Signed(i128),
    Float(R64),
    Bool(bool),
    None,
}

impl Value {
    pub fn from_str_no_bytes(src: &str) -> Result<Value, ()> {
        match src.parse::<Value>() {
            Ok(Value::Byte(x)) => Ok(Value::Unsigned(x.into())),
            other => other,
        }
    }
}

macro_rules! impl_from_signed {
    ($type: ty) => {
        impl From<$type> for Value {
            fn from(input: $type) -> Value {
                Value::Signed(input as i128)
            }
        }
    };
}

macro_rules! impl_from_unsigned {
    ($type: ty) => {
        impl From<$type> for Value {
            fn from(input: $type) -> Value {
                Value::Unsigned(input as u128)
            }
        }
    };
}

impl_from_signed!(i8);
impl_from_signed!(i16);
impl_from_signed!(i32);
impl_from_signed!(i64);
impl_from_signed!(isize);

impl_from_unsigned!(u16);
impl_from_unsigned!(u32);
impl_from_unsigned!(u64);
impl_from_unsigned!(usize);

impl From<&str> for Value {
    fn from(input: &str) -> Value {
        Value::String(input.to_string())
    }
}

impl From<f64> for Value {
    fn from(input: f64) -> Value {
        Value::Float(input.into())
    }
}

impl From<f32> for Value {
    fn from(input: f32) -> Value {
        Value::Float((input as f64).into())
    }
}

impl std::str::FromStr for Value {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src.to_lowercase().as_ref() {
            "true" => return Ok(value(true)),
            "false" => return Ok(value(false)),
            _ => (),
        };

        if let Ok(x) = src.parse::<u8>() {
            return Ok(Value::Byte(x));
        }
        if let Ok(x) = src.parse::<u128>() {
            return Ok(Value::Unsigned(x));
        }
        if let Ok(x) = src.parse::<i128>() {
            return Ok(Value::Signed(x));
        }
        if let Ok(x) = src.parse::<R64>() {
            return Ok(Value::Float(x));
        }

        Ok(Value::String(src.to_string()))
    }
}

pub fn value<V>(input: V) -> Value
where
    V: Into<Value>,
{
    input.into()
}

#[test]
fn value_test() {
    assert_eq!(Value::Byte(123u8), value(123u8));
    assert_eq!(Value::Unsigned(123), value(123usize));
    assert_eq!(Value::Signed(-123), value(-123i8));
    assert_eq!(Value::Signed(-123), value(-123));
    assert_eq!(Value::String(String::from("testing")), value("testing"));
    assert_eq!(
        Value::String(String::from("testing")),
        value(String::from("testing"))
    );
    assert_eq!(Value::Float(1.25.into()), value(1.25f32));
    assert_eq!(Value::Float(1.25.into()), value(1.25f64));
    assert_eq!(Value::Bool(false), value(false));
    assert_eq!(Value::Bool(true), value(true));
}

#[test]
fn value_parse_test() {
    assert_eq!(Value::Byte(123u8), "123".parse().unwrap());
    assert_eq!(Value::Unsigned(123456), "123456".parse().unwrap());
    assert_eq!(Value::Signed(-123456), "-123456".parse().unwrap());
    assert_eq!(Value::String(String::from("test")), "test".parse().unwrap());
    assert_eq!(Value::Bool(false), "False".parse().unwrap());
    assert_eq!(Value::Bool(true), "TRUE".parse().unwrap());
    assert_eq!(Value::Float(1.235.into()), "1.235".parse().unwrap());
    assert_eq!(Value::Float(123.5.into()), "1.235e2".parse().unwrap());
    assert_eq!(Value::Float((-1.235).into()), "-1.235".parse().unwrap());
}
