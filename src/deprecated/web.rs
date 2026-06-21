// use wasm_bindgen::prelude::*;
// use crate::extensions::*;

// use super::{Info, Screen};
// /// Web/Wasm compatible struct to get environment information
// #[cfg_attr(feature = "c_compatible", repr(C))]
pub struct WebInfo {}
// impl WebInfo {
//     /// Create a new [WebInfo] instance
//     pub fn new() -> Self {
//         WebInfo {}
//     }
// }

// impl Info for WebInfo {
//     fn get_os_name() -> String {
//         "wasm".into()
//     }
// }

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "screen"], js_name = width)]
//     fn screen_width() -> u32;

//     #[wasm_bindgen(js_namespace = ["window", "screen"], js_name = height)]
//     fn screen_height() -> u32;
// }

// #[wasm_bindgen]
// pub fn get_screen_width() -> i32 {
//     screen_width() as i32
// }

// #[wasm_bindgen]
// pub fn get_screen_height() -> i32 {
//     screen_height() as i32
// }

// fn get_screen_resolution_raw() -> (i32, i32) {
//     (screen_width(), screen_height()).try_tuple_into()
// }

// impl Screen for WebInfo {
//     fn get_os_menu_height() -> i32 {
//         0
//     }
//     fn get_taskbar_height() -> i32 {
//         0
//     }
//     fn get_screen_resolution() -> (i32, i32) {
//         get_screen_resolution_raw()
//     }
// }
