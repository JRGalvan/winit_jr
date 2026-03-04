//! winit-core is the crate that designs and holds the core types and traits used in winit.
//! If you want to implement a backend for winit, this is the place to start.
//!
//! The event mod holds everything related to events. It is the home for the WindowEvent, the DeviceEvent, the EventLoop, the EventLoopBuilder and the EventHandler.
//!
//! The window mod holds everything related to the window. It is the home for the Window, WindowId, and the WindowAttributes.
//!
//! The os_error mod holds the OS-specific error types and traits that may be generated when using winit.
//!
//! The path to create a new backend is to implement the EventLoopBuilder trait, the EventLoop trait, the Window trait.

pub mod device_id;
pub mod dpi;
pub mod event;
pub mod os_error;
pub mod window;

pub use crate::device_id::DeviceId;
pub use crate::event::device_event::DeviceEvent;
pub use crate::event::event_loop::EventLoop;
pub use crate::event::start_cause::StartCause;
pub use crate::event::window_event::WindowEvent;
pub use crate::window::window_id::WindowId;
