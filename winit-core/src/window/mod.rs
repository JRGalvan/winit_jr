pub mod activation_token;
pub mod resize_direction;
pub mod theme;
pub mod user_attention_type;
pub mod window_attributes;
pub mod window_buttons;
pub mod window_level;

pub use activation_token::ActivationToken;
pub use resize_direction::ResizeDirection;
pub use theme::Theme;
pub use user_attention_type::UserAttentionType;
pub use window_buttons::WindowButtons;
pub use window_level::WindowLevel;

use crate::cursor::{Cursor, CursorGrabMode, CustomCursor};
use crate::dpi::{PhysicalInsets, PhysicalPosition, PhysicalSize, Position, Size};
use crate::error::{ImeRequestError, OsError, RequestError};
use crate::icon::Icon;
use crate::id::WindowId;
use crate::ime::{ImeCapabilities, ImeRequest};
use crate::monitor::{Fullscreen, MonitorHandle};
use raw_window_handle as rwh;
use std::fmt;

pub trait Window: Send + Sync + fmt::Debug {
    type WindowId: WindowId;
    type OsError: OsError;
    type MonitorHandle: MonitorHandle;
    type CustomCursor: CustomCursor;

    fn id(&self) -> Self::WindowId;

    fn scale_factor(&self) -> f64;

    fn request_redraw(&self);

    fn pre_present_notify(&self);

    fn reset_dead_keys(&self);

    fn surface_position(&self) -> PhysicalPosition<i32>;

    fn outer_position(&self) -> Result<PhysicalPosition<i32>, RequestError<Self::OsError>>;

    fn set_outer_position(&self, position: Position);

    fn surface_size(&self) -> PhysicalSize<u32>;

    #[must_use]
    fn request_surface_size(&self, size: Size) -> Option<PhysicalSize<u32>>;

    fn outer_size(&self) -> PhysicalSize<u32>;

    fn safe_area(&self) -> PhysicalInsets<u32>;

    fn set_min_surface_size(&self, min_size: Option<Size>);

    fn set_max_surface_size(&self, max_size: Option<Size>);

    fn surface_resize_increments(&self) -> Option<PhysicalSize<u32>>;

    fn set_surface_resize_increments(&self, increments: Option<Size>);

    fn set_title(&self, title: &str);

    fn set_transparent(&self, transparent: bool);

    fn set_blur(&self, blur: bool);

    fn set_visible(&self, visible: bool);

    fn is_visible(&self) -> Option<bool>;

    fn set_resizable(&self, resizable: bool);

    fn is_resizable(&self) -> bool;

    fn set_enabled_buttons(&self, buttons: WindowButtons);

    fn enabled_buttons(&self) -> WindowButtons;

    fn set_minimized(&self, minimized: bool);

    fn is_minimized(&self) -> Option<bool>;

    fn set_maximized(&self, maximized: bool);

    fn is_maximized(&self) -> bool;

    fn set_fullscreen(&self, fullscreen: Option<Fullscreen<Self::MonitorHandle>>);

    fn fullscreen(&self) -> Option<Fullscreen<Self::MonitorHandle>>;

    fn set_decorations(&self, decorations: bool);

    fn is_decorated(&self) -> bool;

    fn set_window_level(&self, level: WindowLevel);

    fn set_window_icon(&self, window_icon: Option<Icon>);

    fn request_ime_update(&self, request: ImeRequest) -> Result<(), ImeRequestError>;

    fn ime_capabilities(&self) -> Option<ImeCapabilities>;

    fn focus_window(&self);

    fn has_focus(&self) -> bool;

    fn request_user_attention(&self, request_type: Option<UserAttentionType>);

    fn set_theme(&self, theme: Option<Theme>);

    fn theme(&self) -> Option<Theme>;

    fn set_content_protected(&self, protected: bool);

    fn title(&self) -> String;

    fn set_cursor(&self, cursor: Cursor<Self::CustomCursor>);

    fn set_cursor_position(&self, position: Position) -> Result<(), RequestError<Self::OsError>>;

    fn set_cursor_grab(&self, mode: CursorGrabMode) -> Result<(), RequestError<Self::OsError>>;

    fn set_cursor_visible(&self, visible: bool);

    fn drag_window(&self) -> Result<(), RequestError<Self::OsError>>;

    fn drag_resize_window(
        &self,
        direction: ResizeDirection,
    ) -> Result<(), RequestError<Self::OsError>>;

    fn show_window_menu(&self, position: Position);

    fn set_cursor_hittest(&self, hittest: bool) -> Result<(), RequestError<Self::OsError>>;

    fn current_monitor(&self) -> Option<Self::MonitorHandle>;

    fn available_monitors(&self) -> impl Iterator<Item = Self::MonitorHandle>;

    fn primary_monitor(&self) -> Option<Self::MonitorHandle>;

    fn rwh_display_handle(&self) -> &dyn rwh::HasDisplayHandle;

    fn rwh_window_handle(&self) -> &dyn rwh::HasWindowHandle;
}
