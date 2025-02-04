use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

#[proc_macro]
pub fn local(_: TokenStream) -> TokenStream {
    let greeting = Ident::new("greeting", Span::call_site()); // mixed_site hides local variables
                                                              // from call-site
    quote! {
        let #greeting = "Hello!";
    }
    .into()
}

#[proc_macro]
pub fn make_private(item: TokenStream) -> TokenStream {
    //let item_as_stream: proc_macro2::TokenStream = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!(),
    };

    let struct_fields = fields.iter().map(|f| {
        let f_name = f.ident.as_ref();
        let f_ty = &f.ty;
        quote! {#f_name: #f_ty}
    });

    let paren_fields = struct_fields.clone();

    let struct_field_names = fields.iter().map(|f| f.ident.as_ref());

    let struct_field_getters = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let type_name = &f.ty;
        let method_name = format_ident!("get_{}", field_name);
        //let method_name = Ident::new(format!("get_{}", field_name).as_str(), field_name.span());
        quote! {
            #[must_use]
            pub const fn #method_name(&self) -> &#type_name {
                &self.#field_name
            }
        }
    });

    quote! {
        struct #name {
            #(#struct_fields,)*
        }

        impl #name {
            #[must_use]
            pub const fn new(#(#paren_fields,)*) -> Self {
                Self { #(#struct_field_names,)* }
            }
            #(#struct_field_getters)*
        }
    }
    .into()
}

//        match f_ty {
//            Type::Array(_) => println!("Array"),
//            Type::BareFn(_) => println!("BareFn"),
//            Type::Group(_) => println!("Group"),
//            Type::ImplTrait(_) => println!("ImplTrait"),
//            Type::Infer(_) => println!("Infer"),
//            Type::Macro(_) => println!("Macro"),
//            Type::Never(_) => println!("Never"),
//            Type::Paren(_) => println!("Paren"),
//            Type::Path(p) => println!("Path: {}", p.to_token_stream().to_string()), // TODO: Convert String to str
//            Type::Ptr(_) => println!("Ptr"),
//            Type::Reference(_) => println!("Reference"),
//            Type::Slice(_) => println!("Slice"),
//            Type::TraitObject(_) => println!("TraitObject"),
//            Type::Tuple(_) => println!("Tuple"),
//            Type::Verbatim(_) => println!("Verbatim"),
//            _ => println!("Whelp..."),
//        };
