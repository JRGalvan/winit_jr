pub mod cursor;
pub mod device_id;
pub mod dpi;
pub mod event;
pub mod icon;
pub mod keyboard;
pub mod monitor;
pub mod os_error;
pub mod window;

pub use crate::device_id::DeviceId;
pub use crate::event::device_event::DeviceEvent;
pub use crate::event::event_loop::EventLoop;
pub use crate::event::start_cause::StartCause;
pub use crate::event::window_event::WindowEvent;
pub use crate::window::window_id::WindowId;

#[allow(non_camel_case_types)]
pub enum BackendType {
    windows,
    macos,
}
