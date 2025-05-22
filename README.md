# Stack-allocated `poem` Vectors

This create exports 3 different implementations of a `Vec<T>`-like collection that is: 
* able to be parsed like a `Vec<T>` would be, in HTTP route parameters of `poem-openapi` servers.
* allocated on the stack

You would use these to reduce the amount of allocations involved in parsing route parameters, as well as to specify slightly more exact behavior than `Vec<T>`.

## Features

The three vector types exported correspond directly to crate features:
* `PoemArrayVec` is exported by the `arrayvec` feature
* `PoemHeaplessVec` is exported by the `heapless` feature
* `PoemSmallVec` is exported by the `smallvec` feature

## Usage

Use one of the 3 vec types as a route parameter type. Using the `arrayvec` feature, that looks like this:

```rust
use poem_openapi::{OpenApi, param::Query, payload::Json};
use poem_stackvec::PoemArrayVec;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/echo", method = "get")]
    async fn echo(
        &self,
        #[oai(explode = false)]
        // specifies max of 20 elements
        Query(data): Query<PoemArrayVec<u16, 20>>,
    ) -> poem::Result<Json<Vec<u16>>> {
        let slice = data.as_slice();
        Ok(Json(Vec::from(slice)))
    }
}
```

## Type Differences

The OpenAPI semantics specified by these types differ slightly from the default `Vec<T>` or `[T; SIZE]` in the following way:

### Min/Max Element Counts

both `PoemArrayVec` and `PoemHeaplessVec` specify OpenAPI properties for the minimum (1) & maximum (`SIZE`) for the number of items in an array.

`PoemSmallVec` specifies a minimum of 1 item, unlike `Vec<T>` which has no minimum. It does not specify a maximum.

If you want to make a route parameter optional (allow 0 elements), wrap the type in `Option`:

```rust
Option<PoemArrayVec<T, 8>>`
```

