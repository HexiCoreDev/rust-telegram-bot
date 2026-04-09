use crate::{
    command_attr::CommandAttrs, error::compile_error_at, fields_parse::ParserType,
    rename_rules::RenameRule, Result,
};

/// Enum-level configuration extracted from `#[command(...)]` on the enum itself.
pub(crate) struct CommandEnum {
    /// Command prefix (default `"/"`).
    pub prefix: String,
    /// Optional global description header for the help text.
    pub description: Option<String>,
    /// Separator between the command and its arguments (default `" "`).
    pub command_separator: String,
    /// How variant names are converted to command strings.
    pub rename_rule: RenameRule,
    /// Default parser for command arguments.
    pub parser_type: ParserType,
}

impl CommandEnum {
    /// Extract enum-level settings from the attribute list.
    pub fn from_attributes(attributes: &[syn::Attribute]) -> Result<Self> {
        let attrs = CommandAttrs::from_attributes(attributes)?;
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            rename,
            parser,
            separator,
            command_separator,
            hide,
        } = attrs;

        // These attributes are variant-only; error if used on the enum itself.
        if let Some((_, sp)) = rename {
            return Err(compile_error_at(
                "`rename` can only be applied to enum variants, not the enum itself",
                sp,
            ));
        }
        if let Some(sp) = hide {
            return Err(compile_error_at(
                "`hide` can only be applied to enum variants, not the enum itself",
                sp,
            ));
        }

        let mut parser = parser.map(|(p, _)| p).unwrap_or(ParserType::Default);

        // If the global parser is `split` and a separator was provided, attach it.
        if let (
            ParserType::Split {
                separator: ref mut sep,
            },
            Some((s, _)),
        ) = (&mut parser, &separator)
        {
            *sep = Some(s.clone());
        }

        Ok(Self {
            prefix: prefix.map(|(p, _)| p).unwrap_or_else(|| "/".to_owned()),
            description: description.map(|(d, _)| d),
            command_separator: command_separator
                .map(|(s, _)| s)
                .unwrap_or_else(|| " ".to_owned()),
            rename_rule: rename_rule
                .map(|(rr, _)| rr)
                .unwrap_or(RenameRule::Identity),
            parser_type: parser,
        })
    }
}
