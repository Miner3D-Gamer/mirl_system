#![allow(unsafe_op_in_unsafe_fn)]
use std::io::Write;

use mirl_buffer::prelude::*;
use mirl_extensions::*;
use mirl_graphics::misc::rasterize_svg;
use mirl_input::{formats, mouse::LoadCursorError};
use windows::{Win32::UI::WindowsAndMessaging::*, core::*};

/// Load a custom cursor
///
/// # Errors
/// When no tempfile could be created
/// When the tempfile could not be deleted
pub fn load_cursor(
    image_data: &Buffer,
    hotspot_x: u8,
    hotspot_y: u8,
) -> core::result::Result<
    windows::Win32::UI::WindowsAndMessaging::HCURSOR,
    LoadCursorError,
> {
    //let size = cursor_resolution(size);

    let Ok((file_path, temp_file)) = create_temp_file(
        &formats::create_cur_simple(
            image_data.data(),
            image_data.get_size(),
            (hotspot_x, hotspot_y),
        )
        .map_or_else(
            || {
                Err(LoadCursorError::InvalidImageData(
                    "Image data/size invalid".to_string(),
                ))
            },
            Ok,
        )?,
    ) else {
        return Err(LoadCursorError::UnableToCreateTempfile);
    };
    let _ = temp_file.keep();

    let temp = load_cursor_file(&file_path);
    if delete_temp_file(&file_path).is_err() {
        return Err(LoadCursorError::UnableToDeleteTempfile);
    }
    temp.map_or(Err(LoadCursorError::OsError), Ok)
}

// fn load_base_cursor(
//     cursor: BaseCursor,
//     size: U2,
//     main_color: u32,
//     secondary_color: u32,
// ) -> windows::Win32::UI::WindowsAndMessaging::HCURSOR {
//     let expected_size = 24; // WHO TF MAKES THE CURSOR NOT A MULTIPLE OF 16 ???

//     let wanted_size = cursor_resolution(size);

//     let path = get_cursor_path(&cursor.file_path.to_string());
//     let svg_data = std::fs::read_to_string(path).unwrap();
//     // if svg has one {}, insert main_color, if svg has two {}, insert main_color, secondary_color

//     let result_svg = svg_data
//         .replace_first_occurrence("{}", &u32_to_hex(main_color))
//         .replace_first_occurrence("{}", &u32_to_hex(secondary_color));

//     let image_data = rasterize_svg(
//         &result_svg.as_bytes(),
//         wanted_size as u32,
//         wanted_size as u32,
//     );

//     // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
//     let adjusted_hotspot_x = ((cursor.hot_spot_x as f64 / expected_size as f64)
//         * wanted_size as f64)
//         .round() as u16;
//     let adjusted_hotspot_y = ((cursor.hot_spot_y as f64 / expected_size as f64)
//         * wanted_size as f64)
//         .round() as u16;

//     return load_cursor(
//         //&extract_file_name_without_extension(&cursor.file_path),
//         size,
//         pixmap_to_buffer(&image_data),
//         adjusted_hotspot_x,
//         adjusted_hotspot_y,
//     );
// }
use mirl_graphics::misc::pixmap_to_buffer;
/// Load a cursor SVG
///
/// # Errors
/// When the image could not be rasterized
#[allow(clippy::needless_pass_by_value)]
pub fn load_svg_cursor_with_file<C: GetCursorResolution>(
    hotspot: (u8, u8),
    size: &C,
    svg_data: &str,
) -> core::result::Result<NativeCursor, LoadCursorError> {
    let wanted_size: u32 = size.get_cursor_resolution();

    let Ok(image_data) = rasterize_svg(svg_data, wanted_size, wanted_size)
    else {
        return Err(LoadCursorError::InvalidImageData(
            "Unable to rasterize svg".to_string(),
        ));
    };

    match load_cursor(&pixmap_to_buffer(&image_data), hotspot.0, hotspot.1) {
        Ok(v) => Ok(NativeCursor {
            cursor: v,
        }),
        Err(e) => Err(e),
    }
}

/// Expects .cur file
///
/// # Errors
/// When windows refuses the file, error stated in string form
///
/// TODO: Change the error handling of this
pub fn load_cursor_file(
    file_path: &str,
) -> core::result::Result<
    windows::Win32::UI::WindowsAndMessaging::HCURSOR,
    &'static str,
> {
    unsafe {
        let Some(filename) = std::ffi::CString::new(file_path).ok() else {
            return Err(
                "Unable to create null-terminated C-style byte string from file path",
            );
        };
        let Ok(cursor) =
            LoadCursorFromFileA(PCSTR(filename.as_ptr().cast::<u8>()))
        else {
            return Err(
                "Windows refused to load. The file used was potentially corrupted",
            );
        };
        if cursor.0 != 0 {
            return Err(
                "Failed to load cursor. Why? No one known, probably not even windows",
            );
        }

        Ok(cursor)
    }
}

/// Create a temporary file, delete it with [`delete_temp_file`]
///
/// # Errors
/// When it is unable to create a file or access it
pub fn create_temp_file(
    cursor_data: &[u8],
) -> std::io::Result<(String, tempfile::NamedTempFile)> {
    let mut file = tempfile::NamedTempFile::new()?;
    file.write_all(cursor_data)?;

    let path_str = file
        .path()
        .to_str()
        .ok_or_else(|| {
            std::io::Error::other("Invalid UTF-8 in temp file path")
        })?
        .to_string();

    //let (file, _path) = file.keep()?; // Keep returns (File, PathBuf)
    Ok((path_str, file))
}
/// Delete the file at the given path
///
/// # Errors
/// When the file could not be found or when permissions aren't given
pub fn delete_temp_file(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path) // Removes the file at the given path
}

/// A windows only function to set the current cursor
pub fn set_cursor(cursor: &windows::Win32::UI::WindowsAndMessaging::HCURSOR) {
    unsafe {
        SetCursor(*cursor);
    }
}
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

use crate::cursors::NativeCursor;

// TODO: Add multi window support?
/// NOT RECOMMENDED TO ACCESS
/// 
/// Stores the winproc of the window from which the cursor is requested
pub static mut ORIGINAL_WNDPROC: Option<WNDPROC> = None;

/// NOT RECOMMENDED TO ACCESS
/// 
/// Stores the currently used cursor for the stored window
pub static mut CURRENT_CURSOR: windows::Win32::UI::WindowsAndMessaging::HCURSOR =
    windows::Win32::UI::WindowsAndMessaging::HCURSOR(0);

/// A hook to attach to a window -> Setting their cursor style
///
/// # Safety
/// Interacts with windows
#[must_use]
pub unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_SETCURSOR {
        // Only handle if we're in the client area (low word of lparam == HTCLIENT)
        if (lparam.0 as u16) == HTCLIENT as u16 && CURRENT_CURSOR.0 != 0 {
            SetCursor(CURRENT_CURSOR);
            return LRESULT(1); // TRUE - we handled it
        }
        // Let default handler deal with non-client areas
    }

    ORIGINAL_WNDPROC.map_or_else(
        || DefWindowProcW(hwnd, msg, wparam, lparam),
        |orig| CallWindowProcW(orig, hwnd, msg, wparam, lparam),
    )
}
#[allow(clippy::fn_to_numeric_cast)]
/// Define a window to attach to so custom cursors can be defined without flickering
///
/// # Safety
/// Interacts with windows
pub unsafe fn subclass_window(
    handle: raw_window_handle::RawWindowHandle,
    cursor: &windows::Win32::UI::WindowsAndMessaging::HCURSOR,
) {
    if let raw_window_handle::RawWindowHandle::Win32(hwnd_handle) = handle
        && let Some(actual_cursor) = Some(cursor)
    {
        let hwnd = windows::Win32::Foundation::HWND(hwnd_handle.hwnd.get());
        CURRENT_CURSOR = *actual_cursor;

        // Get the original window procedure
        let orig_proc = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
        ORIGINAL_WNDPROC = Some(std::mem::transmute::<isize, core::option::Option<unsafe extern "system" fn(windows::Win32::Foundation::HWND, u32, windows::Win32::Foundation::WPARAM, windows::Win32::Foundation::LPARAM) -> windows::Win32::Foundation::LRESULT>>(orig_proc));

        // Set our window procedure
        #[allow(function_casts_as_integer)]
        SetWindowLongPtrW(hwnd, GWLP_WNDPROC, wndproc as isize);
    }
}

/// Helper function to update cursor without re-sub classing
///
/// # Safety
/// Interacts with windows
pub unsafe fn update_cursor(
    cursor: &windows::Win32::UI::WindowsAndMessaging::HCURSOR,
) {
    CURRENT_CURSOR = *cursor;
}
