use std::{borrow::Cow, ops::Deref};
use poem_openapi::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseFromParameter, ParseResult, Type},
};
use arrayvec::ArrayVec;

/// Fixed‑capacity ArrayVec<[T; SIZE]> wrapper for poem_openapi route params
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

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T: Type, const SIZE: usize> Type for PoemArrayVec<T, SIZE> {
    const IS_REQUIRED: bool = <[T; SIZE]>::IS_REQUIRED;
    type RawValueType = PoemArrayVec<T, SIZE>;
    type RawElementValueType = T;

    fn name() -> Cow<'static, str> {
        <[T; SIZE]>::name()
    }

    fn schema_ref() -> MetaSchemaRef {
        <[T; SIZE]>::schema_ref()
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
        let item = T::parse_from_parameter(value)
            .map_err(|e| ParseError::custom(e.message().to_string()))?;
        arr.try_push(item)
            .map_err(|_| ParseError::custom(format!("too many items (max {SIZE})")))?;
        Ok(PoemArrayVec(arr))
    }

    fn parse_from_parameters<I: IntoIterator<Item = A>, A: AsRef<str>>(
        iter: I,
    ) -> ParseResult<Self> {
        let mut arr = ArrayVec::new();
        for part in iter {
            let item = T::parse_from_parameter(part.as_ref())
                .map_err(|e| ParseError::custom(e.message().to_string()))?;
            arr.try_push(item)
                .map_err(|_| ParseError::custom(format!("too many items (max {SIZE})")))?;
        }
        Ok(PoemArrayVec(arr))
    }
}
