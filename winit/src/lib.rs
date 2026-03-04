//! ```no_run
//! use winit::event::WindowEvent;
//! use winit::event_handler::EventHandler;
//! use winit::event_loop::{EventLoop, EventLoopBuilder};
//! use winit::window::{Window, WindowId, WindowAttributes};
//!
//! #[derive(Default)]
//! struct Game {
//!     window: Option<Window>,
//! }
//!
//! impl EventHandler for Game {
//!     fn resumed(&mut self, event_loop: &EventLoop) {
//!         self.window = Some(event_loop.create_window(WindowAttributes::default()).unwrap());
//!     }
//!
//!     fn window_event(&mut self, event_loop: &EventLoop, id: WindowId, event: WindowEvent) {
//!         match event {
//!             WindowEvent::CloseRequested => event_loop.exit(),
//!             WindowEvent::RedrawRequested => self.window.as_ref().unwrap().request_redraw(),
//!             _ => (),
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let event_loop_builder = EventLoopBuilder::new();
//!
//!     let mut game = Game::default();
//!
//!     event_loop_builder.build(game);
//! }
//!  ```

// **WindowId + EventHandler Redesign**

//- `WindowId` (and similar types) become **traits** in `winit-core` — purely a compile-time correctness contract for backend authors
//- Each backend (`winit-windows`, `winit-macos`) defines its own concrete struct implementing the trait
//- `winit` uses `#[cfg(target_os)]` to re-export the right backend's concrete type — users see a plain struct, no traits, no generics
//- `EventHandler` moves from `winit-core` into `winit`, defined using those concrete re-exported types
//- No `BackendEventHandler` needed — the backend exposes a `run(callback: impl FnMut(...))` and `winit` wraps the user's `EventHandler` in a closure to pass down
//- We'll see if we really need a BackendEventHandler or not. But in case that we do, that backendeventhandler will be more complex as everything should be a trait over multiple dyn traits

// The tricky part when implementing: driving the platform event loop (NSRunLoop / Win32 message loop) in the backend and correctly translating raw platform events into your types inside that closure.

pub mod event_handler;
pub mod event_loop_builder;

#[cfg(target_os = "macos")]
pub use winit_macos as backend;
#[cfg(target_os = "windows")]
pub use winit_windows as backend;
