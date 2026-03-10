use crate::dpi::PhysicalPosition;
use crate::monitor::VideoMode;
use std::borrow::Cow;
use std::fmt;

pub trait MonitorHandle: Clone + fmt::Debug + PartialEq + Eq + Send + Sync {
    fn id(&self) -> u128;

    fn native_id(&self) -> u64;

    fn name(&self) -> Option<Cow<'_, str>>;

    fn position(&self) -> Option<PhysicalPosition<i32>>;

    fn scale_factor(&self) -> f64;

    fn current_video_mode(&self) -> Option<VideoMode>;

    fn video_modes(&self) -> impl Iterator<Item = VideoMode>;
}
