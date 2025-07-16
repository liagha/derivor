pub(crate) mod constants;
#[cfg(feature = "enums")]
pub(crate) mod enums;
#[cfg(any(feature = "enums", feature = "structs", feature = "unions"))]
pub(crate) mod fields;
#[cfg(feature = "structs")]
pub(crate) mod structs;
#[cfg(feature = "unions")]
pub(crate) mod unions;

use constants::CTOR_WORD;
#[cfg(feature = "enums")]
use enums::create_enum_token_stream;
#[cfg(feature = "structs")]
use structs::create_struct_token_stream;
#[cfg(feature = "unions")]
use unions::create_union_token_stream;


use {
    alloc::{
        format,
        string::{
            String, ToString
        },
    },
    quote::ToTokens,
    proc_macro::{
        TokenStream,
    },
    proc_macro2::{
        Delimiter, Ident, Span
    },
    syn::{
        token::Pub,
        spanned::Spanned,
        parse::{
            Parse, ParseStream,
            discouraged::AnyDelimiter,
        },
        parse_macro_input, Data, DeriveInput, Error, Type, Visibility,
    },
    crate::{
        BTreeSet,
    }
};

pub(crate) struct Definition {
    pub(crate) visibility: Visibility,
    pub(crate) ident: Ident,
    pub(crate) attrs: BTreeSet<Attribute>,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Attribute {
    Const,
    DefaultAll,
    Default,
    IntoAll,
}

impl Default for Definition {
    fn default() -> Self {
        Self {
            visibility: Visibility::Public(Pub {
                span: Span::call_site(),
            }),
            ident: Ident::new("new", Span::mixed_site()),
            attrs: Default::default(),
        }
    }
}

pub(crate) fn adjust_keyword_ident(name: String) -> String {
    if syn::parse_str::<Ident>(&name).is_ok() {
        return name;
    }

    format!("r#{}", name)
}

#[cfg(not(any(feature = "enums", feature = "structs", feature = "unions")))]
pub(crate) fn derive_ctor_internal(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    match &derive_input.data {
        Data::Struct(_) => crate::create_struct(derive_input),
        Data::Enum(_) => crate::create_enum(derive_input),
        Data::Union(_) => crate::create_union(derive_input)
    }
}

pub(crate) fn parse_attributes_with_default<T: Parse, F: Fn() -> T>(
    attributes: &[syn::Attribute],
    default: F,
) -> Result<T, Error> {
    for attribute in attributes {
        if attribute.path().is_ident(CTOR_WORD) {
            return attribute.parse_args::<T>();
        }
    }
    Ok(default())
}

pub(crate) fn parse_attributes<T: Parse>(attributes: &[syn::Attribute]) -> Result<Option<T>, Error> {
    for attribute in attributes {
        if attribute.path().is_ident(CTOR_WORD) {
            return attribute.parse_args::<T>().map(Some);
        }
    }
    Ok(None)
}

pub(crate) fn is_phantom(typ: &Type) -> bool {
    for token in typ.to_token_stream() {
        if token.to_string() == "PhantomData" {
            return true;
        }
    }
    false
}

pub(crate) fn consume_delimited<T, F>(
    stream: ParseStream,
    expected: Delimiter,
    expression: F,
) -> Result<T, Error>
where
    F: Fn(ParseStream) -> Result<T, Error>,
{
    let (delimiter, span, buffer) = stream.parse_any_delimiter()?;
    if delimiter != expected {
        return Err(Error::new(span.span(),
                              format!("Expected enclosing {:?}", expected),
        ));
    }
    expression(&buffer)
}
