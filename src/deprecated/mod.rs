// /// Basic OS info
// #[const_trait]
// pub const trait Info {
//     /// The name of the os
//     fn get_os_name() -> String;
// }
// #[const_trait]
// /// Get memory related info
// pub const trait Memory {
//     /// The total amount of memory
//     fn get_total_memory(&self) -> u64;
//     /// The currently remaining amount of memory
//     fn get_free_memory(&self) -> u64;
// }
// #[const_trait]
// /// Basic screen information
// pub const trait Screen {
//     // /// The os menu on top of windows
//     // fn get_os_menu_height() -> i32;
//     /// Screen resolution
//     fn get_screen_resolution() -> (i32, i32);
//     /// Height of task bar
//     fn get_taskbar_height() -> i32;
// }
// #[const_trait]
// /// Basic Battery information
// pub const trait Battery {
//     /// From 0 to 100
//     fn get_battery_percentage(&self) -> Option<u8>;
//     /// If the battery is currently charging
//     fn is_battery_charging() -> bool;
//     /// If the os is in low power mode
//     fn is_in_low_power_mode() -> bool;
// }
// #[const_trait]
// /// Basic Clipboard operations
// // pub const trait Clipboard {
// //     fn get_clipboard() -> ClipboardContents;
// //     fn set_clipboard();
// // }
// /// Basic time information (To get the current time, use std)
// pub const trait Time {
//     /// Get the timezone offset -> How many hours over/under the standart time
//     fn get_timezone_offset() -> i8;
// }
// #[const_trait]
// /// Basic network information
// pub const trait Network {
//     /// Tries to open a connection to the website, Default by None: <http://example.com>
//     fn is_connected_to_internet(website_connection: Option<String>) -> bool;
// }

// // use crate::graphics::Buffer;
