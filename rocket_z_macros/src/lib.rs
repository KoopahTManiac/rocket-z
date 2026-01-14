use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};
use syn::{punctuated::Punctuated, Expr, ExprLit, Token};
use syn::parse::Parser;

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
        rocket_z::register_routes!("/", rocket::routes![#fn_name]);
    })
}

/// Rocket GET route with auto-registration.
///
/// Usage:
/// - `#[get("/path")]`
/// - `#[get("/path", rank = 2)]`
/// - `#[get("/path", format = "json")]`
/// - `#[get("/path", data = "<body>")]`
/// - `#[get("/path", format = "json", data = "<body>")]`
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("get", args, input)
}

/// Rocket POST route with auto-registration.
///
/// Usage:
/// - `#[post("/path")]`
/// - `#[post("/path", rank = 2)]`
/// - `#[post("/path", format = "json", data = "<body>")]`
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("post", args, input)
}

/// Rocket PUT route with auto-registration.
///
/// Usage:
/// - `#[put("/path")]`
/// - `#[put("/path", rank = 2)]`
/// - `#[put("/path", format = "json", data = "<body>")]`
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("put", args, input)
}

/// Rocket DELETE route with auto-registration.
///
/// Usage:
/// - `#[delete("/path")]`
/// - `#[delete("/path", rank = 2)]`
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("delete", args, input)
}

/// Rocket PATCH route with auto-registration.
///
/// Usage:
/// - `#[patch("/path")]`
/// - `#[patch("/path", rank = 2)]`
/// - `#[patch("/path", format = "json", data = "<body>")]`
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route_with_method("patch", args, input)
}

fn route_with_method(method: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let args_ts = proc_macro2::TokenStream::from(args.clone());
    let mount = match parse_mount_from_args(args) {
        Ok(mount) => mount,
        Err(err) => return err.to_compile_error().into(),
    };
    let func = parse_macro_input!(input as ItemFn);
    let fn_name = &func.sig.ident;
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());

    let route_attr = if args_ts.is_empty() {
        quote! { #[rocket::#method_ident] }
    } else {
        quote! { #[rocket::#method_ident(#args_ts)] }
    };

    TokenStream::from(quote! {
        #route_attr
        #func
        rocket_z::register_routes!("/", rocket::routes![#fn_name]);
    })
}

fn parse_mount_from_args(args: TokenStream) -> Result<LitStr, syn::Error> {
    if args.is_empty() {
        return Ok(LitStr::new("/", proc_macro2::Span::call_site()));
    }

    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let parsed = parser.parse(args)?;
    let Some(first) = parsed.first() else {
        return Ok(LitStr::new("/", proc_macro2::Span::call_site()));
    };

    match first {
        Expr::Lit(ExprLit { lit: syn::Lit::Str(lit), .. }) => Ok(lit.clone()),
        _ => Err(syn::Error::new_spanned(
            first,
            "expected first argument to be a string literal route, e.g. \"/path\"",
        )),
    }
}