use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, Expr, ExprLit, Lit, Meta};

use crate::{fields_parse::ParserType, rename_rules::RenameRule, Result};

/// Parsed `#[command(...)]` attributes that may appear on either the enum or a
/// variant.
pub(crate) struct CommandAttrs {
    pub prefix: Option<(String, Span)>,
    pub description: Option<(String, Span)>,
    pub rename_rule: Option<(RenameRule, Span)>,
    pub rename: Option<(String, Span)>,
    pub parser: Option<(ParserType, Span)>,
    pub separator: Option<(String, Span)>,
    pub command_separator: Option<(String, Span)>,
    pub hide: Option<Span>,
}

/// Tracks whether the description was set from a doc comment or an explicit
/// `#[command(description = "...")]` attribute, so explicit always wins.
#[derive(Clone, Copy, PartialEq, Eq)]
enum DescriptionSource {
    DocComment,
    Explicit,
}

impl CommandAttrs {
    /// Parse all `#[command(...)]` and `#[doc = "..."]` attributes from a list.
    pub fn from_attributes(attributes: &[Attribute]) -> Result<Self> {
        let mut this = Self {
            prefix: None,
            description: None,
            rename_rule: None,
            rename: None,
            parser: None,
            separator: None,
            command_separator: None,
            hide: None,
        };

        let mut desc_source: Option<DescriptionSource> = None;

        for attr in attributes {
            if attr.path().is_ident("doc") {
                // Extract doc comment text: `#[doc = "some text"]`
                // Only use it if no explicit description has been set yet.
                if desc_source != Some(DescriptionSource::Explicit) {
                    if let Meta::NameValue(nv) = &attr.meta {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(s), ..
                        }) = &nv.value
                        {
                            let text = s.value();
                            let text = text.strip_prefix(' ').unwrap_or(&text);
                            match &mut this.description {
                                Some((existing, _)) => {
                                    existing.push('\n');
                                    existing.push_str(text);
                                }
                                slot @ None => {
                                    *slot = Some((text.to_owned(), s.span()));
                                }
                            }
                            desc_source = Some(DescriptionSource::DocComment);
                        }
                    }
                }
                continue;
            }

            if !attr.path().is_ident("command") {
                continue;
            }

            // `parse_nested_meta` expects a closure returning `syn::Result`.
            attr.parse_nested_meta(|meta| {
                let span = meta.path.span();

                if meta.path.is_ident("prefix") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        insert_syn(&mut this.prefix, s.value(), span)?;
                    }
                } else if meta.path.is_ident("description") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        // An explicit `description = "..."` always overrides
                        // a previous doc-comment description. Two explicit
                        // description attributes are still an error.
                        if desc_source == Some(DescriptionSource::Explicit) {
                            return Err(syn::Error::new(span, "duplicate `description` attribute"));
                        }
                        this.description = Some((s.value(), span));
                        desc_source = Some(DescriptionSource::Explicit);
                    }
                } else if meta.path.is_ident("rename_rule") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        let rule = RenameRule::parse(&s.value()).map_err(|_| {
                            syn::Error::new(span, format!("invalid rename rule `{}`", s.value()))
                        })?;
                        insert_syn(&mut this.rename_rule, rule, span)?;
                    }
                } else if meta.path.is_ident("rename") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        insert_syn(&mut this.rename, s.value(), span)?;
                    }
                } else if meta.path.is_ident("parse_with") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        let parser = ParserType::parse(&s.value()).map_err(|_| {
                            syn::Error::new(
                                span,
                                format!("invalid parse_with value `{}`", s.value()),
                            )
                        })?;
                        insert_syn(&mut this.parser, parser, span)?;
                    }
                } else if meta.path.is_ident("separator") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        insert_syn(&mut this.separator, s.value(), span)?;
                    }
                } else if meta.path.is_ident("command_separator") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        insert_syn(&mut this.command_separator, s.value(), span)?;
                    }
                } else if meta.path.is_ident("hide") {
                    if this.hide.is_some() {
                        return Err(syn::Error::new(span, "duplicate `hide` attribute"));
                    }
                    this.hide = Some(span);
                } else {
                    return Err(syn::Error::new(
                        span,
                        "unknown command attribute (expected one of: `prefix`, `description`, \
                         `rename_rule`, `rename`, `parse_with`, `separator`, \
                         `command_separator`, `hide`)",
                    ));
                }

                Ok(())
            })?;
        }

        Ok(this)
    }
}

/// Insert a value into an `Option<(T, Span)>`, returning a `syn::Error` on
/// duplicates. This variant is used inside `parse_nested_meta` closures which
/// require `syn::Result`.
fn insert_syn<T>(
    slot: &mut Option<(T, Span)>,
    value: T,
    span: Span,
) -> std::result::Result<(), syn::Error> {
    if slot.is_some() {
        return Err(syn::Error::new(span, "duplicate attribute"));
    }
    *slot = Some((value, span));
    Ok(())
}
