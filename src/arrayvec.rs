use std::{borrow::Cow, ops::Deref};
use poem_openapi::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseFromJSON, ParseFromParameter, ParseResult, Type},
};
use arrayvec::ArrayVec;

use crate::util::fixed_capacity_schema_ref;

/// `arrayvec::ArrayVec` wrapper that works in `poem_openapi` routes
#[derive(Debug)]
pub struct PoemArrayVec<T, const SIZE: usize>(pub ArrayVec<T, SIZE>);

impl<T, const SIZE: usize> PoemArrayVec<T, SIZE> {
    #[inline]
    pub fn new() -> Self {
        PoemArrayVec(ArrayVec::new())
    }
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T, const SIZE: usize> Default for PoemArrayVec<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const SIZE: usize> Deref for PoemArrayVec<T, SIZE> {
    type Target = [T];
    
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T: Type, const SIZE: usize> Type for PoemArrayVec<T, SIZE> {
    const IS_REQUIRED: bool = <[T; SIZE]>::IS_REQUIRED;
    type RawValueType = Self;
    type RawElementValueType = T;

    fn name() -> Cow<'static, str> {
        <[T; SIZE]>::name()
    }

    fn schema_ref() -> MetaSchemaRef {
        fixed_capacity_schema_ref::<T, SIZE>()
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.0.iter())
    }
}

impl<T: ParseFromParameter, const SIZE: usize> ParseFromParameter for PoemArrayVec<T, SIZE> {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        let mut arr = ArrayVec::new();
        parse_param(value, &mut arr)?;
        Ok(PoemArrayVec(arr))
    }

    fn parse_from_parameters<I: IntoIterator<Item = A>, A: AsRef<str>>(
        iter: I,
    ) -> ParseResult<Self> {
        let mut arr = ArrayVec::new();
        for part in iter {
            parse_param(part, &mut arr)?;
        }
        Ok(PoemArrayVec(arr))
    }
}

fn parse_param<S: AsRef<str>, T: ParseFromParameter, const N: usize>(value: S, vec: &mut ArrayVec<T, N>) -> Result<(), ParseError<PoemArrayVec<T, N>>> {
   let item = T::parse_from_parameter(value.as_ref())
        .map_err(ParseError::propagate)?;
    vec.try_push(item)
        .map_err(|_| ParseError::custom(format!("too many items (max {N})")))?;
    Ok(())
}

impl<T: ParseFromJSON, const SIZE: usize> ParseFromJSON for PoemArrayVec<T, SIZE> {
    fn parse_from_json(value: Option<serde_json::Value>) -> ParseResult<Self> {
        let value = value.unwrap_or_default();
        match value {
            serde_json::Value::Array(arr) => Ok(PoemArrayVec(
                arr.into_iter()
                    .map(|part| T::parse_from_json(Some(part)).map_err(ParseError::propagate))
                    .collect::<Result<_, _>>()?,
            )),
            _ => Err(ParseError::expected_type(value)),
        }
    }
}