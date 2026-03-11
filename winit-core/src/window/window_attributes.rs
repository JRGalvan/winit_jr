use crate::cursor::{Cursor, CustomCursor};
use crate::dpi::{Position, Size};
use crate::icon::Icon;
use crate::monitor::{Fullscreen, MonitorHandle};
use crate::window::{Theme, WindowButtons, WindowLevel};
use raw_window_handle as rwh;
use std::fmt;

pub trait WindowAttributes: Clone + fmt::Debug + Default {
    fn parent_window(&self) -> Option<&rwh::RawWindowHandle>;

    fn with_min_surface_size<S: Into<Size>>(self, min_size: S) -> Self;
    fn with_surface_size<S: Into<Size>>(self, size: S) -> Self;
    fn with_max_surface_size<S: Into<Size>>(self, max_size: S) -> Self;

    fn with_surface_resize_increments<S: Into<Size>>(self, surface_resize_increments: S) -> Self;

    fn with_position<P: Into<Position>>(self, position: P) -> Self;

    fn with_resizable(self, resizable: bool) -> Self;

    fn with_enabled_buttons(self, buttons: WindowButtons) -> Self;

    fn with_title<T: Into<String>>(self, title: T) -> Self;

    fn with_fullscreen<MH: MonitorHandle>(self, fullscreen: Option<Fullscreen<MH>>) -> Self;

    fn with_maximized(self, maximized: bool) -> Self;

    fn with_visible(self, visible: bool) -> Self;

    fn with_transparent(self, transparent: bool) -> Self;

    fn with_blur(self, blur: bool) -> Self;

    fn with_decorations(self, decorations: bool) -> Self;

    fn with_window_level(self, level: WindowLevel) -> Self;

    fn with_window_icon(self, window_icon: Option<Icon>) -> Self;

    fn with_theme(self, theme: Option<Theme>) -> Self;

    fn with_content_protected(self, protected: bool) -> Self;

    fn with_active(self, active: bool) -> Self;

    fn with_cursor<CC: CustomCursor>(self, cursor: impl Into<Cursor<CC>>) -> Self;

    unsafe fn with_parent_window(self, parent_window: Option<rwh::RawWindowHandle>) -> Self;

    fn transparent(&self) -> bool;
}
