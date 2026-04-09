use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, Type};

use crate::Result;

/// How arguments for a command variant are parsed from the message text.
#[derive(Clone)]
pub(crate) enum ParserType {
    /// Use `FromStr` for a single-field variant.
    Default,
    /// Split the argument string by a separator, then parse each piece.
    Split { separator: Option<String> },
}

impl ParserType {
    /// Parse from the string value of a `parse_with` attribute.
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "default" => Ok(Self::Default),
            "split" => Ok(Self::Split { separator: None }),
            other => Err(crate::compile_error(format!(
                "unknown parse_with value `{other}` (expected `\"default\"` or `\"split\"`)"
            ))),
        }
    }
}

/// Generate the token stream that parses a variant's fields from the `args`
/// string variable and constructs `Self::Variant(...)`.
pub(crate) fn impl_parse_args(
    fields: &Fields,
    self_variant: proc_macro2::TokenStream,
    parser: &ParserType,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => self_variant,
        Fields::Unnamed(fields) => impl_parse_unnamed(fields, self_variant, parser),
        Fields::Named(fields) => impl_parse_named(fields, self_variant, parser),
    }
}

fn impl_parse_unnamed(
    data: &FieldsUnnamed,
    variant: proc_macro2::TokenStream,
    parser: &ParserType,
) -> proc_macro2::TokenStream {
    let get_arguments = create_parser(parser, data.unnamed.iter().map(|f| &f.ty));
    let indices = (0..data.unnamed.len()).map(syn::Index::from);
    let mut init = quote! {};
    for i in indices {
        init.extend(quote! { arguments.#i, });
    }
    quote! {
        {
            #get_arguments
            #variant(#init)
        }
    }
}

fn impl_parse_named(
    data: &FieldsNamed,
    variant: proc_macro2::TokenStream,
    parser: &ParserType,
) -> proc_macro2::TokenStream {
    let get_arguments = create_parser(parser, data.named.iter().map(|f| &f.ty));
    let i = (0..).map(syn::Index::from);
    let names = data.named.iter().map(|f| f.ident.as_ref().unwrap());
    quote! {
        {
            #get_arguments
            #variant { #(#names: arguments.#i),* }
        }
    }
}

fn create_parser<'a>(
    parser: &ParserType,
    mut types: impl ExactSizeIterator<Item = &'a Type>,
) -> proc_macro2::TokenStream {
    let parser_fn = match parser {
        ParserType::Default => match types.len() {
            0 => return quote! { let arguments = (); },
            1 => {
                let ty = types.next().unwrap();
                quote! {
                    (|s: ::std::string::String| -> ::std::result::Result<(#ty,), ::std::string::String> {
                        let res = <#ty as ::std::str::FromStr>::from_str(&s)
                            .map_err(|e| ::std::format!("failed to parse argument: {}", e))?;
                        ::std::result::Result::Ok((res,))
                    })
                }
            }
            _ => {
                return quote! {
                    ::std::compile_error!(
                        "default parser supports at most 1 field; use `parse_with = \"split\"` for multiple fields"
                    );
                };
            }
        },
        ParserType::Split { separator } => {
            let sep = separator.as_deref().unwrap_or(" ");
            parser_with_separator(sep, types)
        }
    };

    quote! {
        let arguments = #parser_fn(args)?;
    }
}

fn parser_with_separator<'a>(
    separator: &str,
    types: impl ExactSizeIterator<Item = &'a Type>,
) -> proc_macro2::TokenStream {
    let expected = types.len();
    let found_indices = 0usize..;

    let parse_fields = quote! {
        (
            #(
                {
                    let s = splitted.next().ok_or_else(|| {
                        ::std::format!(
                            "too few arguments: expected {}, found {}",
                            #expected,
                            #found_indices
                        )
                    })?;
                    <#types as ::std::str::FromStr>::from_str(s)
                        .map_err(|e| ::std::format!("failed to parse argument: {}", e))?
                },
            )*
        )
    };

    quote! {
        (|s: ::std::string::String| -> ::std::result::Result<_, ::std::string::String> {
            let mut splitted = s.split(#separator);
            let res = #parse_fields;
            match splitted.next() {
                ::std::option::Option::Some(d) if !s.is_empty() => {
                    ::std::result::Result::Err(::std::format!(
                        "too many arguments: expected {}, found at least {} (excess: \"{}\")",
                        #expected,
                        #expected + 1 + splitted.count(),
                        d,
                    ))
                }
                _ => ::std::result::Result::Ok(res),
            }
        })
    }
}
