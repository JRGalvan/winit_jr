use crate::ime::{ImeCapabilities, ImeRequestData};

#[derive(Debug, Clone, PartialEq)]
pub struct ImeEnableRequest {
    capabilities: ImeCapabilities,
    request_data: ImeRequestData,
}

impl ImeEnableRequest {
    pub fn new(capabilities: ImeCapabilities, request_data: ImeRequestData) -> Option<Self> {
        if capabilities.cursor_area() ^ request_data.cursor_area.is_some() {
            return None;
        }

        if capabilities.hint_and_purpose() ^ request_data.hint_and_purpose.is_some() {
            return None;
        }

        if capabilities.surrounding_text() ^ request_data.surrounding_text.is_some() {
            return None;
        }
        Some(Self {
            capabilities,
            request_data,
        })
    }

    pub const fn capabilities(&self) -> &ImeCapabilities {
        &self.capabilities
    }

    pub const fn request_data(&self) -> &ImeRequestData {
        &self.request_data
    }

    pub fn into_raw(self) -> (ImeCapabilities, ImeRequestData) {
        (self.capabilities, self.request_data)
    }
}
