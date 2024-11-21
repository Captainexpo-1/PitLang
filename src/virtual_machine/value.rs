use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::ptr::NonNull;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    Integer,
    Float,
    Boolean,
    String,
    Null,
    Object,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    type_tag: ValueType,
    data: u64, // Store integers, pointers, or other data here
}

impl Value {
    pub fn new_integer(value: i64) -> Self {
        Value {
            type_tag: ValueType::Integer,
            data: value as u64,
        }
    }

    pub fn new_float(value: f64) -> Self {
        Value {
            type_tag: ValueType::Float,
            data: value.to_bits(),
        }
    }

    pub fn new_boolean(value: bool) -> Self {
        Value {
            type_tag: ValueType::Boolean,
            data: value as u64,
        }
    }

    pub fn new_nil() -> Self {
        Value {
            type_tag: ValueType::Null,
            data: 0,
        }
    }

    pub fn new_object<T>(obj: T) -> Self {
        let boxed = Box::new(obj);
        let ptr = Box::into_raw(boxed);
        Value {
            type_tag: ValueType::Object,
            data: ptr as u64,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        if self.type_tag == ValueType::Integer {
            Some(self.data as i64)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if self.type_tag == ValueType::Float {
            Some(f64::from_bits(self.data))
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if self.type_tag == ValueType::Boolean {
            Some(self.data != 0)
        } else {
            None
        }
    }

    pub fn as_object<T>(&self) -> Option<&T> {
        if self.type_tag == ValueType::Object {
            unsafe { Some(&*(self.data as *const T)) }
        } else {
            None
        }
    }

    pub fn as_object_mut<T>(&mut self) -> Option<&mut T> {
        if self.type_tag == ValueType::Object {
            unsafe { Some(&mut *(self.data as *mut T)) }
        } else {
            None
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self.type_tag {
            ValueType::Null => false,
            ValueType::Boolean => self.as_boolean().unwrap(),
            _ => true,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.type_tag {
            ValueType::Integer => write!(f, "{}", self.as_integer().unwrap()),
            ValueType::Float => write!(f, "{}", self.as_float().unwrap()),
            ValueType::Boolean => write!(f, "{}", self.as_boolean().unwrap()),
            ValueType::Null => write!(f, "null"),
            ValueType::Object => write!(f, "Object"),
            ValueType::String => write!(f, "String"),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.type_tag, other.type_tag) {
            (ValueType::Integer, ValueType::Integer) => {
                Value::new_integer(self.as_integer().unwrap() + other.as_integer().unwrap())
            }
            (ValueType::Float, ValueType::Float) => {
                Value::new_float(self.as_float().unwrap() + other.as_float().unwrap())
            }
            (ValueType::String, ValueType::String) => {
                let mut s1 = self.as_object::<String>().unwrap().clone();
                let s2 = other.as_object::<String>().unwrap();
                s1.push_str(s2);
                Value::new_object(s1)
            }
            _ => panic!("Unsupported operation"),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self.type_tag, other.type_tag) {
            (ValueType::Integer, ValueType::Integer) => {
                Value::new_integer(self.as_integer().unwrap() - other.as_integer().unwrap())
            }
            (ValueType::Float, ValueType::Float) => {
                Value::new_float(self.as_float().unwrap() - other.as_float().unwrap())
            }
            _ => panic!("Unsupported operation"),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self.type_tag, other.type_tag) {
            (ValueType::Integer, ValueType::Integer) => {
                Value::new_integer(self.as_integer().unwrap() * other.as_integer().unwrap())
            }
            (ValueType::Float, ValueType::Float) => {
                Value::new_float(self.as_float().unwrap() * other.as_float().unwrap())
            }
            _ => panic!("Unsupported operation"),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self.type_tag, other.type_tag) {
            (ValueType::Integer, ValueType::Integer) => {
                Value::new_integer(self.as_integer().unwrap() / other.as_integer().unwrap())
            }
            (ValueType::Float, ValueType::Float) => {
                Value::new_float(self.as_float().unwrap() / other.as_float().unwrap())
            }
            _ => panic!("Unsupported operation"),
        }
    }
}
