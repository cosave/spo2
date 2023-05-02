use syn::{Field, ItemStruct, __private::ToTokens};

const BASE_TYPES: [&str; 11] = [
    "bool", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64",
];

/// A helper struct used to generate extra code for Rust structs that are
/// intended for use in FFI code.
/// Ideally such structs will have `#[derive(FfiStruct)]` attribute.
///
/// This is used in [spo2_derive] crate to create additional code for common
/// FFI functionality like dropping the struct.
/// This is used in [spo2] for creating foreign language code for
/// the decorated struct.
#[cfg_attr(feature = "develop", derive(Debug))]
pub struct FfiStruct {
    struct_def: ItemStruct,
}

impl FfiStruct {
    /// Creates a new instance of the `FfiStruct` from a struct definition.
    pub fn new(struct_def: ItemStruct) -> Self {
        Self { struct_def }
    }

    /// Get a list of fields in this struct that can safely pass through FFI
    /// without needing special conversion and memory/pointer management.
    /// At this moment only base types are considered safe.
    fn ffi_safe_fields(&self) -> Vec<&Field> {
        self.struct_def
            .fields
            .iter()
            .filter_map(|field| {
                if BASE_TYPES.contains(&field.ty.to_token_stream().to_string().as_str()) {
                    Some(field)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if this struct can be safely copied over FFI.
    /// A struct can be safely copied over FFI if it contains only safe fields.
    pub fn is_copy_safe(&self) -> bool {
        self.ffi_safe_fields().len() == self.struct_def.fields.len()
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse2, parse_quote, ItemStruct};

    use crate::FfiStruct;

    #[test]
    fn test_is_copy_safe() {
        let test_cases = [
            (parse_quote! { struct MyStruct {} }, true, "Empty struct."),
            (
                parse_quote! { struct MyStruct { field1: i32, field2: f64, field3: bool } },
                true,
                "Struct with only base types.",
            ),
            (
                parse_quote! { struct MyStruct { field1: *mut c_char, field2: f64, field3: bool } },
                // *const c_char is copy type as it is just pointer's address.
                // TODO: test for pass after we parse raw pointers.
                false,
                "Struct with string pointer.",
            ),
            (
                parse_quote! { struct MyStruct { field1: [u8; 16], field2: f64, field3: bool } },
                // An array is safe as long as the contents are safe.
                // TODO: test for pass after we parse array.
                false,
                "Struct with array or bytes.",
            ),
            (
                parse_quote! { struct MyStruct { field1: Uuid, field2: f64, field3: bool } },
                // A 3rd party crate type such as Uuid is safe as long as it has
                // repr(C) or repr(transparent) attribute and implement Copy trait.
                // TODO: test for pass after we parse 3rd party crates.
                false,
                "Struct with Uuid.",
            ),
            (
                parse_quote! { struct MyStruct { field1: i32, field2: f64, field3: Vec<u8> } },
                false,
                "Struct with a vec.",
            ),
            (
                parse_quote! { struct MyStruct { field1: MyCustomType, field2: MyOtherCustomType } },
                false,
                "Struct with a custom type.",
            ),
            (
                parse_quote! { struct MyStruct { field1: i32, field2: MyNestedStruct } },
                false,
                "Struct with a nested struct.",
            ),
        ];

        for (input, expected, msg) in test_cases.into_iter() {
            let input_struct =
                parse2::<ItemStruct>(input).expect(&format!("Unable to parse: {}", msg));
            let ffi_struct = FfiStruct::new(input_struct);
            assert_eq!(
                ffi_struct.is_copy_safe(),
                expected,
                "Test case {} failed: expected={}, got={}",
                msg,
                expected,
                ffi_struct.is_copy_safe()
            );
        }
    }
}
