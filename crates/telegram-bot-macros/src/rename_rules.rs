use crate::error::{compile_error, Result};

use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase,
    ToSnakeCase,
};

/// Rules for converting Rust enum variant names into Telegram command strings.
#[derive(Copy, Clone, Debug)]
pub(crate) enum RenameRule {
    /// `Help` -> `help`
    LowerCase,
    /// `Help` -> `HELP`
    UpperCase,
    /// `get_help` -> `GetHelp`
    PascalCase,
    /// `GetHelp` -> `getHelp`
    CamelCase,
    /// `GetHelp` -> `get_help`
    SnakeCase,
    /// `GetHelp` -> `GET_HELP`
    ScreamingSnakeCase,
    /// `GetHelp` -> `get-help`
    KebabCase,
    /// `GetHelp` -> `GET-HELP`
    ScreamingKebabCase,
    /// Leave the variant name as-is.
    Identity,
}

impl RenameRule {
    /// Apply the rename rule to a PascalCase variant name.
    pub fn apply(self, input: &str) -> String {
        match self {
            Self::LowerCase => input.to_lowercase(),
            Self::UpperCase => input.to_uppercase(),
            Self::PascalCase => input.to_pascal_case(),
            Self::CamelCase => input.to_lower_camel_case(),
            Self::SnakeCase => input.to_snake_case(),
            Self::ScreamingSnakeCase => input.to_shouty_snake_case(),
            Self::KebabCase => input.to_kebab_case(),
            Self::ScreamingKebabCase => input.to_shouty_kebab_case(),
            Self::Identity => input.to_owned(),
        }
    }

    /// Parse a rename rule from the string representation used in attributes.
    pub fn parse(rule: &str) -> Result<Self> {
        match rule {
            "lowercase" => Ok(Self::LowerCase),
            "UPPERCASE" => Ok(Self::UpperCase),
            "PascalCase" => Ok(Self::PascalCase),
            "camelCase" => Ok(Self::CamelCase),
            "snake_case" => Ok(Self::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Ok(Self::ScreamingSnakeCase),
            "kebab-case" => Ok(Self::KebabCase),
            "SCREAMING-KEBAB-CASE" => Ok(Self::ScreamingKebabCase),
            "identity" => Ok(Self::Identity),
            invalid => Err(compile_error(format!(
                "invalid rename rule `{invalid}` (supported: `lowercase`, `UPPERCASE`, \
                 `PascalCase`, `camelCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, \
                 `kebab-case`, `SCREAMING-KEBAB-CASE`, `identity`)"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_rule() {
        assert_eq!(RenameRule::LowerCase.apply("HelloWorld"), "helloworld");
        assert_eq!(RenameRule::LowerCase.apply("Help"), "help");
    }

    #[test]
    fn uppercase_rule() {
        assert_eq!(RenameRule::UpperCase.apply("Help"), "HELP");
    }

    #[test]
    fn snake_case_rule() {
        assert_eq!(RenameRule::SnakeCase.apply("GetHelp"), "get_help");
    }

    #[test]
    fn identity_rule() {
        assert_eq!(RenameRule::Identity.apply("Help"), "Help");
    }

    #[test]
    fn parse_invalid_rule() {
        assert!(RenameRule::parse("nope").is_err());
    }

    #[test]
    fn parse_valid_rules() {
        assert!(RenameRule::parse("lowercase").is_ok());
        assert!(RenameRule::parse("UPPERCASE").is_ok());
        assert!(RenameRule::parse("PascalCase").is_ok());
        assert!(RenameRule::parse("camelCase").is_ok());
        assert!(RenameRule::parse("snake_case").is_ok());
        assert!(RenameRule::parse("SCREAMING_SNAKE_CASE").is_ok());
        assert!(RenameRule::parse("kebab-case").is_ok());
        assert!(RenameRule::parse("SCREAMING-KEBAB-CASE").is_ok());
        assert!(RenameRule::parse("identity").is_ok());
    }
}
