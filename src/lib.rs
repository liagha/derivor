#![no_std]
#![allow(dead_code)]

extern crate alloc;
mod constructor;
mod extras;

use {
    alloc::{
        collections::BTreeSet,
        string::ToString
    },
    syn::{
        DeriveInput, Error, Type,
    },
    proc_macro::TokenStream,
    proc_macro2::Span,

    crate::{
        constructor::{
            adjust_keyword_ident,
            is_phantom,
        },
    }
};

#[cfg(not(feature = "enums"))]
pub(crate) fn create_enum(_derive_input: DeriveInput) -> TokenStream {
    TokenStream::from(Error::new(Span::call_site(),
                                 "\"enums\" feature must be enabled to use #[derive(ctor)] on enums.").to_compile_error())
}

#[cfg(not(feature = "structs"))]
pub(crate) fn create_struct(_derive_input: DeriveInput) -> TokenStream {
    TokenStream::from(Error::new(Span::call_site(),
                                 "\"structs\" feature must be enabled to use #[derive(ctor)] on structs.").to_compile_error())
}

#[cfg(not(feature = "unions"))]
pub(crate) fn create_union(_derive_input: DeriveInput) -> TokenStream {
    TokenStream::from(Error::new(Span::call_site(),
                                 "\"unions\" feature must be enabled to use #[derive(ctor)] on unions.").to_compile_error())
}

#[cfg(feature = "shorthand")]
#[proc_macro_derive(ctor, attributes(ctor, cloned, default, expr, into, iter))]
pub fn derive_ctor(input: TokenStream) -> TokenStream {
    constructor::derive_ctor_internal(input)
}

#[cfg(all(not(feature = "shorthand"), not(any(feature = "enums", feature = "structs", feature = "unions"))))]
#[proc_macro_derive(ctor, attributes(ctor))]
pub fn derive_ctor(input: TokenStream) -> TokenStream {
    constructor::derive_ctor_internal(input)
}

#[test]
fn test_is_phantom_data() {
    assert!(is_phantom(&syn::parse_str::<Type>("PhantomData").unwrap()));
    assert!(is_phantom(&syn::parse_str::<Type>("&mut PhantomData<&'static str>").unwrap()));
    assert!(!is_phantom(&syn::parse_str::<Type>("i32").unwrap()));
}

#[test]
fn test_adjust_keyword_ident() {
    assert_eq!("abc".to_string(), adjust_keyword_ident("abc".to_string()));
    assert_eq!("r#break".to_string(), adjust_keyword_ident("break".to_string()));
    assert_eq!("r#fn".to_string(), adjust_keyword_ident("fn".to_string()));
    assert_eq!("r#const".to_string(), adjust_keyword_ident("const".to_string()));
}

