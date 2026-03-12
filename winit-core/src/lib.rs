pub mod cursor;
pub mod dpi;
pub mod error;
pub mod event;
pub mod icon;
pub mod id;
pub mod ime;
pub mod keyboard;
pub mod modifiers;
pub mod monitor;
pub mod tablet;
pub mod window;

pub use crate::event::DeviceEvent;
pub use crate::event::EventLoop;
pub use crate::event::StartCause;
pub use crate::event::WindowEvent;
pub use crate::id::{DeviceId, WindowId};

#[allow(non_camel_case_types)]
pub enum BackendType {
    windows,
    macos,
}
