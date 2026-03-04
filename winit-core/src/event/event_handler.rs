use crate::{DeviceEvent, DeviceId, EventLoop, StartCause, WindowEvent, WindowId};

pub trait EventHandler<EL: EventLoop, WI: WindowId, DI: DeviceId> {
    fn new_events(&mut self, event_loop: &EL, cause: StartCause) {
        let _ = (event_loop, cause);
    }

    fn resumed(&mut self, event_loop: &EL) {
        let _ = event_loop;
    }

    fn can_create_surfaces(&mut self, event_loop: &EL);

    fn proxy_wake_up(&mut self, event_loop: &EL) {
        let _ = event_loop;
    }

    fn window_event(&mut self, event_loop: &EL, window_id: WI, event: WindowEvent);

    fn device_event(&mut self, event_loop: &EL, device_id: Option<DI>, event: DeviceEvent) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &EL) {
        let _ = event_loop;
    }

    fn suspended(&mut self, event_loop: &EL) {
        let _ = event_loop;
    }

    fn destroy_surfaces(&mut self, event_loop: &EL) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &EL) {
        let _ = event_loop;
    }
}
