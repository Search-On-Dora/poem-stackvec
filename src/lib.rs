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

#[cfg(test)]
mod tests {
    // Group tests for PoemSmallVec
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

    // Group tests for PoemArrayVec
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

    // Group tests for PoemHeaplessVec
    #[cfg(feature = "heapless")]
    mod heapless_tests {
        use crate::PoemHeaplessVec;
        use poem_openapi::types::ParseFromParameter;

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
    }
}
