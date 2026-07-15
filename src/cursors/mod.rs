use std::ops::Add;

#[cfg(target_os = "windows")]
use mirl_buffer::traits::BufferMetricsHelper;
use mirl_extensions::TupleOps;
use mirl_input::mouse::{
    DefaultCursorColorInfo, LoadCursorError, LoadDefaultCustomCursor,
    cursors::{CursorResolution, DefaultCursors, RawCursor},
    loading::get_raw_default_cursors,
};
#[cfg(target_os = "windows")]
/// Platform specific cursor related functions
///
/// TODO: Merge this into the action category
pub mod windows;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Errors that might occur when making a window use a cursor
pub enum UseCursorError {
    /// The currently used os is not supported
    UnsupportedOs,
}
/// Create a new and use the new cursor
pub const trait CursorType<Input>:
    core::marker::Sized + std::fmt::Debug + Clone + PartialEq
{
    /// Create a new valid cursor
    ///
    /// # Errors
    /// [`LoadCursorError`]
    fn new(cursor: RawCursor) -> Result<Self, LoadCursorError>;
    /// Use the cursor
    ///
    /// # Errors
    /// [`LoadCursorError`]
    fn use_cursor(&self, input: Input) -> Result<(), UseCursorError>;
}
#[cfg(target_os = "windows")]
mod windows_dep {
    pub(super) use ::windows::Win32::UI::WindowsAndMessaging;
}
#[cfg(target_os = "windows")]
use windows_dep::*;

#[cfg_attr(feature = "c_compatible", repr(C))]
#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The native cursor for windows
pub struct NativeCursor {
    /// The cursor
    pub cursor: WindowsAndMessaging::HCURSOR,
}
#[must_use]
/// Check if the given metrics are valid for cursor creation
pub fn is_valid_cursor_metrics(data: &[u32], size: (usize, usize)) -> bool {
    (size == (16, 16).sub(1)
        || size == (24, 24).sub(1)
        || size == (32, 32).sub(1)
        || size == (48, 48).sub(1)
        || size == (64, 64).sub(1)
        || size == (96, 96).sub(1)
        || size == (128, 128).sub(1)
        || size == (256, 256).sub(1))
        && data.len() == size.0.add(1) * size.1.add(1)
}

#[cfg(target_os = "windows")]
impl CursorType<()> for NativeCursor {
    fn use_cursor(&self, _input: ()) -> Result<(), UseCursorError> {
        unsafe {
            // Update the passive cursor provider windows uses
            crate::cursors::windows::update_cursor(&self.cursor);
            // Update currently used cursor (Passive provider only gets checked when mouse moves)
            crate::cursors::windows::set_cursor(&self.cursor);
        }
        Ok(())
    }
    fn new(raw_cursor: RawCursor) -> Result<Self, LoadCursorError> {
        if !is_valid_cursor_metrics(&raw_cursor.image, raw_cursor.image.get_size()) {
            return Err(LoadCursorError::InvalidImageData(
                "Invalid metrics".to_string(),
            ));
        }
        // TODO: Implement Os specific cursor implementation
        Err(LoadCursorError::UnsupportedOs)
    }
}

impl LoadDefaultCustomCursor for NativeCursor {
    fn get_default_custom_cursors(
        size: CursorResolution,
        color_info: DefaultCursorColorInfo,
    ) -> Result<DefaultCursors<Self>, LoadCursorError> {
        get_windows_default_cursors(size, color_info)
    }
}

/// Get the default cursors in image + hotspot form ([`DefaultRawSVGCursor`])
///
/// # Errors
/// Any errors can be ignored. It is safe to use `.unwrap()` on this function
///
/// TODO: Remove the [`Result`]
pub fn get_windows_default_cursors(
    size: CursorResolution,
    color_info: DefaultCursorColorInfo,
) -> Result<DefaultCursors<NativeCursor>, LoadCursorError> {
    let raw_cursors = get_raw_default_cursors(size, color_info);

    #[rustfmt::skip]
    let final_cursors = DefaultCursors {
        alias:                  NativeCursor::new(raw_cursors.alias)?,
        resize_all:             NativeCursor::new(raw_cursors.resize_all)?,
        arrow_bottom_left:      NativeCursor::new(raw_cursors.arrow_bottom_left)?,
        arrow_bottom_right:     NativeCursor::new(raw_cursors.arrow_bottom_right)?,
        arrow_bottom_stop:      NativeCursor::new(raw_cursors.arrow_bottom_stop)?,
        cell:                   NativeCursor::new(raw_cursors.cell)?,
        centered_pointer:       NativeCursor::new(raw_cursors.centered_pointer)?,
        color_picker:           NativeCursor::new(raw_cursors.color_picker)?,
        context_menu:           NativeCursor::new(raw_cursors.context_menu)?,
        copy:                   NativeCursor::new(raw_cursors.copy)?,
        crosshair:              NativeCursor::new(raw_cursors.crosshair)?,
        pointer:                NativeCursor::new(raw_cursors.pointer)?,
        closed_hand:            NativeCursor::new(raw_cursors.closed_hand)?,
        closed_hand_no_drop:    NativeCursor::new(raw_cursors.closed_hand_no_drop)?,
        arrow_down:             NativeCursor::new(raw_cursors.arrow_down)?,
        draft:                  NativeCursor::new(raw_cursors.draft)?,
        fleur:                  NativeCursor::new(raw_cursors.fleur)?,
        help:                   NativeCursor::new(raw_cursors.help)?,
        arrow_left:             NativeCursor::new(raw_cursors.arrow_left)?,
        arrow_left_stop:        NativeCursor::new(raw_cursors.arrow_left_stop)?,
        no_drop:                NativeCursor::new(raw_cursors.no_drop)?,
        not_allowed:            NativeCursor::new(raw_cursors.not_allowed)?,
        open_hand:              NativeCursor::new(raw_cursors.open_hand)?,
        pencil:                 NativeCursor::new(raw_cursors.pencil)?,
        pirate:                 NativeCursor::new(raw_cursors.pirate)?,
        hand:                   NativeCursor::new(raw_cursors.hand)?,
        arrow_right:            NativeCursor::new(raw_cursors.arrow_right)?,
        mirrored_pointer:       NativeCursor::new(raw_cursors.mirrored_pointer)?,
        arrow_right_stop:       NativeCursor::new(raw_cursors.arrow_right_stop)?,
        resize_nesw:            NativeCursor::new(raw_cursors.resize_nesw)?,
        resize_nwse:            NativeCursor::new(raw_cursors.resize_nwse)?,
        resize_horizontal:      NativeCursor::new(raw_cursors.resize_horizontal)?,
        resize_vertical:        NativeCursor::new(raw_cursors.resize_vertical)?,
        text:                   NativeCursor::new(raw_cursors.text)?,
        arrow_top_left:         NativeCursor::new(raw_cursors.arrow_top_left)?,
        arrow_top_right:        NativeCursor::new(raw_cursors.arrow_top_right)?,
        arrow_top_stop:         NativeCursor::new(raw_cursors.arrow_top_stop)?,
        arrow_up:               NativeCursor::new(raw_cursors.arrow_up)?,
        vertical_text:          NativeCursor::new(raw_cursors.vertical_text)?,
        zoom_in:                NativeCursor::new(raw_cursors.zoom_in)?,
        zoom_out:               NativeCursor::new(raw_cursors.zoom_out)?,
    };

    Ok(final_cursors)
}
