use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn auto_route(args: TokenStream, input: TokenStream) -> TokenStream {
    let mount = if args.is_empty() {
        LitStr::new("/", proc_macro2::Span::call_site())
    } else {
        parse_macro_input!(args as LitStr)
    };
    let func = parse_macro_input!(input as ItemFn);
    let fn_name = &func.sig.ident;

    TokenStream::from(quote! {
        #func
        rocket_z::register_routes!(#mount, rocket::routes![#fn_name]);
    })
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("get", args, input)
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("post", args, input)
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("put", args, input)
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("delete", args, input)
}

#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("patch", args, input)
}

fn route_with_method(method: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let mount = if args.is_empty() {
        LitStr::new("/", proc_macro2::Span::call_site())
    } else {
        parse_macro_input!(args as LitStr)
    };
    let func = parse_macro_input!(input as ItemFn);
    let fn_name = &func.sig.ident;
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());

    TokenStream::from(quote! {
        #[rocket::#method_ident(#mount)]
        #func
        rocket_z::register_routes!(#mount, rocket::routes![#fn_name]);
    })
}