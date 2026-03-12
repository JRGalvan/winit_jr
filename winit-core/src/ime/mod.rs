pub mod ime_capabilities;
pub mod ime_enable_request;
pub mod ime_hint;
pub mod ime_purpose;
pub mod ime_request;
pub mod ime_request_data;
pub mod ime_surrounding_text;

pub use ime_capabilities::ImeCapabilities;
pub use ime_enable_request::ImeEnableRequest;
pub use ime_hint::ImeHint;
pub use ime_purpose::ImePurpose;
pub use ime_request::ImeRequest;
pub use ime_request_data::ImeRequestData;
pub use ime_surrounding_text::ImeSurroundingText;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ime {
    Enabled,
    Preedit(String, Option<(usize, usize)>),
    Commit(String),
    DeleteSurrounding {
        before_bytes: usize,
        after_bytes: usize,
    },
    Disabled,
}
