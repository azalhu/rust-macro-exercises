use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use venial::{parse_item, Enum, Item, Struct, Union};

#[proc_macro_derive(HelloVanilla)]
pub fn hello_vanilla(item: TokenStream) -> TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("no ident"),
        }
    }
    let name = ident_name(item.into_iter().nth(1).unwrap());

    format!(
        "impl {} {{ fn hello_world_vanilla(&self) \
    {{ println!(\"Hello, World, Vanilla! by {}\") }} }} ",
        name, name
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(HelloSyn)]
pub fn hello_syn(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let name_string = name.to_string();

    let add_hello_world = quote! {
        impl #name {
            #[doc = concat!(" Prints \"Hello, World!\" on ", #name_string, ".")]
            fn hello_world_syn(&self) {
                println!("Hello, World, Syn! by {}", stringify!(#name));
            }
        }
    };
    add_hello_world.into()
}

#[proc_macro_derive(HelloVenial)]
pub fn hello_venial(item: TokenStream) -> TokenStream {
    let item = parse_item(item.into()).unwrap();

    let name = match item {
        Item::Struct(Struct { name, .. }) => name,
        Item::Enum(Enum { name, .. }) => name,
        Item::Union(Union { name, .. }) => name,
        _ => panic!("only implemented for struct, enum, and union"),
    };
    let name_string = name.to_string();

    let add_hello_world = quote! {
        impl #name {
            #[doc = concat!(" Prints \"Hello, World!\" on ", #name_string, ".")]
            fn hello_world_venial(&self) {
                println!("Hello, World, Venial! by {}", stringify!(#name));
            }
        }
    };
    add_hello_world.into()
}
