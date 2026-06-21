// use winapi::um::winuser::GetSystemMetrics;

// use super::{Battery, Info, Memory, Screen};

// /// Windows struct implementation of the `OSInfo` trait
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "c_compatible", repr(C))]
// pub struct WindowsInfo {
//     //system: sysinfo::System,
// }
// impl Default for WindowsInfo {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl WindowsInfo {
//     /// Create a new [`WindowsInfo`] instance
//     #[must_use]
//     pub const fn new() -> Self {
//         Self {
//             //system: sysinfo::System::new(),
//         }
//     }
// }

// impl Info for WindowsInfo {
//     fn get_os_name() -> String {
//         "windows".into()
//     }
// }
// impl Memory for WindowsInfo {
//     #[allow(clippy::field_reassign_with_default)]
//     fn get_total_memory(&self) -> u64 {
//         unsafe {
//             let mut mem_status = windows::Win32::System::SystemInformation::MEMORYSTATUSEX::default();
//             mem_status.dwLength = std::mem::size_of::<
//                 windows::Win32::System::SystemInformation::MEMORYSTATUSEX,
//             >() as u32;

//             if windows::Win32::System::SystemInformation::GlobalMemoryStatusEx(
//                 &raw mut mem_status,
//             )
//             .as_bool()
//             {
//                 mem_status.ullTotalPhys
//             } else {
//                 0
//             }
//         }
//     }
//     #[allow(clippy::field_reassign_with_default)]
//     fn get_free_memory(&self) -> u64 {
//         unsafe {
//             let mut mem_status = windows::Win32::System::SystemInformation::MEMORYSTATUSEX::default();
//             mem_status.dwLength = std::mem::size_of::<
//                 windows::Win32::System::SystemInformation::MEMORYSTATUSEX,
//             >() as u32;

//             if windows::Win32::System::SystemInformation::GlobalMemoryStatusEx(
//                 &raw mut mem_status,
//             )
//             .as_bool()
//             {
//                 mem_status.ullAvailPhys
//             } else {
//                 0
//             }
//         }
//     }
// }
// impl Screen for WindowsInfo {
//     // fn get_os_menu_height() -> i32 {
//     //     unsafe { GetSystemMetrics(4) }
//     // }

//     fn get_taskbar_height() -> i32 {
//         unsafe {
//             let mut abd: winapi::um::shellapi::APPBARDATA = std::mem::zeroed();
//             abd.cbSize =
//                 std::mem::size_of::<winapi::um::shellapi::APPBARDATA>() as u32;
//             abd.hWnd = winapi::um::winuser::GetDesktopWindow();

//             if winapi::um::shellapi::SHAppBarMessage(
//                 winapi::um::shellapi::ABM_GETTASKBARPOS,
//                 &raw mut abd,
//             ) != 0
//             {
//                 abd.rc.bottom - abd.rc.top
//             } else {
//                 0 // Failed to get taskbar info
//             }
//         }
//     }
//     fn get_screen_resolution() -> (i32, i32) {
//         let width =
//             unsafe { GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) };
//         let height =
//             unsafe { GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) };
//         (width, height)
//     }
// }
// use windows::Win32::System::Power::{
//     GetSystemPowerStatus, SYSTEM_POWER_STATUS,
// };

// impl Battery for WindowsInfo {
//     fn get_battery_percentage(&self) -> Option<u8> {
//         let mut power_status = SYSTEM_POWER_STATUS::default();

//         unsafe {
//             GetSystemPowerStatus(&raw mut power_status);
//         }

//         if power_status.BatteryLifePercent == 255 {
//             None
//         } else {
//             Some(power_status.BatteryLifePercent)
//         }
//     }
//     fn is_in_low_power_mode() -> bool {
//         false
//     }
//     fn is_battery_charging() -> bool {
//         false
//     }
// }
