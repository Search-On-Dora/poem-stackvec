use poem_openapi::{registry::MetaSchemaRef, types::Type};

pub(crate) fn fixed_capacity_schema_ref<T: Type, const SIZE: usize>() -> MetaSchemaRef {
    let arr_schema = <[T; SIZE]>::schema_ref();
    let mut schema = arr_schema.unwrap_inline().clone();
    schema.min_length = Some(1); 
    schema.min_items = Some(1);
    schema.max_items = Some(SIZE);
    schema.title = Some(format!("ArrayVec<{}>", T::name()));

    let desc = format!("1 to {SIZE} items of type {}", T::name());
    // this should only get called to build the openapi schema, and not be a repeated cost
    schema.description = Some(Box::leak(desc.into_boxed_str()));

    MetaSchemaRef::Inline(Box::new(schema))
}
