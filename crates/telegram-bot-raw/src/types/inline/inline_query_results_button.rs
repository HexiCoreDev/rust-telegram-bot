use serde::{Deserialize, Serialize};

use crate::types::web_app_info::WebAppInfo;

/// A button to be shown above inline query results.
///
/// You must use exactly one of the optional fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InlineQueryResultsButton {
    /// Label text on the button.
    pub text: String,

    /// Description of the Web App that will be launched when the user presses the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    /// Deep-linking parameter for the /start message sent to the bot when user presses the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

impl_new!(InlineQueryResultsButton { text: String });

impl InlineQueryResultsButton {
    /// Set the Web App to be launched.
    pub fn web_app(mut self, web_app: WebAppInfo) -> Self {
        self.web_app = Some(web_app);
        self
    }

    /// Set the deep-linking start parameter.
    pub fn start_parameter(mut self, param: impl Into<String>) -> Self {
        self.start_parameter = Some(param.into());
        self
    }
}
