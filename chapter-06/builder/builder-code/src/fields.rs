use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataStruct, Field, Fields, FieldsNamed, Ident, Type,
};

pub(super) fn get_named_fields(data: &Data) -> &Punctuated<Field, Comma> {
    match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only implemented for named structs"),
    }
}

pub(super) fn builder_field_definitions(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, ty) = get_name_and_type(f);
        quote! { #name: Option<#ty> }
    })
}

pub(super) fn builder_init_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, _) = get_name_and_type(f);
        quote! { #name: None }
    })
}

pub(super) fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, ty) = get_name_and_type(f);
        quote! {
            pub fn #name(mut self, input: #ty) -> Self {
                self.#name = Some(input);
                self
            }
        }
    })
}

pub(super) fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, _) = get_name_and_type(f);
        let name_as_string = name.as_ref().unwrap().to_string();
        quote! {
            #name: self.#name
                .expect(concat!("field not set: ", #name_as_string))
        }
    })
}

#[allow(dead_code)]
pub(super) fn original_struct_setters_for_borrowed_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, ty) = get_name_and_type(f);
        let name_as_string = name.as_ref().unwrap().to_string();
        let error = quote!(expect(&format!("field {} not set", #name_as_string)));

        let handle_type = if matches_type(ty, "String") {
            quote! {
                as_ref()
                .#error
                .to_string()
            }
        } else {
            quote!(#error)
        };

        quote! {
            #name: self.#name.#handle_type
        }
    })
}

fn get_name_and_type<'a>(f: &'a Field) -> (Option<&'a Ident>, &'a Type) {
    let field_name = f.ident.as_ref();
    let field_type = &f.ty;
    (field_name, field_type)
}

fn matches_type(ty: &Type, type_name: &str) -> bool {
    if let Type::Path(ref p) = ty {
        let first_match = p.path.segments[0].ident.to_string();
        first_match == type_name
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{FieldMutability, Path, PathSegment, TypePath, Visibility};

    use super::*;

    #[test]
    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("String", Span::call_site()),
            arguments: Default::default(),
        };
        let mut pun = Punctuated::new();
        pun.push(p);
        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
        });
        let f = Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new("example", Span::call_site())),
            colon_token: None,
            ty,
        };
        println!("{:#?}", f);

        let (actual_name, _) = get_name_and_type(&f);

        assert_eq!(actual_name.unwrap(), "example");
    }
}
