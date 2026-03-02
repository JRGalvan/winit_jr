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
