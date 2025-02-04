use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Colon,
    Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Type, Variant, Visibility,
};

// iter 1

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

// iter 2

struct StructField {
    name: Ident,
    ty: Type,
}

impl StructField {
    fn new(field: &Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
        }
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

#[proc_macro_attribute]
pub fn public_alt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields.iter().map(StructField::new);

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

// iter 3

struct StructFieldParsed {
    name: Ident,
    ty: Ident,
}

impl ToTokens for StructFieldParsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

impl Parse for StructFieldParsed {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(Self {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

#[proc_macro_attribute]
pub fn public_alt_parsed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields
        .iter()
        .map(quote::ToTokens::to_token_stream)
        .map(syn::parse2::<StructFieldParsed>)
        .map(Result::unwrap);

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

// iter 4

#[derive(Debug)]
struct StructFieldParsedCursor {
    name: Option<Ident>,
    ty: Ident,
}

impl ToTokens for StructFieldParsedCursor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = self.name.as_ref();
        let t = &self.ty;
        if let Some(n) = n {
            quote!(pub #n: #t).to_tokens(tokens)
        } else {
            quote!(pub #t).to_tokens(tokens)
        }
    }
}

impl Parse for StructFieldParsedCursor {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let first = input.cursor().ident().unwrap();

        let res = if first.0.to_string().contains("pub") {
            let second = first.1.ident().unwrap();
            let third = second.1.punct().and_then(|p| p.1.ident());
            if let Some(third) = third {
                Ok(Self {
                    name: Some(second.0),
                    ty: third.0,
                })
            } else {
                Ok(Self {
                    name: None,
                    ty: second.0,
                })
            }
        } else {
            let second = first.1.punct().and_then(|p| p.1.ident());
            if let Some(second) = second {
                Ok(Self {
                    name: Some(first.0),
                    ty: second.0,
                })
            } else {
                Ok(Self {
                    name: None,
                    ty: first.0,
                })
            }
        };

        let _: Result<proc_macro2::TokenStream, _> = input.parse();
        //println!("{:#?}", &res);
        res
    }
}

#[proc_macro_attribute]
pub fn public_alt_parsed_cursor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    //println!("{:#?}", &ast);
    let name = ast.ident;

    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => unnamed,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructFieldParsedCursor>(f.to_token_stream()).unwrap());

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

// iter 4 extended

#[allow(dead_code)]
enum TargetType {
    Struct(TargetStruct),
    Enum(TargetEnum),
    Union(TargetUnion),
}

struct TargetStruct {
    name: Ident,
    ty: TargetStructType,
}

enum TargetStructType {
    Named(Vec<TargetStructNamedField>),
    Unnamed(Vec<TargetStructUnnamedField>),
    Unit,
}

struct TargetStructNamedField {
    name: Ident,
    ty: Ident,
}

struct TargetStructUnnamedField {
    ty: Ident,
}

struct TargetEnum {
    name: Ident,
    variants: Vec<Variant>,
}

struct TargetUnion {
    name: Ident,
    fields: Vec<TargetUnionField>,
}

struct TargetUnionField {
    name: Ident,
    ty: Ident,
}

impl ToTokens for TargetType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Struct(s) => s.to_tokens(tokens),
            Self::Enum(e) => e.to_tokens(tokens),
            Self::Union(u) => u.to_tokens(tokens),
        }
    }
}

impl ToTokens for TargetStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        match &self.ty {
            TargetStructType::Named(fields) => {
                let fields = fields.iter().map(|f| {
                    let f_name = &f.name;
                    let f_ty = &f.ty;
                    quote!(pub #f_name: #f_ty)
                });
                quote! {
                    pub #name {
                        #(#fields,)*
                    }
                }
            }
            TargetStructType::Unnamed(fields) => {
                let fields = fields.iter().map(|f| {
                    let f_ty = &f.ty;
                    quote!(pub #f_ty)
                });
                quote! {
                    pub #name(#(#fields,)*);
                }
            }
            TargetStructType::Unit => quote!(pub #name;),
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for TargetEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let variants = &self.variants;
        quote! {
            pub enum #name {
                #(#variants,)*
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for TargetUnion {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let fields = self.fields.iter().map(|f| {
            let f_name = &f.name;
            let f_ty = &f.ty;
            quote!(pub #f_name: #f_ty)
        });
        quote! {
            pub struct #name {
                #(#fields,)*
            }
        }
        .to_tokens(tokens)
    }
}

impl Parse for TargetStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.cursor().ident().unwrap().1.ident().unwrap();
        let ty = match name.1.punct().unwrap() {
            named if named.0.as_char() == '{' => {
                let mut fields = vec![];
                let mut next = named.1.ident();
                while let Some(f_name) = match next {
                    Some(vis) if vis.0.to_string().contains("pub") => vis.1.ident(),
                    i => i,
                } {
                    let f_ty = f_name.1.punct().unwrap().1.ident().unwrap();
                    fields.push(TargetStructNamedField {
                        name: f_name.0,
                        ty: f_ty.0,
                    });
                    next = f_ty.1.punct().unwrap().1.ident();
                }
                TargetStructType::Named(fields)
            }
            unnamed if unnamed.0.as_char() == '(' => {
                let mut fields = vec![];
                let mut next = unnamed.1.ident();
                while let Some(f_ty) = match next {
                    Some(vis) if vis.0.to_string().contains("pub") => vis.1.ident(),
                    i => i,
                } {
                    fields.push(TargetStructUnnamedField { ty: f_ty.0 });
                    next = f_ty.1.punct().unwrap().1.ident();
                }
                TargetStructType::Unnamed(fields)
            }
            unit if unit.0.as_char() == ';' => TargetStructType::Unit,
            _ => panic!(),
        };

        let _: Result<proc_macro2::TokenStream, _> = input.parse();
        Ok(Self { name: name.0, ty })
    }
}

impl Parse for TargetEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _name = input.cursor().ident().unwrap().1.ident().unwrap();
        todo!()
    }
}

// exercises

#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let public_version = quote! {};
    public_version.into()
}

#[proc_macro_attribute]
pub fn make_public(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let attrs = ast.attrs;
    let name = ast.ident;
    let mut public_version = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let fields = named.iter().map(|f| {
                let name = f.ident.as_ref();
                let ty = &f.ty;
                quote!(pub #name: #ty)
            });
            quote! {
                pub struct #name {
                    #(#fields,)*
                }
            }
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let fields = unnamed.iter().map(|f| {
                let ty = &f.ty;
                quote!(pub #ty)
            });
            quote! {
                pub struct #name(#(#fields,)*);
            }
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            quote!(pub struct #name;)
        }
        Data::Enum(DataEnum { ref variants, .. }) => {
            let variants: Vec<_> = variants.iter().collect();
            quote! {
                pub enum #name {
                    #(#variants,)*
                }
            }
        }
        Data::Union(DataUnion {
            fields: FieldsNamed { ref named, .. },
            ..
        }) => {
            let fields = named.iter().map(|f| {
                let name = f.ident.as_ref();
                let ty = &f.ty;
                quote!(pub #name: #ty)
            });
            quote! {
                pub union #name {
                    #(#fields,)*
                }
            }
        }
    };
    public_version = quote! {
        #(#attrs)*
        #public_version
    };
    let attr = attr.to_string();
    public_version = quote! {
        #public_version

        impl #name {
            fn herro(&self) {
                println!("Herro from {}: {}", stringify!(#name), #attr);
            }
        }
    };
    public_version.into()
}

// exercises again - unstarted

struct MakePublicStruct {
    name: Ident,
    ty: Ident,
}

impl ToTokens for MakePublicStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

impl Parse for MakePublicStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(Self {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

#[proc_macro_attribute]
pub fn make_public_alt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields: Vec<_> = fields
        .iter()
        .map(quote::ToTokens::to_token_stream)
        .map(syn::parse2::<MakePublicStruct>)
        .map(Result::unwrap)
        .collect();

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}
