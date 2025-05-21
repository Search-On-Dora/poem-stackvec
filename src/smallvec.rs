use std::{borrow::Cow, ops::Deref};
use poem_openapi::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseFromJSON, ParseFromParameter, ParseResult, Type},
};
use smallvec::{SmallVec, smallvec};

/// SmallVec<[T; SIZE]> wrapper that works in poem_openapi routes
#[derive(Debug)]
pub struct PoemSmallVec<T, const SIZE: usize>(pub SmallVec<[T; SIZE]>);

impl<T, const SIZE: usize> PoemSmallVec<T, SIZE> {
    #[inline]
    pub fn new() -> Self {
        PoemSmallVec(SmallVec::new())
    }
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T, const SIZE: usize> Default for PoemSmallVec<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const SIZE: usize> Deref for PoemSmallVec<T, SIZE> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T: Type, const SIZE: usize> Type for PoemSmallVec<T, SIZE> {
    const IS_REQUIRED: bool = Vec::<T>::IS_REQUIRED;
    type RawValueType = PoemSmallVec<T, SIZE>;
    type RawElementValueType = T;

    fn name() -> Cow<'static, str> {
        Vec::<T>::name()
    }
    fn schema_ref() -> MetaSchemaRef {
        Vec::<T>::schema_ref()
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

impl<T: ParseFromParameter, const SIZE: usize> ParseFromParameter for PoemSmallVec<T, SIZE> {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        match T::parse_from_parameter(value) {
            Ok(item) => Ok(PoemSmallVec(smallvec![item])),
            Err(err) => convert_err(value, err),
        }
    }
    fn parse_from_parameters<I: IntoIterator<Item = A>, A: AsRef<str>>(iter: I) -> ParseResult<Self> {
        let mut list = SmallVec::new();
        for part in iter {
            match T::parse_from_parameter(part.as_ref()) {
                Ok(item) => list.push(item),
                Err(err) => return convert_err(part, err),
            };
        }
        Ok(PoemSmallVec(list))
    }
}

impl<T: ParseFromJSON, const SIZE: usize> ParseFromJSON for PoemSmallVec<T, SIZE> {
    fn parse_from_json(value: Option<serde_json::Value>) -> ParseResult<Self> {
        let value = value.unwrap_or_default();
        match value {
            serde_json::Value::Array(arr) => Ok(PoemSmallVec(
                arr.into_iter()
                    .map(|part| T::parse_from_json(Some(part)).map_err(ParseError::propagate))
                    .collect::<Result<_, _>>()?,
            )),
            _ => Err(ParseError::expected_type(value)),
        }
    }
}

fn convert_err<A: AsRef<str>, T: Type, const SIZE: usize>(
    part: A,
    err: ParseError<T>,
) -> Result<PoemSmallVec<T, SIZE>, ParseError<PoemSmallVec<T, SIZE>>> {
    Err(ParseError::custom(format!(
        "failed to parse {part} as type {name}: {msg}",
        part = part.as_ref(),
        name = T::name(),
        msg = err.message()
    )))
}
