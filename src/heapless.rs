use std::{borrow::Cow, ops::Deref};
use poem_openapi::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseFromJSON, ParseFromParameter, ParseResult, Type},
};
use heapless::Vec as HeaplessVec;

use crate::util::fixed_capacity_schema_ref;

/// `heapless::Vec` wrapper that works in `poem_openapi` routes
#[derive(Debug)]
pub struct PoemHeaplessVec<T, const N: usize>(pub HeaplessVec<T, N>);

impl<T, const N: usize> PoemHeaplessVec<T, N> {
    #[inline]
    pub fn new() -> Self {
        PoemHeaplessVec(HeaplessVec::new())
    }
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T, const N: usize> Default for PoemHeaplessVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Deref for PoemHeaplessVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T: Type, const N: usize> Type for PoemHeaplessVec<T, N> {
    const IS_REQUIRED: bool = <[T; N]>::IS_REQUIRED;
    type RawValueType = Self;
    type RawElementValueType = T;

    fn name() -> Cow<'static, str> {
        <[T; N]>::name()
    }

    fn schema_ref() -> MetaSchemaRef {
        fixed_capacity_schema_ref::<T, N>()
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.0.iter())
    }
}

impl<T: ParseFromParameter, const N: usize> ParseFromParameter for PoemHeaplessVec<T, N> {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        let mut vec = HeaplessVec::new();
        parse_param(value, &mut vec)?;
        Ok(PoemHeaplessVec(vec))
    }

    fn parse_from_parameters<I: IntoIterator<Item = A>, A: AsRef<str>>(iter: I) -> ParseResult<Self> {
        let mut vec = HeaplessVec::new();
        for part in iter {
            parse_param(part, &mut vec)?;
        }
        Ok(PoemHeaplessVec(vec))
    }
}

fn parse_param<S: AsRef<str>, T: ParseFromParameter, const N: usize>(value: S, vec: &mut HeaplessVec<T, N>) -> Result<(), ParseError<PoemHeaplessVec<T, N>>> {
    let item = T::parse_from_parameter(value.as_ref())
        .map_err(ParseError::propagate)?;
    vec.push(item)
        .map_err(|_| ParseError::custom(format!("too many items (max {N})")))?;
    Ok(())
}

impl<T: ParseFromJSON, const N: usize> ParseFromJSON for PoemHeaplessVec<T, N> {
    fn parse_from_json(value: Option<serde_json::Value>) -> ParseResult<Self> {
        let value = value.unwrap_or_default();
        match value {
            serde_json::Value::Array(arr) => {
                let mut vec = HeaplessVec::new();
                for part in arr {
                    let item = T::parse_from_json(Some(part))
                        .map_err(ParseError::propagate)?;
                    vec.push(item)
                        .map_err(|_| ParseError::custom(format!("too many items (max {N})")))?;
                }
                Ok(PoemHeaplessVec(vec))
            },
            _ => Err(ParseError::expected_type(value)),
        }
    }
}
