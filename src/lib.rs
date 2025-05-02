use std::{borrow::Cow, ops::Deref};

use poem_openapi::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseFromJSON, ParseFromParameter, ParseResult, Type},
};

#[cfg(feature = "smallvec")]
use smallvec::{SmallVec, smallvec};

#[cfg(feature = "arrayvec")]
use arrayvec::ArrayVec;

#[cfg(feature = "smallvec")]
/// SmallVec<[T; SIZE]> wrapper that works in poem_openapi routes
#[derive(Debug)]
pub struct PoemSmallVec<T, const SIZE: usize>(pub SmallVec<[T; SIZE]>);

#[cfg(feature = "smallvec")]
impl<T, const SIZE: usize> PoemSmallVec<T, SIZE> {
    #[inline]
    pub fn new() -> Self {
        PoemSmallVec(SmallVec::new())
    }
    #[inline]
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

#[cfg(feature = "smallvec")]
impl<T, const SIZE: usize> Default for PoemSmallVec<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "smallvec")]
impl<T, const SIZE: usize> Deref for PoemSmallVec<T, SIZE> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

#[cfg(feature = "smallvec")]
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

#[cfg(feature = "smallvec")]
impl<T: ParseFromParameter, const SIZE: usize> ParseFromParameter for PoemSmallVec<T, SIZE> {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        match T::parse_from_parameter(value) {
            Ok(item) => Ok(PoemSmallVec(smallvec![item])),
            Err(err) => convert_err(value, err),
        }
    }

    fn parse_from_parameters<I: IntoIterator<Item = A>, A: AsRef<str>>(
        iter: I,
    ) -> ParseResult<Self> {
        let mut list = SmallVec::new();
        for part in iter {
            match T::parse_from_parameter(part.as_ref()) {
                Ok(item) => list.push(item),
                Err(err) => {
                    return convert_err(part, err);
                }
            };
        }
        Ok(PoemSmallVec(list))
    }
}

#[cfg(feature = "smallvec")]
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

#[cfg(feature = "smallvec")]
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

#[cfg(feature = "arrayvec")]
/// Fixed‑capacity ArrayVec<[T; SIZE]> wrapper for
/// poem_openapi route params (no heap fallback)
#[derive(Debug)]
pub struct PoemArrayVec<T, const SIZE: usize>(pub ArrayVec<T, SIZE>);

#[cfg(feature = "arrayvec")]
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

#[cfg(feature = "arrayvec")]
impl<T, const SIZE: usize> Default for PoemArrayVec<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "arrayvec")]
impl<T, const SIZE: usize> Deref for PoemArrayVec<T, SIZE> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

#[cfg(feature = "arrayvec")]
impl<T: Type, const SIZE: usize> Type for PoemArrayVec<T, SIZE> {
    const IS_REQUIRED: bool = Vec::<T>::IS_REQUIRED;
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

#[cfg(feature = "arrayvec")]
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

#[cfg(test)]
mod tests {
    // Group tests for PoemSmallVec behind the "smallvec" feature flag.
    #[cfg(feature = "smallvec")]
    mod smallvec_tests {
        use crate::PoemSmallVec;
        use poem_openapi::types::ParseFromParameter;

        #[test]
        fn parse_single_element() {
            let vec = PoemSmallVec::<i32, 4>::parse_from_parameter("42")
                .expect("should parse single element");
            assert_eq!(vec.as_slice(), &[42]);
        }

        #[test]
        fn parse_multiple_elements() {
            let input = vec!["1", "2", "3"];
            let vec = PoemSmallVec::<i32, 4>::parse_from_parameters(input)
                .expect("should parse multiple elements");
            assert_eq!(vec.as_slice(), &[1, 2, 3]);
        }

        #[test]
        fn parse_invalid_single() {
            assert!(PoemSmallVec::<i32, 4>::parse_from_parameter("not_a_number").is_err());
        }

        #[test]
        fn parse_invalid_multiple() {
            let input = vec!["10", "xx", "30"];
            assert!(PoemSmallVec::<i32, 4>::parse_from_parameters(input).is_err());
        }
    }

    // Group tests for PoemArrayVec behind the "arrayvec" feature flag.
    #[cfg(feature = "arrayvec")]
    mod arrayvec_tests {
        use crate::PoemArrayVec;
        use poem_openapi::types::ParseFromParameter;

        #[test]
        fn parse_array_single_element() {
            let vec = PoemArrayVec::<i32, 4>::parse_from_parameter("42")
                .expect("should parse single element");
            assert_eq!(vec.as_slice(), &[42]);
        }

        #[test]
        fn parse_array_multiple_elements() {
            let input = vec!["1", "2", "3"];
            let vec = PoemArrayVec::<i32, 4>::parse_from_parameters(input)
                .expect("should parse multiple elements");
            assert_eq!(vec.as_slice(), &[1, 2, 3]);
        }

        #[test]
        fn parse_invalid_array_single() {
            assert!(PoemArrayVec::<i32, 4>::parse_from_parameter("not_a_number").is_err());
        }

        #[test]
        fn parse_invalid_array_multiple() {
            let input = vec!["10", "xx", "30"];
            assert!(PoemArrayVec::<i32, 4>::parse_from_parameters(input).is_err());
        }
    }
}
