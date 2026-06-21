use winapi::um::winuser::GetAsyncKeyState;

use crate::platform::keycodes::KeyCode;

/// Convert Windows VK codes to `KeyCode` enum
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn vk_code_to_keycode(vk_code: u32) -> KeyCode {
    match vk_code {
        // Letters A-Z (0x41-0x5A)
        0x41 => KeyCode::A,
        0x42 => KeyCode::B,
        0x43 => KeyCode::C,
        0x44 => KeyCode::D,
        0x45 => KeyCode::E,
        0x46 => KeyCode::F,
        0x47 => KeyCode::G,
        0x48 => KeyCode::H,
        0x49 => KeyCode::I,
        0x4A => KeyCode::J,
        0x4B => KeyCode::K,
        0x4C => KeyCode::L,
        0x4D => KeyCode::M,
        0x4E => KeyCode::N,
        0x4F => KeyCode::O,
        0x50 => KeyCode::P,
        0x51 => KeyCode::Q,
        0x52 => KeyCode::R,
        0x53 => KeyCode::S,
        0x54 => KeyCode::T,
        0x55 => KeyCode::U,
        0x56 => KeyCode::V,
        0x57 => KeyCode::W,
        0x58 => KeyCode::X,
        0x59 => KeyCode::Y,
        0x5A => KeyCode::Z,

        // Numbers 0-9 (0x30-0x39)
        0x30 => KeyCode::Num0,
        0x31 => KeyCode::Num1,
        0x32 => KeyCode::Num2,
        0x33 => KeyCode::Num3,
        0x34 => KeyCode::Num4,
        0x35 => KeyCode::Num5,
        0x36 => KeyCode::Num6,
        0x37 => KeyCode::Num7,
        0x38 => KeyCode::Num8,
        0x39 => KeyCode::Num9,

        // Keypad numbers (0x60-0x69)
        0x60 => KeyCode::KeyPad0,
        0x61 => KeyCode::KeyPad1,
        0x62 => KeyCode::KeyPad2,
        0x63 => KeyCode::KeyPad3,
        0x64 => KeyCode::KeyPad4,
        0x65 => KeyCode::KeyPad5,
        0x66 => KeyCode::KeyPad6,
        0x67 => KeyCode::KeyPad7,
        0x68 => KeyCode::KeyPad8,
        0x69 => KeyCode::KeyPad9,

        // Function keys F1-F24 (0x70-0x87)
        0x70 => KeyCode::F1,
        0x71 => KeyCode::F2,
        0x72 => KeyCode::F3,
        0x73 => KeyCode::F4,
        0x74 => KeyCode::F5,
        0x75 => KeyCode::F6,
        0x76 => KeyCode::F7,
        0x77 => KeyCode::F8,
        0x78 => KeyCode::F9,
        0x79 => KeyCode::F10,
        0x7A => KeyCode::F11,
        0x7B => KeyCode::F12,
        0x7C => KeyCode::F13,
        0x7D => KeyCode::F14,
        0x7E => KeyCode::F15,
        0x7F => KeyCode::F16,
        0x80 => KeyCode::F17,
        0x81 => KeyCode::F18,
        0x82 => KeyCode::F19,
        0x83 => KeyCode::F20,
        0x84 => KeyCode::F21,
        0x85 => KeyCode::F22,
        0x86 => KeyCode::F23,
        0x87 => KeyCode::F24,

        // Modifiers
        0xA0 => KeyCode::LeftShift,
        0xA1 => KeyCode::RightShift,
        0xA2 => KeyCode::LeftControl,
        0xA3 => KeyCode::RightControl,
        0xA4 => KeyCode::LeftAlt,
        0xA5 => KeyCode::RightAlt,
        0x5B => KeyCode::LeftSuper,  // VK_LWIN
        0x5C => KeyCode::RightSuper, // VK_RWIN

        // Special keys
        0x20 => KeyCode::Space,
        0x0D => KeyCode::Enter,
        0x1B => KeyCode::Escape,
        0x08 => KeyCode::Backspace,
        0x09 => KeyCode::Tab,

        // Punctuation
        0xBC => KeyCode::Comma,        // VK_OEM_COMMA
        0xBE => KeyCode::Period,       // VK_OEM_PERIOD
        0xBD => KeyCode::Minus,        // VK_OEM_MINUS
        0xBB => KeyCode::Equal,        // VK_OEM_PLUS
        0xDB => KeyCode::LeftBracket,  // VK_OEM_4
        0xDD => KeyCode::RightBracket, // VK_OEM_6
        0xDC => KeyCode::Backslash,    // VK_OEM_5
        0xBA => KeyCode::Semicolon,    // VK_OEM_1
        0xDE => KeyCode::Quote,        // VK_OEM_7
        0xC0 => KeyCode::Grave,        // VK_OEM_3
        0xBF => KeyCode::Slash,        // VK_OEM_2

        // Arrow keys
        0x26 => KeyCode::UpArrow,
        0x28 => KeyCode::DownArrow,
        0x25 => KeyCode::LeftArrow,
        0x27 => KeyCode::RightArrow,

        // Editing keys
        0x2D => KeyCode::Insert,
        0x2E => KeyCode::Delete,
        0x24 => KeyCode::Home,
        0x23 => KeyCode::End,
        0x21 => KeyCode::PageUp,
        0x22 => KeyCode::PageDown,

        // Lock keys
        0x14 => KeyCode::CapsLock,
        0x90 => KeyCode::NumLock,
        0x91 => KeyCode::ScrollLock,

        // Keypad operations
        0x6F => KeyCode::KeyPadDivide, // VK_DIVIDE
        0x6A => KeyCode::KeyPadMultiply, // VK_MULTIPLY
        0x6D => KeyCode::KeyPadSubtract, // VK_SUBTRACT
        0x6B => KeyCode::KeyPadAdd,    // VK_ADD
        0x6E => KeyCode::KeyPadDecimal, // VK_DECIMAL

        // Media keys
        0xB3 => KeyCode::MediaPlayPause, // VK_MEDIA_PLAY_PAUSE
        0xB2 => KeyCode::MediaStop,      // VK_MEDIA_STOP
        0xB0 => KeyCode::MediaNext,      // VK_MEDIA_NEXT_TRACK
        0xB1 => KeyCode::MediaPrev,      // VK_MEDIA_PREV_TRACK
        0xAF => KeyCode::VolumeUp,       // VK_VOLUME_UP
        0xAE => KeyCode::VolumeDown,     // VK_VOLUME_DOWN
        0xAD => KeyCode::Mute,           // VK_VOLUME_MUTE

        // Browser keys
        0xA6 => KeyCode::BrowserBack, // VK_BROWSER_BACK
        0xA7 => KeyCode::BrowserForward, // VK_BROWSER_FORWARD
        0xA8 => KeyCode::BrowserRefresh, // VK_BROWSER_REFRESH
        0xAC => KeyCode::BrowserHome, // VK_BROWSER_HOME

        // Application launch keys
        0xB4 => KeyCode::LaunchMail, // VK_LAUNCH_MAIL
        0xB6 => KeyCode::LaunchApp1, // VK_LAUNCH_APP1
        0xB7 => KeyCode::LaunchApp2, // VK_LAUNCH_APP2

        // Other keys
        0x5D => KeyCode::Menu,        // VK_APPS
        0x2C => KeyCode::PrintScreen, // VK_SNAPSHOT
        0x13 => KeyCode::Pause,       // VK_PAUSE

        _ => KeyCode::Unknown,
    }
}
/// Convert a keycode to the windows virtual key equivalent
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn keycode_to_vk_code(keycode: KeyCode) -> u32 {
    match keycode {
        // Letters A-Z
        KeyCode::A => 0x41,
        KeyCode::B => 0x42,
        KeyCode::C => 0x43,
        KeyCode::D => 0x44,
        KeyCode::E => 0x45,
        KeyCode::F => 0x46,
        KeyCode::G => 0x47,
        KeyCode::H => 0x48,
        KeyCode::I => 0x49,
        KeyCode::J => 0x4A,
        KeyCode::K => 0x4B,
        KeyCode::L => 0x4C,
        KeyCode::M => 0x4D,
        KeyCode::N => 0x4E,
        KeyCode::O => 0x4F,
        KeyCode::P => 0x50,
        KeyCode::Q => 0x51,
        KeyCode::R => 0x52,
        KeyCode::S => 0x53,
        KeyCode::T => 0x54,
        KeyCode::U => 0x55,
        KeyCode::V => 0x56,
        KeyCode::W => 0x57,
        KeyCode::X => 0x58,
        KeyCode::Y => 0x59,
        KeyCode::Z => 0x5A,

        // Numbers 0-9
        KeyCode::Num0 => 0x30,
        KeyCode::Num1 => 0x31,
        KeyCode::Num2 => 0x32,
        KeyCode::Num3 => 0x33,
        KeyCode::Num4 => 0x34,
        KeyCode::Num5 => 0x35,
        KeyCode::Num6 => 0x36,
        KeyCode::Num7 => 0x37,
        KeyCode::Num8 => 0x38,
        KeyCode::Num9 => 0x39,

        // Keypad numbers
        KeyCode::KeyPad0 => 0x60,
        KeyCode::KeyPad1 => 0x61,
        KeyCode::KeyPad2 => 0x62,
        KeyCode::KeyPad3 => 0x63,
        KeyCode::KeyPad4 => 0x64,
        KeyCode::KeyPad5 => 0x65,
        KeyCode::KeyPad6 => 0x66,
        KeyCode::KeyPad7 => 0x67,
        KeyCode::KeyPad8 => 0x68,
        KeyCode::KeyPad9 => 0x69,

        // Function keys F1-F24
        KeyCode::F1 => 0x70,
        KeyCode::F2 => 0x71,
        KeyCode::F3 => 0x72,
        KeyCode::F4 => 0x73,
        KeyCode::F5 => 0x74,
        KeyCode::F6 => 0x75,
        KeyCode::F7 => 0x76,
        KeyCode::F8 => 0x77,
        KeyCode::F9 => 0x78,
        KeyCode::F10 => 0x79,
        KeyCode::F11 => 0x7A,
        KeyCode::F12 => 0x7B,
        KeyCode::F13 => 0x7C,
        KeyCode::F14 => 0x7D,
        KeyCode::F15 => 0x7E,
        KeyCode::F16 => 0x7F,
        KeyCode::F17 => 0x80,
        KeyCode::F18 => 0x81,
        KeyCode::F19 => 0x82,
        KeyCode::F20 => 0x83,
        KeyCode::F21 => 0x84,
        KeyCode::F22 => 0x85,
        KeyCode::F23 => 0x86,
        KeyCode::F24 => 0x87,

        // Modifiers
        KeyCode::LeftShift => 0xA0,
        KeyCode::RightShift => 0xA1,
        KeyCode::LeftControl => 0xA2,
        KeyCode::RightControl => 0xA3,
        KeyCode::LeftAlt => 0xA4,
        KeyCode::RightAlt => 0xA5,
        KeyCode::LeftSuper => 0x5B,  // VK_LWIN
        KeyCode::RightSuper => 0x5C, // VK_RWIN

        // Special keys
        KeyCode::Space => 0x20,
        KeyCode::Enter => 0x0D,
        KeyCode::Escape => 0x1B,
        KeyCode::Backspace => 0x08,
        KeyCode::Tab => 0x09,

        // Punctuation
        KeyCode::Comma => 0xBC,        // VK_OEM_COMMA
        KeyCode::Period => 0xBE,       // VK_OEM_PERIOD
        KeyCode::Minus => 0xBD,        // VK_OEM_MINUS
        KeyCode::Equal => 0xBB,        // VK_OEM_PLUS
        KeyCode::LeftBracket => 0xDB,  // VK_OEM_4
        KeyCode::RightBracket => 0xDD, // VK_OEM_6
        KeyCode::Backslash => 0xDC,    // VK_OEM_5
        KeyCode::Semicolon => 0xBA,    // VK_OEM_1
        KeyCode::Quote => 0xDE,        // VK_OEM_7
        KeyCode::Grave => 0xC0,        // VK_OEM_3
        KeyCode::Slash => 0xBF,        // VK_OEM_2

        // Arrow keys
        KeyCode::UpArrow => 0x26,
        KeyCode::DownArrow => 0x28,
        KeyCode::LeftArrow => 0x25,
        KeyCode::RightArrow => 0x27,

        // Editing keys
        KeyCode::Insert => 0x2D,
        KeyCode::Delete => 0x2E,
        KeyCode::Home => 0x24,
        KeyCode::End => 0x23,
        KeyCode::PageUp => 0x21,
        KeyCode::PageDown => 0x22,

        // Lock keys
        KeyCode::CapsLock => 0x14,
        KeyCode::NumLock => 0x90,
        KeyCode::ScrollLock => 0x91,

        // Keypad operations
        KeyCode::KeyPadDivide => 0x6F, // VK_DIVIDE
        KeyCode::KeyPadMultiply => 0x6A, // VK_MULTIPLY
        KeyCode::KeyPadSubtract => 0x6D, // VK_SUBTRACT
        KeyCode::KeyPadAdd => 0x6B,    // VK_ADD
        KeyCode::KeyPadDecimal => 0x6E, // VK_DECIMAL

        // Media keys
        KeyCode::MediaPlayPause => 0xB3, // VK_MEDIA_PLAY_PAUSE
        KeyCode::MediaStop => 0xB2,      // VK_MEDIA_STOP
        KeyCode::MediaNext => 0xB0,      // VK_MEDIA_NEXT_TRACK
        KeyCode::MediaPrev => 0xB1,      // VK_MEDIA_PREV_TRACK
        KeyCode::VolumeUp => 0xAF,       // VK_VOLUME_UP
        KeyCode::VolumeDown => 0xAE,     // VK_VOLUME_DOWN
        KeyCode::Mute => 0xAD,           // VK_VOLUME_MUTE

        // Browser keys
        KeyCode::BrowserBack => 0xA6, // VK_BROWSER_BACK
        KeyCode::BrowserForward => 0xA7, // VK_BROWSER_FORWARD
        KeyCode::BrowserRefresh => 0xA8, // VK_BROWSER_REFRESH
        KeyCode::BrowserHome => 0xAC, // VK_BROWSER_HOME

        // Application launch keys
        KeyCode::LaunchMail => 0xB4, // VK_LAUNCH_MAIL
        KeyCode::LaunchApp1 => 0xB6, // VK_LAUNCH_APP1
        KeyCode::LaunchApp2 => 0xB7, // VK_LAUNCH_APP2

        // Other keys
        KeyCode::Menu => 0x5D,        // VK_APPS
        KeyCode::PrintScreen => 0x2C, // VK_SNAPSHOT
        KeyCode::Pause => 0x13,       // VK_PAUSE

        //KeyCode::Unknown => 0x00, // Return 0 for unknown keys
        _ => 0x00,
    }
}
/// Checks if the virtual key is pressed
#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn is_key_down_raw(vk_code: u32) -> bool {
    unsafe { (GetAsyncKeyState(vk_code as i32) as u16 & 0x8000) != 0 }
}
/// Checks if the key code is pressed (windows)
#[must_use]
pub fn is_key_down(vk_code: KeyCode) -> bool {
    is_key_down_raw(keycode_to_vk_code(vk_code))
}
