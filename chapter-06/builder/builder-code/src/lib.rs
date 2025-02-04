use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

    quote! {
        struct #builder {}

        impl #builder {
            pub fn build(&self) -> #name {
                #name {}
            }
        }

        impl #name {
            pub fn builder() -> #builder {
                #builder {}
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
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };
        let expected = quote! {
            struct StructWithNoFieldsBuilder {}

            impl StructWithNoFieldsBuilder {
                pub fn build(&self) -> StructWithNoFields {
                    StructWithNoFields {}
                }
            }

            impl StructWithNoFields {
                pub fn builder() -> StructWithNoFieldsBuilder {
                    StructWithNoFieldsBuilder {}
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn assert_with_parsing() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        let derived: DeriveInput = syn::parse2(actual).unwrap();
        let name = derived.ident;

        assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
    }
}
