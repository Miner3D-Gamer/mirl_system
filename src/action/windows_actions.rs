#![allow(unsafe_op_in_unsafe_fn)]
// TODO: Make all raw functions unsafe

/// `OsImplementation` for Window
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct WindowsActions {}

impl SetWindowPosition for WindowsActions {
    fn set_window_position(handle: &raw_window_handle::RawWindowHandle, x: i32, y: i32) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                set_window_position_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()), x, y);
                true
            }
            _ => false,
        }
    }
}

impl DefaultWindowing for WindowsActions {
    fn set_window_level(
        handle: &raw_window_handle::RawWindowHandle,
        level: WindowRenderLayer,
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                set_window_level_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()), level);
                true
            }
            _ => false,
        }
    }
    fn get_window_position(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32) {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_position_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()))
                    .unwrap_or((i32::MIN, i32::MIN))
            }
            _ => (i32::MIN, i32::MIN),
        }
    }
    fn get_window_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32) {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_size_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()))
            }
            _ => (i32::MIN, i32::MIN),
        }
    }
    fn get_window_hitbox_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32) {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_hitbox_size_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()))
                    .unwrap_or((i32::MIN, i32::MIN))
            }
            _ => (i32::MIN, i32::MIN),
        }
    }
    fn set_window_size(handle: &raw_window_handle::RawWindowHandle, size: (i32, i32)) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                resize(handle.hwnd.get() as *mut std::ffi::c_void, size);
                true
            }
            _ => false,
        }
    }
}

impl Transparency for WindowsActions {
    fn make_color_transparent(
        handle: &raw_window_handle::RawWindowHandle,
        color: (u8, u8, u8),
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => make_color_transparent_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
                color,
                0,
            ),
            _ => false,
        }
    }
    fn set_window_opacity(handle: &raw_window_handle::RawWindowHandle, opacity: u8) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                set_window_opacity_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    opacity,
                );
                true
            }
            _ => false,
        }
    }
}
impl Decoration for WindowsActions {
    fn set_window_borderless(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                if boolean {
                    make_window_borderless_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()));
                } else {
                    give_window_a_border_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()));
                }
                true
            }
            _ => false,
        }
    }
}
impl Misc for WindowsActions {
    fn set_window_hidden_from_taskbar_and_alt_tab(
        handle: &raw_window_handle::RawWindowHandle,
        boolean: bool,
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                if boolean {
                    hide_from_taskbar_and_alt_tab_raw(windows::Win32::Foundation::HWND(
                        handle.hwnd.get(),
                    ));
                } else {
                    show_in_taskbar_and_alt_tab_raw(windows::Win32::Foundation::HWND(
                        handle.hwnd.get(),
                    ));
                }
                true
            }
            _ => false,
        }
    }
    fn get_all_windows() -> Vec<raw_window_handle::RawWindowHandle> {
        let windows = get_all_windows_raw();
        let mut new = Vec::new();
        for i in windows {
            new.push(raw_window_handle::RawWindowHandle::Win32(
                raw_window_handle::Win32WindowHandle::new(unsafe {
                    core::num::NonZero::new(i.0).unwrap_unchecked()
                }),
            ));
        }
        new
    }
    fn get_title_using_id(handle: &raw_window_handle::RawWindowHandle) -> String {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_title_using_id_raw(handle.hwnd.get() as *mut std::ffi::c_void)
            }
            _ => String::new(),
        }
    }
    fn get_id_using_title(
        title: &str,
        exact_match: bool,
        case_sensitive: bool,
        include_hidden: bool,
        just_one: bool,
    ) -> Option<Vec<raw_window_handle::RawWindowHandle>> {
        get_window_id_by_title(title, exact_match, case_sensitive, include_hidden, just_one)
    }
    fn capture_screen() -> Option<Buffer> {
        capture_screen_raw()
    }
    fn capture_desktop_background() -> Option<Buffer> {
        capture_desktop_background_image()
    }
    fn set_click_ability_of_window(
        handle: &raw_window_handle::RawWindowHandle,
        click_through: bool,
    ) {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            if click_through {
                make_window_click_through_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()));
            } else {
                make_window_click_solid_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()));
            }
        }
    }
    fn get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32 {
        match handle {
            #[cfg(target_os = "windows")]
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_z_raw(windows::Win32::Foundation::HWND(handle.hwnd.get())).unwrap_or(u32::MIN)
            }
            _ => u32::MIN,
        }
    }
    fn set_window_z(handle: &raw_window_handle::RawWindowHandle, z: u32) -> bool {
        match handle {
            #[cfg(target_os = "windows")]
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                set_z_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()), z);
                true
            }
            _ => false,
        }
    }
    fn set_window_z_after(
        handle: &raw_window_handle::RawWindowHandle,
        after: &raw_window_handle::RawWindowHandle,
    ) -> bool {
        // Matching 2 values at the same time feels cursed
        match (handle, after) {
            #[cfg(target_os = "windows")]
            (
                raw_window_handle::RawWindowHandle::Win32(handle),
                raw_window_handle::RawWindowHandle::Win32(after),
            ) => {
                set_z_to_after_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    windows::Win32::Foundation::HWND(after.hwnd.get()),
                );
                true
            }
            _ => false,
        }
    }
    fn set_cpu_priority(handle: &raw_window_handle::RawWindowHandle, priority: ProcessCpuPriority) {
        if let raw_window_handle::RawWindowHandle::Win32(hwnd) = handle {
            set_cpu_priority(windows::Win32::Foundation::HWND(hwnd.hwnd.get()), priority);
        }
    }
}

// extern crate winapi;

use mirl_buffer::Buffer;
use mirl_graphics::u32_color_casting::PackChannelsIntoColor;
// use mirl_extensions::*;
// TODO: Replace these imports with full paths where the items are used
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GetClientRect, GetWindowLongW, SWP_NOSIZE, SWP_NOZORDER, SetWindowLongW,
    WS_EX_LAYERED, WS_EX_TRANSPARENT,
};
use windows::{
    Win32::Graphics::Gdi::{
        BitBlt,
        CreateCompatibleBitmap,
        CreateCompatibleDC,
        DeleteDC,
        DeleteObject,
        GetDC,
        ReleaseDC,
        SRCCOPY, //GetPixel
        SelectObject,
    },
    Win32::UI::WindowsAndMessaging::{GetDesktopWindow, GetShellWindow, GetWindowRect},
};

use crate::traits::*;
#[must_use]
/// Create a screenshot
///
/// TODO: Add better return result
#[allow(trivial_casts)]
pub fn capture_screen_raw() -> Option<Buffer> {
    unsafe {
        // Get the desktop window handle
        let desktop_hwnd = GetDesktopWindow();
        if desktop_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = windows::Win32::Foundation::RECT::default();
        if !GetWindowRect(desktop_hwnd, &raw mut rect).as_bool() {
            return None;
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Get the device context for the desktop
        let hdc = GetDC(desktop_hwnd);
        if hdc.is_invalid() {
            return None;
        }

        // Create a compatible DC and bitmap
        let hdc_mem = CreateCompatibleDC(hdc);
        if hdc_mem.is_invalid() {
            ReleaseDC(desktop_hwnd, hdc);
            return None;
        }

        let hbitmap = CreateCompatibleBitmap(hdc, width, height);
        if hbitmap.is_invalid() {
            DeleteDC(hdc_mem);
            ReleaseDC(desktop_hwnd, hdc);
            return None;
        }

        // Select the bitmap into the compatible DC
        let old_obj = SelectObject(hdc_mem, hbitmap);

        // Copy screen contents to our bitmap
        let result = BitBlt(
            hdc_mem, 0, 0, width, height, hdc, rect.left, rect.top, SRCCOPY,
        );

        // Create a vector to store the pixel data
        // Calculate the correct size: width * height (as usize to prevent overflow)
        let size = (width as usize) * (height as usize);
        let mut pixels = vec![0u32; size];

        // Get the bitmap data
        let bmi = windows::Win32::Graphics::Gdi::BITMAPINFO {
            bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<windows::Win32::Graphics::Gdi::BITMAPINFOHEADER>()
                    as u32,
                biWidth: width,
                biHeight: -height, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB is 0
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
        };

        let _scan_lines = windows::Win32::Graphics::Gdi::GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr().cast::<std::ffi::c_void>()),
            (&raw const bmi).cast_mut(),
            windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
        );

        // Clean up
        SelectObject(hdc_mem, old_obj);
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(desktop_hwnd, hdc);

        if !result.as_bool() {
            return None;
        }
        Buffer::new((width as usize, height as usize), pixels).ok()
    }
}
#[must_use]
/// Capture the desktop background image
///
/// TODO: Add better return result
#[allow(trivial_casts)]
pub fn capture_desktop_background_image() -> Option<Buffer> {
    unsafe {
        // Get the shell window handle (desktop background + icons)
        let shell_hwnd = GetShellWindow();
        if shell_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = windows::Win32::Foundation::RECT::default();
        if !GetWindowRect(shell_hwnd, &raw mut rect).as_bool() {
            return None;
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Get the device context for the shell window
        let hdc = GetDC(shell_hwnd);
        if hdc.is_invalid() {
            return None;
        }

        // Create a compatible DC and bitmap
        let hdc_mem = CreateCompatibleDC(hdc);
        if hdc_mem.is_invalid() {
            ReleaseDC(shell_hwnd, hdc);
            return None;
        }

        let hbitmap = CreateCompatibleBitmap(hdc, width, height);
        if hbitmap.is_invalid() {
            DeleteDC(hdc_mem);
            ReleaseDC(shell_hwnd, hdc);
            return None;
        }

        // Select the bitmap into the compatible DC
        let old_obj = SelectObject(hdc_mem, hbitmap);

        // Copy screen contents to our bitmap
        let result = BitBlt(
            hdc_mem, 0, 0, width, height, hdc, rect.left, rect.top, SRCCOPY,
        );

        // Create a vector to store the pixel data
        let size = (width as usize) * (height as usize);
        let mut pixels = vec![0u32; size];

        // Get the bitmap data
        let bmi = windows::Win32::Graphics::Gdi::BITMAPINFO {
            bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<windows::Win32::Graphics::Gdi::BITMAPINFOHEADER>()
                    as u32,
                biWidth: width,
                biHeight: -height, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB is 0
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
        };

        let _scan_lines = windows::Win32::Graphics::Gdi::GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr().cast::<std::ffi::c_void>()),
            (&raw const bmi).cast_mut(),
            windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
        );

        // Clean up
        SelectObject(hdc_mem, old_obj);
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(shell_hwnd, hdc);

        if !result.as_bool() {
            return None;
        }

        Buffer::new((width as usize, height as usize), pixels).ok()
    }
}
use windows::Win32::Foundation::COLORREF;
#[must_use]
/// Make a selected color transparent for a window
pub fn make_color_transparent_raw(
    hwnd: windows::Win32::Foundation::HWND,
    color: (u8, u8, u8),
    alpha: u8,
) -> bool {
    // TODO: If alpha is supported, change this rgb to rgba (u32)
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32);

        let colorref = COLORREF((color.0, color.1, color.2).pack_rgba_u32());

        windows::Win32::UI::WindowsAndMessaging::SetLayeredWindowAttributes(
            hwnd,
            colorref,
            alpha,
            windows::Win32::UI::WindowsAndMessaging::LWA_COLORKEY,
        )
        .as_bool()
    }
}
#[must_use]
/// Get the position of a window if that window exists
pub fn get_window_position_raw(hwnd: windows::Win32::Foundation::HWND) -> Option<(i32, i32)> {
    unsafe {
        let mut rect = windows::Win32::Foundation::RECT::default();
        if GetWindowRect(hwnd, &raw mut rect).as_bool() {
            Some((rect.left, rect.top))
        } else {
            None
        }
    }
}
/// Set the position of a window
///
/// TODO: Check the other parameters of `windows::Win32::UI::WindowsAndMessaging::SetWindowPos`
pub fn set_window_position_raw(hwnd: windows::Win32::Foundation::HWND, x: i32, y: i32) {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            windows::Win32::Foundation::HWND(0),
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER,
        );
    }
}
/// Make it so clicking the window clicks whats below it
///
/// Inverse: [`make_window_click_solid_raw`]
pub fn make_window_click_through_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style | WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as i32,
        );
    }
}
/// Make it so clickign the window doesn't click whats below it
///
/// Inverse: [`make_window_click_through_raw`]
pub fn make_window_click_solid_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style & !(WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0)) as i32,
        );
    }
}
/// Remove the decoration from a window
///
/// Inverse: [`give_window_a_border_raw`]
pub fn make_window_borderless_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let style = GetWindowLongW(hwnd, windows::Win32::UI::WindowsAndMessaging::GWL_STYLE) as u32;
        SetWindowLongW(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::GWL_STYLE,
            (style
                & !(windows::Win32::UI::WindowsAndMessaging::WS_CAPTION.0
                    | windows::Win32::UI::WindowsAndMessaging::WS_THICKFRAME.0)) as i32,
        );
        windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            windows::Win32::Foundation::HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}

/// Give back the decoration to a window
///
/// Inverse: [`make_window_borderless_raw`]
pub fn give_window_a_border_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let style = GetWindowLongW(hwnd, windows::Win32::UI::WindowsAndMessaging::GWL_STYLE) as u32;
        SetWindowLongW(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::GWL_STYLE,
            (style
                | windows::Win32::UI::WindowsAndMessaging::WS_CAPTION.0
                | windows::Win32::UI::WindowsAndMessaging::WS_THICKFRAME.0) as i32,
        );
        windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            windows::Win32::Foundation::HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}
/// Hide a window from the taskbar and alt tab
///
/// Inverse: [`show_in_taskbar_and_alt_tab_raw`]
pub fn hide_from_taskbar_and_alt_tab_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            ((ex_style | windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW.0)
                & !windows::Win32::UI::WindowsAndMessaging::WS_EX_APPWINDOW.0) as i32,
        );
        windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            windows::Win32::Foundation::HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}
/// Show a window on the taskbar and alt tab
///
/// Inverse: [`hide_from_taskbar_and_alt_tab_raw`]
pub fn show_in_taskbar_and_alt_tab_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            ((ex_style | windows::Win32::UI::WindowsAndMessaging::WS_EX_APPWINDOW.0)
                & !windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW.0) as i32,
        );
        windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            windows::Win32::Foundation::HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}
/// Set the transparency of the window
pub fn set_window_opacity_raw(hwnd: windows::Win32::Foundation::HWND, alpha: u8) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(hwnd, GWL_EXSTYLE, (ex_style | WS_EX_LAYERED.0) as i32);
        windows::Win32::UI::WindowsAndMessaging::SetLayeredWindowAttributes(
            hwnd,
            windows::Win32::Foundation::COLORREF(0),
            alpha,
            windows::Win32::UI::WindowsAndMessaging::LWA_ALPHA,
        );
    }
}
#[must_use]
/// Get the Z order of the window if it exists
pub fn get_z_raw(hwnd: windows::Win32::Foundation::HWND) -> Option<u32> {
    unsafe {
        let mut order = 0;
        let mut current = windows::Win32::UI::WindowsAndMessaging::GetTopWindow(
            windows::Win32::Foundation::HWND(0),
        );
        while current.0 != 0 {
            if current.0 == hwnd.0 {
                return Some(order);
            }
            current = windows::Win32::UI::WindowsAndMessaging::GetWindow(
                current,
                windows::Win32::UI::WindowsAndMessaging::GW_HWNDNEXT,
            );
            order += 1;
        }
    }
    None
}

/// Set the Z value to be after another Z
pub fn set_z_to_after_raw(
    hwnd: windows::Win32::Foundation::HWND,
    insert_after: windows::Win32::Foundation::HWND,
) {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            insert_after,
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE,
        );
    }
}
/// Set the raw z order of a window
pub fn set_z_raw(hwnd: windows::Win32::Foundation::HWND, index: u32) {
    unsafe {
        let mut order = 0;
        let mut current = windows::Win32::UI::WindowsAndMessaging::GetTopWindow(
            windows::Win32::Foundation::HWND(0),
        );
        while order == index {
            if current.0 == hwnd.0 {
                windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
                    hwnd,
                    current,
                    0,
                    0,
                    0,
                    0,
                    windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                        | SWP_NOSIZE
                        | windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE,
                );
            }
            current = windows::Win32::UI::WindowsAndMessaging::GetWindow(
                current,
                windows::Win32::UI::WindowsAndMessaging::GW_HWNDNEXT,
            );
            order += 1;
        }
    }
}

// fn get_desktop_pixel_color(x: i32, y: i32) -> u32 {
//     unsafe {
//         // Get the desktop window handle directly
//         let desktop_hwnd = GetDesktopWindow();
//         if desktop_hwnd.0 == 0 {
//             return 0;
//         }

//         // Get the device context for the desktop window
//         let hdc = GetDC(desktop_hwnd);
//         if hdc.is_invalid() {
//             return 0;
//         }

//         // Make sure we release the DC even if getting the pixel fails
//         let result = std::panic::catch_unwind(|| {
//             let pixel_color = GetPixel(hdc, x, y).0;
//             if pixel_color == u32::MAX {
//                 return Err(windows::core::Error::new(
//                     windows::core::HRESULT(0),
//                     "Failed to get pixel color".into(),
//                 ));
//             }
//             return Ok(pixel_color);
//             // // Convert to u32 first, then perform the bitwise operations
//             // let color_u32: u32 = pixel_color;

//             // Ok(PixelColor {
//             //     r: (color_u32 & 0xFF) as u8,
//             //     g: ((color_u32 >> 8) & 0xFF) as u8,
//             //     b: ((color_u32 >> 16) & 0xFF) as u8,
//             // })
//         });

//         // Always release the DC
//         ReleaseDC(desktop_hwnd, hdc);

//         match result {
//             Ok(Ok(color)) => color,
//             Ok(Err(_e)) => 0,
//             Err(_) => 0,
//         }
//     }
// }
// PLEASE HELP WHAT DOES THIS FUNCTION DO??

// #[cfg(target_os = "windows")]
// fn optimize_large_window(window: &winit::window::Window) {
//     use winapi::um::winuser::{
//         SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetWindowPos,
//     };

//     if let raw_window_handle::RawWindowHandle::Win32(handle) =
//         window.raw_window_handle()
//     {
//         let hwnd = handle.hwnd as winapi::shared::windef::HWND;

//         unsafe {
//             // Force window to refresh its layered window properties
//             SetWindowPos(
//                 hwnd,
//                 core::ptr::null_mut(),
//                 0,
//                 0,
//                 0,
//                 0,
//                 SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
//             );
//         }
//     }
// }
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_BOTTOM, HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE,
};

/// Set the [`WindowRenderLayer`] of a window
pub fn set_window_level_raw(hwnd: windows::Win32::Foundation::HWND, level: WindowRenderLayer) {
    let insert_after = match level {
        WindowRenderLayer::AlwaysOnBottom => HWND_BOTTOM,
        WindowRenderLayer::Normal => HWND_NOTOPMOST,
        WindowRenderLayer::AlwaysOnTop => HWND_TOPMOST,
    };

    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            insert_after,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE,
        );
    }
}

use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
use windows_sys::Win32::{
    Foundation::{HWND, LPARAM},
    UI::WindowsAndMessaging::{EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible},
};
#[allow(missing_docs)]
/// A data struct temporarily used when searching for a window
///
/// Not intended to use used or accessed
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct WindowSearchData {
    pub target_title: String,
    pub found_hwnds: Option<Vec<HWND>>,
    pub exact_match: bool,
    pub case_sensitive: bool,
    pub include_hidden: bool,
    pub just_one: bool,
}
/// I don't remember what this does
///
/// # Safety
/// Unknown. Best not to use.
pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
    let search_data = &mut *(lparam as *mut WindowSearchData);

    if !search_data.include_hidden && IsWindowVisible(hwnd) == 0 {
        return 1;
    }

    let len = GetWindowTextLengthW(hwnd);
    if len <= 0 {
        return 1;
    }

    let mut buffer = vec![0u16; (len + 1) as usize];
    let read = GetWindowTextW(hwnd, buffer.as_mut_ptr(), len + 1);

    if read <= 0 {
        return 1;
    }

    let mut window_title = String::from_utf16_lossy(&buffer[..read as usize]);
    let mut target_title = search_data.target_title.clone();

    if !search_data.case_sensitive {
        window_title = window_title.to_lowercase();
        target_title = target_title.to_lowercase();
    }

    let matches = if search_data.exact_match {
        window_title == target_title
    } else {
        window_title.contains(&target_title)
    };
    if hwnd as isize == 0 {
        return 1;
    }

    if matches {
        search_data
            .found_hwnds
            .get_or_insert_with(Vec::new)
            .push(hwnd);

        if search_data.just_one {
            return 0;
        }
    }

    1
}
/// Get a window hwnd by their title
pub fn get_window_id_by_title(
    title: &str,
    exact_match: bool,
    case_sensitive: bool,
    include_hidden: bool,
    just_one: bool,
) -> Option<Vec<RawWindowHandle>> {
    let mut search_data = WindowSearchData {
        target_title: title.to_string(),
        found_hwnds: None,
        exact_match,
        case_sensitive,
        include_hidden,
        just_one,
    };

    unsafe {
        EnumWindows(Some(enum_windows_proc), &raw mut search_data as isize);
    }

    search_data.found_hwnds.map(|hwnds| {
        hwnds
            .into_iter()
            .map(|hwnd| {
                RawWindowHandle::Win32(Win32WindowHandle::new(unsafe {
                    core::num::NonZeroIsize::new_unchecked(hwnd as isize)
                }))
            })
            .collect()
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_notepad() {
        if let Some(handle) = get_window_id_by_title("Notepad", false, true, false, false) {
            println!("Found Notepad window: {handle:?}");
        } else {
            println!("Notepad window not found");
        }
    }
}

// unsafe extern "system" fn enum_windows_proc(
//     hwnd: winapi::shared::windef::HWND,
//     long_param: isize,
// ) -> i32 {
//     let data = &mut *(long_param as *mut WindowSearchData);

//     // Check visibility based on criteria
//     if !data.include_hidden && IsWindowVisible(hwnd) == 0 {
//         return 1; // Continue enumeration
//     }
//     if !data.target_title.is_empty() {
//         // Get window title
//         let mut title_buf = [0u16; 512];
//         let title_len = GetWindowTextW(
//             hwnd,
//             title_buf.as_mut_ptr(),
//             title_buf.len() as i32,
//         );

//         let title = if title_len > 0 {
//             String::from_utf16_lossy(&title_buf[..title_len as usize])
//         } else {
//             String::new()
//         };

//         // Check title match
//         let title_matches = if data.exact_match {
//             if data.case_sensitive {
//                 title == data.target_title
//             } else {
//                 title.to_lowercase() == data.target_title.to_lowercase()
//             }
//         } else if data.case_sensitive {
//             title.contains(&data.target_title)
//         } else {
//             title.to_lowercase().contains(&data.target_title.to_lowercase())
//         };

//         if !title_matches {
//             return 1; // Continue enumeration
//         }
//     }

//     data.found_hwnds =
//         Some(data.found_hwnds.clone().unwrap_or_default().combined(hwnd));
//     // 0 Stop enumeration
//     // 1 Continue enumeration

//     i32::from(!data.just_one)
// }

/// Get all existing windows
#[allow(trivial_casts)]
pub fn get_all_windows_raw() -> Vec<windows::Win32::Foundation::HWND> {
    let mut search_data = WindowSearchData {
        target_title: String::new(),
        found_hwnds: None,
        exact_match: false,
        case_sensitive: false,
        include_hidden: false,
        just_one: false,
    };

    unsafe {
        EnumWindows(Some(enum_windows_proc), &raw mut search_data as isize);
    }
    let mut found_windows = Vec::new();
    for i in search_data.found_hwnds.unwrap_or_default() {
        found_windows.push(windows::Win32::Foundation::HWND(i as isize));
    }

    found_windows
}
#[must_use]
/// get the "hitbox" of a window
pub fn get_window_hitbox_size_raw(hwnd: windows::Win32::Foundation::HWND) -> Option<(i32, i32)> {
    unsafe {
        let mut rect = windows::Win32::Foundation::RECT::default();
        if GetWindowRect(hwnd, &raw mut rect).as_bool() {
            Some((rect.right - rect.left, rect.bottom - rect.top))
        } else {
            None
        }
    }
}
#[must_use]
/// Get the size of a window
///
/// Warning: Returns (0, 0) for windows that don't exist
pub fn get_window_size_raw(hwnd: windows::Win32::Foundation::HWND) -> (i32, i32) {
    unsafe {
        let mut rect = windows::Win32::Foundation::RECT::default();
        GetClientRect(hwnd, &raw mut rect);
        (rect.right - rect.left, rect.bottom - rect.top)
    }
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Get the window title using the hwnd
pub fn get_title_using_id_raw(hwnd: windows_sys::Win32::Foundation::HWND) -> String {
    let mut title_buf = [0u16; 512];
    let title_len;
    unsafe {
        title_len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32);
    }

    if title_len > 0 {
        String::from_utf16_lossy(&title_buf[..title_len as usize])
    } else {
        String::new()
    }
}

use windows::Win32::{
    System::Threading::{
        ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS,
        IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, OpenProcess, OpenThread,
        PROCESS_SET_INFORMATION, REALTIME_PRIORITY_CLASS, SetPriorityClass, SetThreadPriority,
        THREAD_PRIORITY_ABOVE_NORMAL, THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_HIGHEST,
        THREAD_PRIORITY_IDLE, THREAD_PRIORITY_NORMAL, THREAD_PRIORITY_TIME_CRITICAL,
        THREAD_SET_INFORMATION,
    },
    UI::WindowsAndMessaging::GetWindowThreadProcessId,
};

/// Set the priority of a running process
pub fn set_cpu_priority(hwnd: windows::Win32::Foundation::HWND, priority: ProcessCpuPriority) {
    unsafe {
        let mut process_id = 0u32;
        let thread_id = GetWindowThreadProcessId(hwnd, Some(&raw mut process_id));

        if thread_id == 0 {
            return; // Failed to get thread ID
        }

        // Open the process with permission to set priority
        let Ok(process) = OpenProcess(PROCESS_SET_INFORMATION, false, process_id) else {
            return;
        };

        // Open the thread with permission to set priority
        let Ok(thread) = OpenThread(THREAD_SET_INFORMATION, false, thread_id) else {
            return;
        };

        match priority {
            ProcessCpuPriority::Idle => {
                let _ = SetPriorityClass(process, IDLE_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_IDLE);
            }
            ProcessCpuPriority::BelowNormal => {
                let _ = SetPriorityClass(process, BELOW_NORMAL_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_BELOW_NORMAL);
            }
            ProcessCpuPriority::Normal => {
                let _ = SetPriorityClass(process, NORMAL_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_NORMAL);
            }
            ProcessCpuPriority::AboveNormal => {
                let _ = SetPriorityClass(process, ABOVE_NORMAL_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_ABOVE_NORMAL);
            }
            ProcessCpuPriority::High => {
                let _ = SetPriorityClass(process, HIGH_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_HIGHEST);
            }
            ProcessCpuPriority::Realtime => {
                let _ = SetPriorityClass(process, REALTIME_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_TIME_CRITICAL);
            }
        }
    }
}

// use windows::Win32::UI::WindowsAndMessaging::{FLASHWINFO, FLASHW_ALL, FLASHW_TIMERNOFG, FlashWindowEx};

// pub fn flash_window(hwnd: HWND) {
//     unsafe {
//         let info = FLASHWINFO {
//             cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
//             hwnd,
//             dwFlags: FLASHW_ALL | FLASHW_TIMERNOFG,
//             uCount: 0,
//             dwTimeout: 0,
//         };
//         FlashWindowEx(&info);
//     }
// }

impl TaskBar for WindowsActions {
    fn set_icon_state(handle: &raw_window_handle::RawWindowHandle, state: &TaskbarLoadingState) {
        if let raw_window_handle::RawWindowHandle::Win32(raw) = handle {
            let _ =
                set_taskbar_progress_state(windows::Win32::Foundation::HWND(raw.hwnd.get()), state);
        }
    }
    fn set_icon_progress(handle: &raw_window_handle::RawWindowHandle, current: u64, total: u64) {
        if let raw_window_handle::RawWindowHandle::Win32(raw) = handle {
            let _ = set_taskbar_progress_value(
                windows::Win32::Foundation::HWND(raw.hwnd.get()),
                current,
                total,
            );
        }
    }
}

use windows::Win32::{
    System::Com::{CLSCTX_ALL, CoCreateInstance},
    UI::Shell::{
        ITaskbarList3, TBPF_ERROR, TBPF_INDETERMINATE, TBPF_NOPROGRESS, TBPF_NORMAL, TBPF_PAUSED,
        TBPFLAG, TaskbarList,
    },
};
/// Convert between `TaskbarLoadingState` and TBPFLAG
#[must_use]
pub const fn map_state_to_windows_state(state: &TaskbarLoadingState) -> TBPFLAG {
    match state {
        TaskbarLoadingState::NoBar => TBPF_NOPROGRESS,
        TaskbarLoadingState::Error => TBPF_ERROR,
        TaskbarLoadingState::Loading => TBPF_INDETERMINATE,
        TaskbarLoadingState::Normal => TBPF_NORMAL,
        TaskbarLoadingState::Paused => TBPF_PAUSED,
    }
}
/// Set how the taskbar icon looks
///
/// # Errors
/// When unable to set the progress
pub fn set_taskbar_progress_state(
    hwnd: windows::Win32::Foundation::HWND,
    state: &TaskbarLoadingState,
) -> windows::core::Result<()> {
    unsafe {
        let taskbar: ITaskbarList3 = CoCreateInstance(&TaskbarList, None, CLSCTX_ALL)?;
        taskbar.HrInit()?;
        taskbar.SetProgressState(hwnd, map_state_to_windows_state(state))?;
    }
    Ok(())
}
/// Set the progress of a few styles
///
/// # Errors
/// When the taskbar could not be referenced
/// When setting the value didn't work
pub fn set_taskbar_progress_value(
    hwnd: windows::Win32::Foundation::HWND,
    completed: u64,
    total: u64,
) -> windows::core::Result<()> {
    unsafe {
        let taskbar: ITaskbarList3 = CoCreateInstance(&TaskbarList, None, CLSCTX_ALL)?;
        taskbar.HrInit()?;
        taskbar.SetProgressValue(hwnd, completed, total)?;
    }
    Ok(())
}
#[must_use]
/// Get the resolution of the screen
///
/// No clue what happens when there are multiple monitors
#[track_caller]
pub fn get_screen_resolution() -> (i32, i32) {
    // use winapi::um::winuser::GetSystemMetrics;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
    let width =
        unsafe { GetSystemMetrics(windows_sys::Win32::UI::WindowsAndMessaging::SM_CXSCREEN) };
    let height =
        unsafe { GetSystemMetrics(windows_sys::Win32::UI::WindowsAndMessaging::SM_CYSCREEN) };
    (width, height)
}
// impl Host for WindowsActions {
//     fn get_os_name() -> String {
//         "Windows".to_string()
//     }
// }
impl Screen for WindowsActions {
    fn get_screen_resolution() -> (i32, i32) {
        get_screen_resolution()
    }
}

// use winapi::um::winuser::{
//     // SWP_NOSIZE,
//     // SWP_SHOWWINDOW,
//     SW_MAXIMIZE,
//     SW_MINIMIZE,
//     SW_RESTORE,
//     ShowWindow, //HWND_NOTOPMOST, HWND_TOPMOST,
// };

use windows_sys::Win32::UI::WindowsAndMessaging::{
    SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, ShowWindow,
};
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Maximize the given window -> Expand its borders to fit the screen
pub fn maximize(hwnd: windows_sys::Win32::Foundation::HWND) {
    unsafe {
        ShowWindow(hwnd, SW_MAXIMIZE);
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Hide away the given window
pub fn minimize(hwnd: windows_sys::Win32::Foundation::HWND) {
    unsafe {
        ShowWindow(hwnd, SW_MINIMIZE);
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Unhide un-away the given window
pub fn restore(hwnd: windows_sys::Win32::Foundation::HWND) {
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }
}
// pub fn always_on_top(window: &*mut c_void, always_on_top_bool: bool) {
//     if always_on_top_bool {
//         topmost(window);
//     } else {
//         not_topmost(window);
//     }
// }

// pub fn topmost(window: &*mut c_void) {
//     let hwnd = *window as HWND;
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             HWND_TOPMOST,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
//         );
//     }
// }
// pub fn not_topmost(window: &*mut c_void) {
//     let hwnd = *window as HWND;
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             HWND_NOTOPMOST,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
//         );
//     }
// }

/// Resize the current window to the specified size
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn resize(hwnd: windows_sys::Win32::Foundation::HWND, size: (i32, i32)) {
    unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::SetWindowPos(
            hwnd,
            core::ptr::null_mut(),
            0,
            0,
            size.0,
            size.1,
            windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOZORDER
                | windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOMOVE,
        );
    }
}
/// Wether the window is minimized on windows
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn is_window_minimized(hwnd: windows_sys::Win32::Foundation::HWND) -> bool {
    unsafe { windows_sys::Win32::UI::WindowsAndMessaging::IsIconic(hwnd) != 0 }
}
/// Wether a window is maximized on windows
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn is_window_maximized(hwnd: windows_sys::Win32::Foundation::HWND) -> bool {
    unsafe { windows_sys::Win32::UI::WindowsAndMessaging::IsZoomed(hwnd) != 0 }
}
impl Iconized for WindowsActions {
    fn is_minimized(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            is_window_minimized(handle.hwnd.get() as *mut std::ffi::c_void);
            true
        } else {
            false
        }
    }

    fn is_maximized(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            is_window_maximized(handle.hwnd.get() as *mut std::ffi::c_void);
            true
        } else {
            false
        }
    }

    fn restore(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            restore(handle.hwnd.get() as *mut std::ffi::c_void);
            true
        } else {
            false
        }
    }

    fn minimize(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            minimize(handle.hwnd.get() as *mut std::ffi::c_void);
            true
        } else {
            false
        }
    }

    fn maximize(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            maximize(handle.hwnd.get() as *mut std::ffi::c_void);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A wrapper making sure that the Registry key will be closed once the struct goes out of scope
pub struct RegKeyGuard(pub windows::Win32::System::Registry::HKEY);
impl Drop for RegKeyGuard {
    fn drop(&mut self) {
        unsafe { windows::Win32::System::Registry::RegCloseKey(self.0) };
    }
}

#[cfg(feature = "font_support")]
impl SystemFontProvider for WindowsActions {
    fn get_system_font_file(
        &self,
    ) -> std::result::Result<mirl_collections::BinaryData, Box<dyn std::error::Error>> {
        let path = get_font_path()?;
        Ok(mirl_collections::BinaryData::from_bytes(
            std::fs::read(path)?,
            mirl_collections::GenericDataType::Font,
        ))
    }
}
/// Get the font the os is using
///
/// # Errors
/// When no font can be found
pub fn get_font_path() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let face = unsafe {
        let mut metrics = windows::Win32::UI::WindowsAndMessaging::NONCLIENTMETRICSW {
            cbSize: std::mem::size_of::<windows::Win32::UI::WindowsAndMessaging::NONCLIENTMETRICSW>(
            ) as u32,
            ..Default::default()
        };

        let result = windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
            windows::Win32::UI::WindowsAndMessaging::SPI_GETNONCLIENTMETRICS,
            metrics.cbSize,
            Some((&raw mut metrics).cast()),
            windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );

        if result == false {
            return Err("SystemParametersInfoW failed".into());
        }

        String::from_utf16_lossy(
            &metrics
                .lfMessageFont
                .lfFaceName
                .iter()
                .take_while(|&&c| c != 0)
                .copied()
                .collect::<Vec<_>>(),
        )
    };

    let key = unsafe {
        let mut key = windows::Win32::System::Registry::HKEY::default();
        if windows::Win32::System::Registry::RegOpenKeyExW(
            windows::Win32::System::Registry::HKEY_LOCAL_MACHINE,
            windows::core::w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts"),
            0,
            windows::Win32::System::Registry::KEY_READ,
            &raw mut key,
        ) != windows::Win32::Foundation::ERROR_SUCCESS
        {
            return Err("Failed to open registry".into());
        }
        key
    };
    let _key_guard = RegKeyGuard(key);

    let fonts_dir = unsafe {
        let path_ptr = windows::Win32::UI::Shell::SHGetKnownFolderPath(
            &windows::Win32::UI::Shell::FOLDERID_Fonts,
            windows::Win32::UI::Shell::KNOWN_FOLDER_FLAG(0),
            None,
        )?;
        let path = path_ptr.to_string()?;
        windows::Win32::System::Com::CoTaskMemFree(Some(path_ptr.0.cast()));
        path
    };

    let mut index = 0u32;
    loop {
        let mut name_buf = [0u16; 256];
        let mut name_len = name_buf.len() as u32;
        let mut data_buf = [0u16; 256];
        let mut data_len = (data_buf.len() * 2) as u32;

        let result = unsafe {
            windows::Win32::System::Registry::RegEnumValueW(
                key,
                index,
                windows::core::PWSTR(name_buf.as_mut_ptr()),
                &raw mut name_len,
                None,
                None,
                Some(data_buf.as_mut_ptr().cast::<u8>()),
                Some(&raw mut data_len),
            )
        };

        if result.is_err() {
            break;
        }

        let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
        let entry_face = name
            .trim_end_matches(" (TrueType)")
            .trim_end_matches(" (OpenType)");

        if entry_face == face {
            let file = String::from_utf16_lossy(
                &data_buf
                    .iter()
                    .take_while(|&&c| c != 0)
                    .copied()
                    .collect::<Vec<_>>(),
            );

            // Some registry entries already contain an absolute path.
            let path = if file.contains('\\') || file.contains('/') {
                file
            } else {
                format!("{fonts_dir}\\{file}")
            };

            return Ok(path);
        }

        index += 1;
    }

    Err("font not found".into())
}

/// Execute a program with admin (elevated) privileges on Windows.
/// If permission wasn't given, the given program will still be executed.
///
/// # Arguments
/// * `program_path` - Path to the executable to run
/// * `args` - Command-line arguments to pass to the program
///
/// # Returns
/// * `Ok(())` if elevation was requested
/// * `Err(String)` if there was an error preparing the elevation request
///
/// # Errors
/// When the given path isn't utf-8
/// When the executable doesn't exist
pub fn execute_elevated(
    program_path: &std::path::Path,
    args: &[&str],
) -> std::result::Result<(), String> {
    use windows_sys::Win32::{
        Foundation::FALSE,
        System::Threading::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW},
    };

    if !program_path.exists() {
        return Err(format!("Program not found: {}", program_path.display()));
    }

    let Some(mut wide_exe): Option<Vec<u16>> = path_to_wide(program_path) else {
        return Err("Path is not valid utf8".to_string());
    };

    // Safety: Since path_to_wide would reject any path that isn't utf8, this is assured to be valid utf8
    let cmd_line = crate::action::build_command_line(
        unsafe { program_path.to_str().unwrap_unchecked() },
        args,
    );
    let mut wide_cmd: Vec<u16> = cmd_line.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut startup_info: STARTUPINFOW = std::mem::zeroed();
        let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

        startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

        let success = CreateProcessW(
            wide_exe.as_mut_ptr(),
            wide_cmd.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            FALSE,
            0,
            std::ptr::null_mut(),
            std::ptr::null(),
            &raw const startup_info,
            &raw mut process_info,
        );

        if success == 0 {
            return Err("Failed to create elevated process".to_string());
        }

        // Clean up handles
        if !process_info.hProcess.is_null() {
            windows_sys::Win32::Foundation::CloseHandle(process_info.hProcess);
        }
        if !process_info.hThread.is_null() {
            windows_sys::Win32::Foundation::CloseHandle(process_info.hThread);
        }
    }

    Ok(())
}
/// Convert a path to the wide path format windows uses
#[must_use]
/// Convert a Path to a null-terminated wide string suitable for Windows API
pub fn path_to_wide(path: &std::path::Path) -> Option<Vec<u16>> {
    let path_str = path.to_str()?;

    Some(string_to_wide(path_str))
}

#[must_use]
/// Convert a String/&str to a null-terminated wide string for Windows API
pub fn string_to_wide(s: &str) -> std::vec::Vec<u16> {
    use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
use windows_sys::Win32::{
    Foundation::CloseHandle,
    Security::{GetTokenInformation, TOKEN_ELEVATION, TokenElevation},
};
#[must_use]
/// Test if the given process if elevated
pub fn is_elevated(hwnd: windows_sys::Win32::Foundation::HWND) -> bool {
    unsafe {
        let handle = hwnd.cast::<std::ffi::c_void>();

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };

        let mut ret_size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            handle,
            TokenElevation,
            (&raw mut elevation).cast(),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &raw mut ret_size,
        ) != 0;

        CloseHandle(handle);

        result && elevation.TokenIsElevated != 0
    }
}

impl ElevatedExecution for WindowsActions {
    fn execute_in_elevated_mode(
        path: &std::path::Path,
        args: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        execute_elevated(path, args).map_err(Box::<dyn std::error::Error>::from)?;
        Ok(())
    }
    fn is_in_elevated_mode(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match handle {
            #[cfg(target_os = "windows")]
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                Ok(is_elevated(handle.hwnd.get() as *mut std::ffi::c_void))
            }
            _ => Err("Wrong operating system".into()),
        }
    }
}
