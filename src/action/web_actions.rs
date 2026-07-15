//use wasm_bindgen::prelude::*;
use web_sys::window;

use crate::system::action::Screen;

/// A struct holding information about the underlying os
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct WebActions;

impl Screen for WebActions {
    fn get_screen_resolution() -> (i32, i32) {
        let window = window().expect("no global `window` exists");

        let width = window
            .screen_x()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as i32;
        let height = window
            .screen_y()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as i32;

        (width, height)
    }
}
