#[cfg(not(feature = "miri"))]
#[cfg(target_os = "windows")]
/// An OS specific library, only use when necessary
pub mod windows_actions;

#[cfg(target_os = "windows")]
#[cfg(not(feature = "miri"))]
pub use windows_actions::WindowsActions as Os;

#[cfg(target_os = "linux")]
#[cfg(not(feature = "miri"))]
/// An OS specific library, only use when necessary
pub mod linux_actions;
#[cfg(target_os = "linux")]
#[cfg(not(feature = "miri"))]
pub use linux_actions::LinuxActions as Os;

#[cfg(target_arch = "wasm32")]
#[cfg(not(feature = "miri"))]
/// An OS specific library, only use when necessary
pub mod web_actions;
#[cfg(target_arch = "wasm32")]
#[cfg(not(feature = "miri"))]
pub use web_actions::WebActions as Os;

#[cfg(target_os = "macos")]
#[cfg(not(feature = "miri"))]
/// An OS specific library, only use when necessary
pub mod mac_actions;
#[cfg(target_os = "macos")]
#[cfg(not(feature = "miri"))]
pub use mac_actions::MacActions as Os;

#[cfg(feature = "miri")]
/// An OS specific library, only use when necessary
pub mod miri_actions;
#[cfg(feature = "miri")]
pub use miri_actions::MiriActions as Os;

#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "linux",
    target_os = "windows",
    target_os = "macos",
    feature = "miri"
)))]
/// A last resort, not to be used by any devs but the Os struct import from super::
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct Os {}
// #[cfg(target_arch = "wasm32")]
// /// An OS specific library, only use when necessary
// pub mod web_actions;
// #[cfg(target_arch = "wasm32")]
// use web_actions::{
//     capture_desktop_background_raw, capture_screen_raw, get_window_id_by_title,
// };
// use crate::{platform::WindowLevel, prelude::Buffer};
// #[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
// #[cfg_attr(feature = "wincode", derive(wincode::SchemaWrite, wincode::SchemaRead))]

#[must_use]
/// Build a properly quoted command line from a program and its arguments
///
/// # Errors (None)
/// When unable to convert the program path into str
pub fn build_command_line(program_str: &str, args: &[&str]) -> String {
    let mut cmd_parts = Vec::new();

    cmd_parts.push(quote_arg(program_str));

    for arg in args {
        cmd_parts.push(quote_arg(arg));
    }

    cmd_parts.join(" ")
}
#[must_use]
/// Quote a command-line argument if necessary (contains spaces or special chars)
pub fn quote_arg(arg: &str) -> String {
    if arg.contains(' ') || arg.contains('"') || arg.contains('\t') {
        let escaped = arg.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        arg.to_string()
    }
}

// Example usage:
#[cfg(all(windows, test))]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_quote_arg() {
        assert_eq!(quote_arg("simple"), "simple");
        assert_eq!(quote_arg("with space"), "\"with space\"");
        assert_eq!(quote_arg("with\"quote"), "\"with\"\"quote\"");
    }

    #[test]
    fn test_build_command_line() {
        let path = PathBuf::from(r"C:\Program Files\app.exe");
        let args = &["arg1", "arg with spaces", "C:\\path\\to\\file"];

        let cmd = build_command_line(path.to_str().unwrap(), args);
        println!("Command line: {cmd}");

        assert!(cmd.contains("\"C:\\Program Files\\app.exe\""));
        assert!(cmd.contains("arg1"));
        assert!(cmd.contains("\"arg with spaces\""));
    }
}
