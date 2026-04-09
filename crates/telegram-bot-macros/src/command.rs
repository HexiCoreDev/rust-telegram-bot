use crate::{
    command_attr::CommandAttrs, command_enum::CommandEnum, error::compile_error_at,
    fields_parse::ParserType, Result,
};

/// A single parsed command variant with all attributes resolved.
pub(crate) struct Command {
    /// Prefix of this command (e.g. `"/"`).
    pub prefix: String,
    /// Description for help text. `None` means no description was provided.
    pub description: Option<String>,
    /// Resolved command name with rename rules applied.
    pub name: String,
    /// Parser type for this command's arguments.
    pub parser: ParserType,
    /// Whether this command is hidden from help text and `bot_commands()`.
    pub hidden: bool,
}

impl Command {
    /// Build a `Command` by merging variant-level attributes with enum-level defaults.
    pub fn new(
        variant_name: &str,
        attributes: &[syn::Attribute],
        global: &CommandEnum,
    ) -> Result<Self> {
        let attrs = CommandAttrs::from_attributes(attributes)?;
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            rename,
            parser,
            separator: _,
            command_separator: _,
            hide,
        } = attrs;

        // Determine the command name: explicit `rename` wins, then variant rename_rule,
        // then the enum-level rename_rule.
        let name = match (rename, rename_rule) {
            (Some((r, _)), None) => r,
            (Some(_), Some((_, sp))) => {
                return Err(compile_error_at(
                    "`rename_rule` cannot be used together with `rename` on a variant",
                    sp,
                ));
            }
            (None, Some((rule, _))) => rule.apply(variant_name),
            (None, None) => global.rename_rule.apply(variant_name),
        };

        let prefix = prefix
            .map(|(p, _)| p)
            .unwrap_or_else(|| global.prefix.clone());
        let parser = parser
            .map(|(p, _)| p)
            .unwrap_or_else(|| global.parser_type.clone());
        let hidden = hide.is_some();

        Ok(Self {
            prefix,
            description: description.map(|(d, _)| d),
            parser,
            name,
            hidden,
        })
    }

    /// The full command string including prefix (e.g. `"/help"`).
    pub fn prefixed_command(&self) -> String {
        format!("{}{}", self.prefix, self.name)
    }

    /// Whether this command should appear in help text and `bot_commands()`.
    pub fn is_visible(&self) -> bool {
        !self.hidden
    }
}
