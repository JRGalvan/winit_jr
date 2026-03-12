use crate::dpi::PhysicalPosition;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(PhysicalPosition<f64>),
}
