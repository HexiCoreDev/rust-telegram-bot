use serde::{Deserialize, Serialize};

use super::input_contact_message_content::InputContactMessageContent;
use super::input_invoice_message_content::InputInvoiceMessageContent;
use super::input_location_message_content::InputLocationMessageContent;
use super::input_rich_message_content::InputRichMessageContent;
use super::input_shared_widget_message_content::InputSharedWidgetMessageContent;
use super::input_text_message_content::InputTextMessageContent;
use super::input_venue_message_content::InputVenueMessageContent;
use super::input_widget_message_content::InputWidgetMessageContent;

/// Content of a message to be sent as the result of an inline query.
///
/// Serde uses untagged deserialization: the variant is inferred from the fields present.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum InputMessageContent {
    /// A text message.
    Text(InputTextMessageContent),
    /// A venue.
    Venue(InputVenueMessageContent),
    /// A location.
    Location(InputLocationMessageContent),
    /// A contact.
    Contact(InputContactMessageContent),
    /// An invoice.
    Invoice(InputInvoiceMessageContent),
    /// A rich message.
    Rich(InputRichMessageContent),
    /// A message with an HTML widget.
    Widget(InputWidgetMessageContent),
    /// A message with a shared widget.
    SharedWidget(InputSharedWidgetMessageContent),
}
