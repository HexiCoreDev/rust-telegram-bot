use crate::{
    command::Command, command_enum::CommandEnum, compile_error, fields_parse::impl_parse_args,
    Result,
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Main entry point: generate the `BotCommands` trait implementation.
pub(crate) fn bot_commands_impl(input: DeriveInput) -> Result<TokenStream> {
    let data_enum = get_enum_data(&input)?;
    let command_enum = CommandEnum::from_attributes(&input.attrs)?;

    let mut var_inits = Vec::new();
    let mut commands = Vec::new();

    for variant in &data_enum.variants {
        let command = Command::new(&variant.ident.to_string(), &variant.attrs, &command_enum)?;

        let variant_name = &variant.ident;
        let self_variant = quote! { Self::#variant_name };
        let parse_init = impl_parse_args(&variant.fields, self_variant, &command.parser);

        var_inits.push(parse_init);
        commands.push(command);
    }

    let type_name = &input.ident;
    let fn_descriptions = impl_descriptions(&commands, &command_enum);
    let fn_parse = impl_parse(&commands, &var_inits, &command_enum.command_separator);
    let fn_bot_commands = impl_bot_commands(&commands);

    let expanded = quote! {
        impl #type_name {
            /// Parse a command from incoming message text.
            ///
            /// The `bot_name` is used to handle `@BotName` suffixes in commands
            /// (e.g. `/help@MyBot`).
            ///
            /// Returns `Ok(Self)` on a successful parse, or `Err(String)` with a
            /// human-readable error message.
            #fn_parse

            /// Build a formatted help-text string listing all visible commands
            /// and their descriptions.
            #fn_descriptions

            /// Return a list of [`rust_tg_bot_raw::types::bot_command::BotCommand`]
            /// values suitable for the Telegram `setMyCommands` API method.
            #fn_bot_commands
        }
    };

    Ok(expanded)
}

fn impl_bot_commands(infos: &[Command]) -> TokenStream {
    let commands = infos.iter().filter(|c| c.is_visible()).map(|c| {
        let cmd = &c.name;
        let desc = c.description.as_deref().unwrap_or("");
        quote! {
            rust_tg_bot_raw::types::bot_command::BotCommand::new(#cmd, #desc)
        }
    });

    quote! {
        pub fn bot_commands() -> ::std::vec::Vec<rust_tg_bot_raw::types::bot_command::BotCommand> {
            ::std::vec![#(#commands),*]
        }
    }
}

fn impl_descriptions(infos: &[Command], global: &CommandEnum) -> TokenStream {
    let mut lines: Vec<TokenStream> = Vec::new();

    // Optional global description header
    if let Some(ref desc) = global.description {
        lines.push(quote! { parts.push(::std::string::String::from(#desc)); });
        lines.push(quote! { parts.push(::std::string::String::new()); });
    }

    for cmd in infos.iter().filter(|c| c.is_visible()) {
        let prefixed = cmd.prefixed_command();
        let desc = cmd.description.as_deref().unwrap_or("");
        if desc.is_empty() {
            lines.push(quote! {
                parts.push(::std::string::String::from(#prefixed));
            });
        } else {
            lines.push(quote! {
                parts.push(::std::format!("{} \u{2014} {}", #prefixed, #desc));
            });
        }
    }

    quote! {
        pub fn descriptions() -> ::std::string::String {
            let mut parts: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
            #(#lines)*
            parts.join("\n")
        }
    }
}

fn impl_parse(
    infos: &[Command],
    variant_inits: &[TokenStream],
    command_separator: &str,
) -> TokenStream {
    let match_arms: Vec<TokenStream> = infos
        .iter()
        .zip(variant_inits.iter())
        .map(|(cmd, init)| {
            let prefixed = cmd.prefixed_command();
            quote! {
                #prefixed => ::std::result::Result::Ok(#init),
            }
        })
        .collect();

    quote! {
        pub fn parse(
            s: &str,
            bot_name: &str,
        ) -> ::std::result::Result<Self, ::std::string::String> {
            use ::std::str::FromStr;

            // Split into at most 2 parts: the command token, and the rest (arguments).
            let mut words = s.splitn(2, #command_separator);

            // The split iterator always yields at least one element.
            let full_command = words.next().unwrap();
            let mut at_split = full_command.split('@');
            let command = at_split.next().unwrap();

            // Validate the @BotName suffix if present.
            if let ::std::option::Option::Some(username) = at_split.next() {
                if !username.eq_ignore_ascii_case(bot_name) {
                    return ::std::result::Result::Err(
                        ::std::format!("command addressed to wrong bot: @{}", username),
                    );
                }
            }

            let args = words.next().unwrap_or("").to_owned();

            match command {
                #(#match_arms)*
                _ => ::std::result::Result::Err(
                    ::std::format!("unknown command: {}", command),
                ),
            }
        }
    }
}

fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data),
        _ => Err(compile_error("`BotCommands` can only be derived for enums")),
    }
}
