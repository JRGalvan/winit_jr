pub mod event_handler_contract;

use crate::backend::{DeviceId, EventLoop, WindowId};
use winit_core::{DeviceEvent, StartCause, WindowEvent};

pub trait EventHandler {
    fn new_events(&mut self, event_loop: &EventLoop, cause: StartCause) {
        let _ = (event_loop, cause);
    }

    fn resumed(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }

    fn can_create_surfaces(&mut self, event_loop: &EventLoop);

    fn proxy_wake_up(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }

    fn window_event(&mut self, event_loop: &EventLoop, window_id: WindowId, event: WindowEvent);

    fn device_event(
        &mut self,
        event_loop: &EventLoop,
        device_id: Option<DeviceId>,
        event: DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }

    fn suspended(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }

    fn destroy_surfaces(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }
}
