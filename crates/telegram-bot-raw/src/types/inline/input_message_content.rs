use serde::{Deserialize, Serialize};

use super::input_contact_message_content::InputContactMessageContent;
use super::input_invoice_message_content::InputInvoiceMessageContent;
use super::input_location_message_content::InputLocationMessageContent;
use super::input_text_message_content::InputTextMessageContent;
use super::input_venue_message_content::InputVenueMessageContent;

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
}
