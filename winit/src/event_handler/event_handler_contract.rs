use crate::backend::{DeviceId, EventLoop, WindowId};
use crate::event_handler::EventHandler as UserEventHandler;
use winit_core::event::event_handler::EventHandler as BackendEventHandler;
use winit_core::{DeviceEvent, StartCause, WindowEvent};

pub struct EventHandlerContract<T: UserEventHandler>(pub T);

impl<T: UserEventHandler> BackendEventHandler<EventLoop, WindowId, DeviceId>
    for EventHandlerContract<T>
{
    fn new_events(&mut self, event_loop: &EventLoop, cause: StartCause) {
        self.0.new_events(event_loop, cause);
    }

    fn resumed(&mut self, event_loop: &EventLoop) {
        self.0.resumed(event_loop);
    }

    fn can_create_surfaces(&mut self, event_loop: &EventLoop) {
        self.0.can_create_surfaces(event_loop);
    }

    fn proxy_wake_up(&mut self, event_loop: &EventLoop) {
        self.0.proxy_wake_up(event_loop);
    }

    fn window_event(&mut self, event_loop: &EventLoop, window_id: WindowId, event: WindowEvent) {
        self.0.window_event(event_loop, window_id, event);
    }

    fn device_event(
        &mut self,
        event_loop: &EventLoop,
        device_id: Option<DeviceId>,
        event: DeviceEvent,
    ) {
        self.0.device_event(event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &EventLoop) {
        self.0.about_to_wait(event_loop);
    }

    fn suspended(&mut self, event_loop: &EventLoop) {
        self.0.suspended(event_loop);
    }

    fn destroy_surfaces(&mut self, event_loop: &EventLoop) {
        self.0.destroy_surfaces(event_loop);
    }

    fn memory_warning(&mut self, event_loop: &EventLoop) {
        self.0.memory_warning(event_loop);
    }
}
