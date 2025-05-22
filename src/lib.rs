#[cfg(feature = "smallvec")]
pub mod smallvec;
#[cfg(feature = "smallvec")]
pub use smallvec::PoemSmallVec;

#[cfg(feature = "arrayvec")]
pub mod arrayvec;
#[cfg(feature = "arrayvec")]
pub use arrayvec::PoemArrayVec;

#[cfg(feature = "heapless")]
pub mod heapless;
#[cfg(feature = "heapless")]
pub use heapless::PoemHeaplessVec;

pub(crate) mod util;

#[cfg(test)]
mod tests {
    use poem_openapi::{registry::MetaSchemaRef, types::Type};

    pub(crate) fn check_openapi_props<T: Type, const MAX: usize>() 
        where <T as Type>::RawElementValueType: Type
    {
        match T::schema_ref() {
            MetaSchemaRef::Inline(meta) => {
                assert_eq!(meta.ty, "array");
                assert_eq!(meta.nullable, false);
                assert_eq!(meta.min_items, Some(1));
                assert_eq!(meta.max_items, Some(MAX));
                // (min/max)-length is supposed to be for strings, but poem uses it for arrays too?
                assert_eq!(meta.min_length, Some(1));
                assert_eq!(meta.max_length, Some(MAX));
                let max_str = MAX.to_string();
                let max = max_str.as_str();
                assert_eq!(meta.description, Some(format!("1 to {max} items of type {}", <T as Type>::RawElementValueType::name()).as_str()));
                assert_eq!(meta.items, Some(Box::new(T::RawElementValueType::schema_ref())));
            },
            MetaSchemaRef::Reference(s) => {
                panic!("expected Inline schema, got Reference: {s}");
            },
        }
    }

    // Group tests for PoemSmallVec
    #[cfg(feature = "smallvec")]
    mod smallvec_tests {
        use crate::PoemSmallVec;
        use poem_openapi::{registry::MetaSchemaRef, types::{ParseFromParameter, ParseFromJSON, Type}};
        use serde_json::json;

        #[test]
        fn specifies_correct_openapi_props() {
            match PoemSmallVec::<u32, 4>::schema_ref() {
                MetaSchemaRef::Inline(meta) => {
                    assert_eq!(meta.ty, "array");
                    assert_eq!(meta.nullable, false);
                    assert_eq!(meta.min_items, Some(1));
                    assert_eq!(meta.max_items, None);
                    assert_eq!(meta.min_length, Some(1));
                    assert_eq!(meta.max_length, None);
                    let elem_type_name = <PoemSmallVec::<u32, 4> as Type>::RawElementValueType::name();
                    assert_eq!(meta.title, Some(format!("Vec<{}>", elem_type_name)));
                    assert_eq!(meta.description, Some(format!("at least 1 item of type {}", elem_type_name)).as_deref());
                    assert_eq!(meta.items, Some(Box::new(u32::schema_ref())));
                },
                MetaSchemaRef::Reference(s) => {
                    panic!("expected Inline schema, got Reference: {s}");
                },
            }
        }

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

        #[test]
        fn parse_json_valid_array() {
            let value = json!([1, 2, 3]);
            let vec = PoemSmallVec::<i32, 4>::parse_from_json(Some(value))
                .expect("should parse valid JSON array");
            assert_eq!(vec.as_slice(), &[1, 2, 3]);
        }

        #[test]
        fn parse_json_invalid_type() {
            let value = json!({"not": "an array"});
            assert!(PoemSmallVec::<i32, 4>::parse_from_json(Some(value)).is_err());
        }
    }

    // Group tests for PoemArrayVec
    #[cfg(feature = "arrayvec")]
    mod arrayvec_tests {
        use crate::PoemArrayVec;
        use poem_openapi::types::{ParseFromJSON, ParseFromParameter};
        use serde_json::json;

        #[test]
        fn specifies_correct_openapi_props() {
            super::check_openapi_props::<PoemArrayVec<u16, 8>, 8>();
        }

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

        #[test]
        fn parse_json_valid_array() {
            let value = json!([1, 2, 3]);
            let vec = PoemArrayVec::<i32, 4>::parse_from_json(Some(value))
                .expect("should parse valid JSON array");
            assert_eq!(vec.as_slice(), &[1, 2, 3]);
        }

        #[test]
        fn parse_json_invalid_type() {
            let value = json!({"not": "an array"});
            assert!(PoemArrayVec::<i32, 4>::parse_from_json(Some(value)).is_err());
        }
    }

    // Group tests for PoemHeaplessVec
    #[cfg(feature = "heapless")]
    mod heapless_tests {
        use crate::PoemHeaplessVec;
        use poem_openapi::types::{ParseFromParameter, ParseFromJSON};
        use serde_json::json;

        #[test]
        fn specifies_correct_openapi_props() {
            super::check_openapi_props::<PoemHeaplessVec<u64, 2>, 2>();
        }

        #[test]
        fn parse_heapless_single_element() {
            let vec = PoemHeaplessVec::<i32, 4>::parse_from_parameter("42")
                .expect("should parse single element");
            assert_eq!(vec.as_slice(), &[42]);
        }

        #[test]
        fn parse_heapless_multiple_elements() {
            let input = vec!["1", "2", "3"];
            let vec = PoemHeaplessVec::<i32, 4>::parse_from_parameters(input)
                .expect("should parse multiple elements");
            assert_eq!(vec.as_slice(), &[1, 2, 3]);
        }

        #[test]
        fn parse_heapless_invalid_single() {
            assert!(PoemHeaplessVec::<i32, 4>::parse_from_parameter("not_a_number").is_err());
        }

        #[test]
        fn parse_heapless_invalid_multiple() {
            let input = vec!["10", "xx", "30"];
            assert!(PoemHeaplessVec::<i32, 4>::parse_from_parameters(input).is_err());
        }

        #[test]
        fn parse_json_valid_array() {
            let value = json!([1, 2, 3]);
            let vec = PoemHeaplessVec::<i32, 4>::parse_from_json(Some(value))
                .expect("should parse valid JSON array");
            assert_eq!(vec.as_slice(), &[1, 2, 3]);
        }

        #[test]
        fn parse_json_invalid_type() {
            let value = json!({"not": "an array"});
            assert!(PoemHeaplessVec::<i32, 4>::parse_from_json(Some(value)).is_err());
        }
    }
}
