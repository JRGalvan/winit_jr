use crate::ime::{ImeEnableRequest, ImeRequestData};

#[derive(Debug, PartialEq, Clone)]
pub enum ImeRequest {
    Enable(ImeEnableRequest),
    Update(ImeRequestData),
    Disable,
}
