#![allow(unused)]
// //extern crate image;
// extern crate x11;

// //use core::ptr;

// use image::{Rgba, RgbaImage};
// use x11::xlib::{
//     Display, Window, XDefaultRootWindow, XDestroyImage, XGetGeometry,
//     XGetImage, XGetWindowAttributes, XOpenDisplay,
// };

// pub fn get_title_bar_height() -> i32 {
//     unsafe {
//         let display: *mut Display = XOpenDisplay(core::ptr::null());
//         if display.is_null() {
//             panic!("Unable to open X display");
//         }

//         let root_window: Window = XDefaultRootWindow(display);
//         let mut attributes = std::mem::zeroed();
//         if XGetWindowAttributes(display, root_window, &mut attributes) != 0 {
//             panic!("Failed to get window attributes");
//         }

//         // Here, attributes.y gives the window's title bar height
//         attributes.y
//     }
// }

// pub fn get_screen_resolution() -> (i32, i32) {
//     unsafe {
//         let display = XOpenDisplay(null_mut());
//         if display.is_null() {
//             panic!("Unable to open X display");
//         }

//         let screen = XScreenOfDisplay(display, 0);
//         let width = XDisplayWidth(display, 0);
//         let height = XDisplayHeight(display, 0);

//         (width, height)
//     }
// }

// pub fn capture_screen() -> Option<ScreenCapture> {
//     unsafe {
//         // Open the display
//         let display: *mut Display = XOpenDisplay(ptr::null());
//         if display.is_null() {
//             return None;
//         }

//         // Get the root window
//         let root_window: Window = XDefaultRootWindow(display);

//         // Get the window's geometry (screen dimensions)
//         let mut width = 0;
//         let mut height = 0;
//         let mut x = 0;
//         let mut y = 0;
//         let mut border_width = 0;
//         let mut depth = 0;
//         if XGetGeometry(
//             display,
//             root_window,
//             ptr::null_mut(),
//             &mut x,
//             &mut y,
//             &mut width,
//             &mut height,
//             &mut border_width,
//             &mut depth,
//         ) == 0
//         {
//             return None;
//         }

//         // Get the screen image
//         let image = XGetImage(
//             display,
//             root_window,
//             0,
//             0,
//             width as u32,
//             height as u32,
//             !0,
//             2,
//         );
//         if image.is_null() {
//             return None;
//         }

//         // Allocate a vector to store pixel data
//         let mut pixels: Vec<u32> =
//             Vec::with_capacity((width * height) as usize);

//         // Process the image and store the pixels in a format suitable for an RGBA image
//         for y in 0..height {
//             for x in 0..width {
//                 let pixel = XGetImage(
//                     display,
//                     root_window,
//                     x as u32,
//                     y as u32,
//                     1,
//                     1,
//                     !0,
//                     2,
//                 );
//                 if !pixel.is_null() {
//                     let color = *((*pixel).data.offset(0)) as u32;
//                     pixels.push(color);
//                 }
//             }
//         }

//         // Clean up and close the display
//         XDestroyImage(image);

//         Some(ScreenCapture {
//             width,
//             height,
//             pixels,
//         })
//     }
// }

// pub fn capture_desktop_background() -> Option<ScreenImage> {
//     unsafe {
//         // Open the X display
//         let display: *mut Display = XOpenDisplay(ptr::null());
//         if display.is_null() {
//             return None;
//         }

//         // Get the root window (the desktop background)
//         let root_window: Window = XDefaultRootWindow(display);

//         // Get the root window's geometry (screen dimensions)
//         let mut width = 0;
//         let mut height = 0;
//         let mut x = 0;
//         let mut y = 0;
//         let mut border_width = 0;
//         let mut depth = 0;
//         if XGetGeometry(
//             display,
//             root_window,
//             ptr::null_mut(),
//             &mut x,
//             &mut y,
//             &mut width,
//             &mut height,
//             &mut border_width,
//             &mut depth,
//         ) == 0
//         {
//             return None;
//         }

//         // Get the screen image (background image)
//         let image = XGetImage(
//             display,
//             root_window,
//             0,
//             0,
//             width as u32,
//             height as u32,
//             !0,
//             2,
//         );
//         if image.is_null() {
//             return None;
//         }

//         // Allocate a vector to store pixel data
//         let mut pixels: Vec<u32> =
//             Vec::with_capacity((width * height) as usize);

//         // Process the image and store the pixels in a format suitable for an RGBA image
//         for y in 0..height {
//             for x in 0..width {
//                 let pixel = XGetImage(
//                     display,
//                     root_window,
//                     x as u32,
//                     y as u32,
//                     1,
//                     1,
//                     !0,
//                     2,
//                 );
//                 if !pixel.is_null() {
//                     let color = *((*pixel).data.offset(0)) as u32;
//                     pixels.push(color);
//                 }
//             }
//         }

//         // Clean up and close the display
//         XDestroyImage(image);

//         Some(ScreenImage {
//             width,
//             height,
//             pixels,
//         })
//     }
// }

use std::{ffi::CString, ptr};

use x11::xlib::*;

use crate::system::action::{
    CpuPriority, Decoration, Default, Host, LoadingState, Misc, Screen,
    TaskBar, Transparency,
};
/// `OsImplementation` for Linux
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct LinuxActions {}

impl Screen for LinuxActions {
    fn get_screen_resolution() -> (i32, i32) {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return (0, 0);
            }

            let screen = XDefaultScreen(display);
            let width = XDisplayWidth(display, screen);
            let height = XDisplayHeight(display, screen);

            XCloseDisplay(display);
            (width, height)
        }
    }
}
