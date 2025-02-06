use fields::{
    builder_field_definitions, builder_init_values, builder_methods, get_named_fields,
    original_struct_setters,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

mod fields;

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

    let fields = get_named_fields(&ast.data);

    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_init_values(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields);

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#[must_use] #builder_methods)*

            #[must_use]
            pub fn build(self) -> #name {
                #name {
                    #(#set_fields,)*
                }
            }
        }

        impl #name {
            #[must_use]
            pub fn builder() -> #builder {
                #builder {
                    #(#builder_inits,)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_struct_name_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));
    }

    #[test]
    fn builder_struct_with_no_methods_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };
        let expected = quote! {
            struct StructWithNoFieldsBuilder {}

            impl StructWithNoFieldsBuilder {
                #[must_use]
                pub fn build(self) -> StructWithNoFields {
                    StructWithNoFields {}
                }
            }

            impl StructWithNoFields {
                #[must_use]
                pub fn builder() -> StructWithNoFieldsBuilder {
                    StructWithNoFieldsBuilder {}
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
            struct StructWithFields {
                field_1: i32,
            }
        };
        let expected = quote! {
            struct StructWithFieldsBuilder {
                field_1: Option<i32>,
            }

            impl StructWithFieldsBuilder {
                #[must_use]
                pub fn field_1(mut self, input: i32) -> Self {
                    self.field_1 = Some(input);
                    self
                }

                #[must_use]
                pub fn build(self) -> StructWithFields {
                    StructWithFields {
                        field_1: self.field_1
                            .expect(concat!("field not set: ", "field_1")),
                    }
                }
            }

            impl StructWithFields {
                #[must_use]
                pub fn builder() -> StructWithFieldsBuilder {
                    StructWithFieldsBuilder {
                        field_1: None,
                    }
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    #[ignore = "too verbose to keep up to speed"]
    fn assert_with_parsing() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        let derived: DeriveInput = syn::parse2(actual).unwrap();
        let name = derived.ident;

        assert_eq!(name, "StructWithNoFieldsBuilder");
    }
}
