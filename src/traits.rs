use mirl_buffer::Buffer;
// TODO: Replace bool returns with Result<(), ()>

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Errors that might occur when trying to get information from/to a window
///
/// TODO: Add some actual errors
pub enum WindowingErrors {}

/// Set the position of a window
pub const trait SetWindowPosition {
    /// Set the position of a window, ¯\_(ツ)_/¯
    fn set_window_position(handle: &raw_window_handle::RawWindowHandle, x: i32, y: i32) -> bool;
}

/// The simplest actions you would expect from interacting with the os
///
/// TODO: Add error handling (Currently silent)
pub const trait DefaultWindowing {
    /// How the os should go about ordering this window, check the documentation of [`WindowLevel`] for more information
    fn set_window_level(
        handle: &raw_window_handle::RawWindowHandle,
        level: WindowRenderLayer,
    ) -> bool;
    /// Get the current position of a window
    fn get_window_position(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32);
    /// Get the current size of a window
    fn get_window_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32);
    /// Get the current size of a window, including its decorations. Windows for example likes to add an 8 pixel padding both to the left and right sides of a window
    fn get_window_hitbox_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32);
    /// Resize the desired window to the desired size
    fn set_window_size(handle: &raw_window_handle::RawWindowHandle, size: (i32, i32)) -> bool;
}

/// Transparency information/Manipulation
pub const trait Transparency {
    /// Culls the given color -> Essentially just a green screen
    fn make_color_transparent(
        handle: &raw_window_handle::RawWindowHandle,
        color: (u8, u8, u8),
    ) -> bool;
    /// Sets the opacity ¯\_(ツ)_/¯
    fn set_window_opacity(handle: &raw_window_handle::RawWindowHandle, opacity: u8) -> bool;
}

/// Decoration like os menu manipulation
pub const trait Decoration {
    /// Remove/Give a window their border
    fn set_window_borderless(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool;
}

/// Stuff I couldn't categorize yet
pub const trait Misc {
    /// The title says it all
    fn set_window_hidden_from_taskbar_and_alt_tab(
        handle: &raw_window_handle::RawWindowHandle,
        boolean: bool,
    ) -> bool;
    /// Get ALL windows the os reveals
    fn get_all_windows() -> Vec<raw_window_handle::RawWindowHandle>;
    /// Get the title of the application associated with the given id
    fn get_title_using_id(handle: &raw_window_handle::RawWindowHandle) -> String;
    #[allow(clippy::fn_params_excessive_bools)]
    /// Get the title of a window
    fn get_id_using_title(
        title: &str,
        exact_match: bool,
        case_sensitive: bool,
        include_hidden: bool,
        just_one: bool,
    ) -> Option<Vec<raw_window_handle::RawWindowHandle>>;
    /// Capture the screen with all application - What happens if you have multiple monitors? Idk
    fn capture_screen() -> Option<Buffer>;
    /// Capture the desktop background without any applications
    fn capture_desktop_background() -> Option<Buffer>;
    /// Sets if you can click through a window
    fn set_click_ability_of_window(
        handle: &raw_window_handle::RawWindowHandle,
        click_through: bool,
    );
    /// Get the current z ordering of a window
    fn get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32;

    /// Sets the z ordering of the current window - How does [`WindowLevel`] affect ordering? No clue.
    fn set_window_z(handle: &raw_window_handle::RawWindowHandle, z: u32) -> bool;
    /// Sets the z ordering of the current window - How does [`WindowLevel`] affect ordering? No clue.
    fn set_window_z_after(
        handle: &raw_window_handle::RawWindowHandle,
        after: &raw_window_handle::RawWindowHandle,
    ) -> bool;
    /// Set the priority of a running process
    fn set_cpu_priority(handle: &raw_window_handle::RawWindowHandle, priority: ProcessCpuPriority);
}

/// Additional actions for tinkering with the taskbar
pub const trait TaskBar {
    // fn flash(handle: &raw_window_handle::RawWindowHandle);
    // /// For things like `Recently used` or `Pinned`
    // fn right_click_menu(
    //     handle: &raw_window_handle::RawWindowHandle,
    //     actions: std::collections::HashMap<String, String>,
    // );
    // /// Set a custom preview instead of the default snapshot the os takes
    // fn set_preview(
    //     handle: &raw_window_handle::RawWindowHandle,
    //     preview: &Buffer,
    // );
    // /// Little buttons like mute, pause, next
    // fn set_toolbar_tools(
    //     handle: &raw_window_handle::RawWindowHandle,
    //     tools: Vec<ToolbarTool>,
    // );
    // /// Like the discord red circle
    // fn set_icon_overlay(
    //     handle: &raw_window_handle::RawWindowHandle,
    //     overlay: &Buffer,
    // );
    /// Loading indicators
    fn set_icon_state(handle: &raw_window_handle::RawWindowHandle, state: &TaskbarLoadingState);
    /// Loading progress
    fn set_icon_progress(handle: &raw_window_handle::RawWindowHandle, current: u64, total: u64);
}
/// The state of a window - Is it here or minimized away into an icon?
pub const trait Iconized {
    /// Checks wether a window is currently minimized
    fn is_minimized(handle: &raw_window_handle::RawWindowHandle) -> bool;
    /// Checks wether a window is currently maximized
    fn is_maximized(handle: &raw_window_handle::RawWindowHandle) -> bool;
    /// When a window is minimized, this restores it back to "normal"
    fn restore(handle: &raw_window_handle::RawWindowHandle) -> bool;
    /// Minimize/Iconify the given window
    fn minimize(handle: &raw_window_handle::RawWindowHandle) -> bool;
    /// Maximize the given window (full screen like)
    fn maximize(handle: &raw_window_handle::RawWindowHandle) -> bool;
}

/// Screen information
pub const trait Screen {
    /// Get the resolution of the screen in pixels (does not support multiple monitors, multi screen workspaces)
    fn get_screen_resolution() -> (i32, i32);
}
/// Get the position at which an object needs to be for it to appear at the center of the screen
pub const trait ObjectInCenterOfScreen {
    /// Get the postion (top left corner) where the object needs to be to appear in the center of the screen
    fn get_center_of_screen_for_object(size: (i32, i32)) -> (i32, i32);
}
impl<S: Screen> ObjectInCenterOfScreen for S {
    fn get_center_of_screen_for_object(size: (i32, i32)) -> (i32, i32) {
        let (screen_width, screen_height): (i32, i32) = S::get_screen_resolution();

        (
            screen_width / 2 - size.0 / 2,
            screen_height / 2 - size.1 / 2,
        )
    }
}

#[cfg(feature = "font_support")]
/// Get the system font
pub trait SystemFontProvider {
    /// Get a font file, use [`to_font`](FileData::to_font) to convert the file into a font
    ///
    /// # Errors
    /// Plenty of things can go wrong from the file being hidden to the Os refusing to answer
    fn get_system_font_file(
        &self,
    ) -> Result<mirl_collections::BinaryData, Box<dyn std::error::Error>>;
    /// Get a font file, use [`to_font`](FileData::to_font) to convert the file into a font
    ///
    /// # Errors
    /// Plenty of things can go wrong from the file being hidden to the Os refusing to answer
    fn get_system_font(&self) -> Result<fontdue::Font, Box<dyn std::error::Error>> {
        let file = self.get_system_font_file()?;
        file.to_font()
    }
}
/// Execute programs in elevated mode or detect if a program is in elevated
pub const trait ElevatedExecution {
    /// Execute the given executable in elevated mode if allowed, if denied run it in normal mode
    ///
    /// # Errors
    /// It's the os, anything could go wrong anytime
    fn execute_in_elevated_mode(
        path: &std::path::Path,
        args: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>>;
    /// Check if the given process is in elevated mode
    ///
    /// # Errors
    /// It's the os, anything could go wrong anytime
    fn is_in_elevated_mode(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}

// /// Os information
// pub const trait Host {
//     /// Get the name of the os
//     /// Windows -> Windows
//     /// Linux (Ubuntu) -> Ubuntu
//     /// Linux (Mint) -> Mint
//     /// Linux (Unknown) -> Linux
//     fn get_os_name() -> String;

//     /// Get the name of the kernel
//     /// Windows -> Windows
//     /// Linux (Ubuntu) -> Linux
//     /// Linux (Mint) -> Linux
//     /// Linux (Unknown) -> Linux
//     fn get_kernel_name() -> String;
// }

/// The loading state of the taskbar icon be aware that some OS may not support all of these
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
// #[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
pub enum TaskbarLoadingState {
    #[default]
    /// Default behavior
    Normal,
    /// Make the white bar Red
    Error,
    /// Make the white bar Yellow
    Paused,
    /// Flat throbber
    Loading,
    /// Hide bar
    NoBar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// The cpu/thread priority a process can have
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
pub enum ProcessCpuPriority {
    /// I do not with to be perceived
    Idle,
    /// Handled later than default processes
    BelowNormal,
    /// The default priority
    Normal,
    /// Handled earlier than default processes
    AboveNormal,
    /// Handled even earlier than default processes
    High,
    /// I own the cpu.
    Realtime,
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
// #[cfg_attr(feature = "wincode", derive(wincode::SchemaWrite, wincode::SchemaRead))]
#[derive(PartialEq, Copy, Clone, Debug, Eq, PartialOrd, Ord)]
/// The render level of the window the os should use
/// Any window on the same render level will move in front of every other window on that same level if the user clicks them
pub enum WindowRenderLayer {
    /// Render layer on the bottom -> Always under '[`Self::Normal`]'
    AlwaysOnBottom,
    /// Render layer sandwiched in the middle of '[`Self::AlwaysOnBottom`]' and '[`WindowLevel::AlwaysOnTop`]'
    Normal,
    /// Render layer on the top -> Always on top of '[`Self::Normal`]'
    AlwaysOnTop,
}
