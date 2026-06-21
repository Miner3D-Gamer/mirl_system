#[cfg(target_os = "windows")]
mod windows;
#[cfg(feature = "keyboard_query")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
use device_query::{DeviceQuery, DeviceState, Keycode as DQKeycode};
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(feature = "keyboard_query")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
use mirl_core::platform::KeyCode;
/// Converts mirls keycodes to discovery queues keycodes
#[must_use]
#[cfg(feature = "keyboard_query")]
#[allow(clippy::too_many_lines)]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub const fn mirl_keycode_to_keyboard_query_keycode(
    keycode: KeyCode,
) -> Option<DQKeycode> {
    match keycode {
        KeyCode::A => Some(DQKeycode::A),
        KeyCode::B => Some(DQKeycode::B),
        KeyCode::C => Some(DQKeycode::C),
        KeyCode::D => Some(DQKeycode::D),
        KeyCode::E => Some(DQKeycode::E),
        KeyCode::F => Some(DQKeycode::F),
        KeyCode::G => Some(DQKeycode::G),
        KeyCode::H => Some(DQKeycode::H),
        KeyCode::I => Some(DQKeycode::I),
        KeyCode::J => Some(DQKeycode::J),
        KeyCode::K => Some(DQKeycode::K),
        KeyCode::L => Some(DQKeycode::L),
        KeyCode::M => Some(DQKeycode::M),
        KeyCode::N => Some(DQKeycode::N),
        KeyCode::O => Some(DQKeycode::O),
        KeyCode::P => Some(DQKeycode::P),
        KeyCode::Q => Some(DQKeycode::Q),
        KeyCode::R => Some(DQKeycode::R),
        KeyCode::S => Some(DQKeycode::S),
        KeyCode::T => Some(DQKeycode::T),
        KeyCode::U => Some(DQKeycode::U),
        KeyCode::V => Some(DQKeycode::V),
        KeyCode::W => Some(DQKeycode::W),
        KeyCode::X => Some(DQKeycode::X),
        KeyCode::Y => Some(DQKeycode::Y),
        KeyCode::Z => Some(DQKeycode::Z),
        KeyCode::Num0 => Some(DQKeycode::Key0),
        KeyCode::Num1 => Some(DQKeycode::Key1),
        KeyCode::Num2 => Some(DQKeycode::Key2),
        KeyCode::Num3 => Some(DQKeycode::Key3),
        KeyCode::Num4 => Some(DQKeycode::Key4),
        KeyCode::Num5 => Some(DQKeycode::Key5),
        KeyCode::Num6 => Some(DQKeycode::Key6),
        KeyCode::Num7 => Some(DQKeycode::Key7),
        KeyCode::Num8 => Some(DQKeycode::Key8),
        KeyCode::Num9 => Some(DQKeycode::Key9),
        KeyCode::KeyPad0 => Some(DQKeycode::Numpad0),
        KeyCode::KeyPad1 => Some(DQKeycode::Numpad1),
        KeyCode::KeyPad2 => Some(DQKeycode::Numpad2),
        KeyCode::KeyPad3 => Some(DQKeycode::Numpad3),
        KeyCode::KeyPad4 => Some(DQKeycode::Numpad4),
        KeyCode::KeyPad5 => Some(DQKeycode::Numpad5),
        KeyCode::KeyPad6 => Some(DQKeycode::Numpad6),
        KeyCode::KeyPad7 => Some(DQKeycode::Numpad7),
        KeyCode::KeyPad8 => Some(DQKeycode::Numpad8),
        KeyCode::KeyPad9 => Some(DQKeycode::Numpad9),
        KeyCode::F1 => Some(DQKeycode::F1),
        KeyCode::F2 => Some(DQKeycode::F2),
        KeyCode::F3 => Some(DQKeycode::F3),
        KeyCode::F4 => Some(DQKeycode::F4),
        KeyCode::F5 => Some(DQKeycode::F5),
        KeyCode::F6 => Some(DQKeycode::F6),
        KeyCode::F7 => Some(DQKeycode::F7),
        KeyCode::F8 => Some(DQKeycode::F8),
        KeyCode::F9 => Some(DQKeycode::F9),
        KeyCode::F10 => Some(DQKeycode::F10),
        KeyCode::F11 => Some(DQKeycode::F11),
        KeyCode::F12 => Some(DQKeycode::F12),
        KeyCode::LeftShift => Some(DQKeycode::LShift),
        KeyCode::RightShift => Some(DQKeycode::RShift),
        KeyCode::LeftControl => Some(DQKeycode::LControl),
        KeyCode::RightControl => Some(DQKeycode::RControl),
        KeyCode::LeftAlt => Some(DQKeycode::LAlt),
        KeyCode::RightAlt => Some(DQKeycode::RAlt),
        KeyCode::Space => Some(DQKeycode::Space),
        KeyCode::Enter => Some(DQKeycode::Enter),
        KeyCode::Escape => Some(DQKeycode::Escape),
        KeyCode::Backspace => Some(DQKeycode::Backspace),
        KeyCode::Tab => Some(DQKeycode::Tab),
        KeyCode::Comma => Some(DQKeycode::Comma),
        KeyCode::Period => Some(DQKeycode::Dot),
        KeyCode::Minus => Some(DQKeycode::Minus),
        KeyCode::Equal => Some(DQKeycode::Equal),
        KeyCode::LeftBracket => Some(DQKeycode::LeftBracket),
        KeyCode::RightBracket => Some(DQKeycode::RightBracket),
        KeyCode::Backslash => Some(DQKeycode::BackSlash),
        KeyCode::Semicolon => Some(DQKeycode::Semicolon),
        KeyCode::Grave => Some(DQKeycode::Grave),
        KeyCode::Slash => Some(DQKeycode::Slash),
        KeyCode::UpArrow => Some(DQKeycode::Up),
        KeyCode::DownArrow => Some(DQKeycode::Down),
        KeyCode::LeftArrow => Some(DQKeycode::Left),
        KeyCode::RightArrow => Some(DQKeycode::Right),
        KeyCode::Insert => Some(DQKeycode::Insert),
        KeyCode::Delete => Some(DQKeycode::Delete),
        KeyCode::Home => Some(DQKeycode::Home),
        KeyCode::End => Some(DQKeycode::End),
        KeyCode::PageUp => Some(DQKeycode::PageUp),
        KeyCode::PageDown => Some(DQKeycode::PageDown),
        KeyCode::CapsLock => Some(DQKeycode::CapsLock),
        KeyCode::KeyPadDivide => Some(DQKeycode::NumpadDivide),
        KeyCode::KeyPadMultiply => Some(DQKeycode::NumpadMultiply),
        KeyCode::KeyPadSubtract => Some(DQKeycode::NumpadSubtract),
        KeyCode::KeyPadAdd => Some(DQKeycode::NumpadAdd),
        KeyCode::KeyPadDecimal => Some(DQKeycode::NumpadDecimal),
        KeyCode::LeftSuper
        | KeyCode::RightSuper
        | KeyCode::NumLock
        | KeyCode::Quote
        | KeyCode::F13
        | KeyCode::F14
        | KeyCode::F15
        | KeyCode::F16
        | KeyCode::F17
        | KeyCode::F18
        | KeyCode::F19
        | KeyCode::F20
        | KeyCode::F21
        | KeyCode::F22
        | KeyCode::F23
        | KeyCode::F24
        | KeyCode::F25
        | KeyCode::KeyPadEnter
        | KeyCode::KeyPadEqual
        | KeyCode::Tilde
        | KeyCode::Apostrophe
        | KeyCode::ScrollLock
        | KeyCode::AUmlautÄ
        | KeyCode::UUmlautÜ
        | KeyCode::OUmlautÖ
        | KeyCode::SS
        | KeyCode::ACircumflexÂ
        | KeyCode::UAcuteÚ
        | KeyCode::OCircumflexÔ
        | KeyCode::ICircumflexÎ
        | KeyCode::ECircumflexÊ
        | KeyCode::EthÐ
        | KeyCode::OELigatureŒ
        | KeyCode::AAcuteÁ
        | KeyCode::YAcuteÝ
        | KeyCode::IUmlautÏ
        | KeyCode::NTildeÑ
        | KeyCode::OGraveÒ
        | KeyCode::UGraveÙ
        | KeyCode::ARingÅ
        | KeyCode::AELigatureÆ
        | KeyCode::OSlashØ
        | KeyCode::IGraveÌ
        | KeyCode::ThornÞ
        | KeyCode::MediaPlayPause
        | KeyCode::MediaStop
        | KeyCode::MediaNext
        | KeyCode::MediaPrev
        | KeyCode::VolumeUp
        | KeyCode::VolumeDown
        | KeyCode::Mute
        | KeyCode::BrowserBack
        | KeyCode::BrowserForward
        | KeyCode::BrowserRefresh
        | KeyCode::BrowserHome
        | KeyCode::LaunchMail
        | KeyCode::LaunchApp1
        | KeyCode::LaunchApp2
        | KeyCode::Menu
        | KeyCode::PrintScreen
        | KeyCode::Pause
        | KeyCode::Application
        | KeyCode::World1
        | KeyCode::World2
        | KeyCode::AnyAlt
        | KeyCode::AnyControl
        | KeyCode::AnyShift
        | KeyCode::AnySuper
        | KeyCode::Unknown
        | KeyCode::LeftHyper
        | KeyCode::RightHyper
        | KeyCode::AltControl
        | KeyCode::BackTab
        | KeyCode::MediaPlay
        | KeyCode::MediaPause
        | KeyCode::MediaReverse
        | KeyCode::MediaFastForward
        | KeyCode::MediaRecord
        | KeyCode::SpecialControl => None,
    }
}

/// Converts discovery queues keycodes to mirl keycodes
#[must_use]
#[allow(clippy::too_many_lines)]
#[cfg(feature = "keyboard_query")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub const fn keyboard_query_keycodes_to_mirls_keycode(
    dq_keycode: DQKeycode,
) -> KeyCode {
    match dq_keycode {
        // Letters
        DQKeycode::A => KeyCode::A,
        DQKeycode::B => KeyCode::B,
        DQKeycode::C => KeyCode::C,
        DQKeycode::D => KeyCode::D,
        DQKeycode::E => KeyCode::E,
        DQKeycode::F => KeyCode::F,
        DQKeycode::G => KeyCode::G,
        DQKeycode::H => KeyCode::H,
        DQKeycode::I => KeyCode::I,
        DQKeycode::J => KeyCode::J,
        DQKeycode::K => KeyCode::K,
        DQKeycode::L => KeyCode::L,
        DQKeycode::M => KeyCode::M,
        DQKeycode::N => KeyCode::N,
        DQKeycode::O => KeyCode::O,
        DQKeycode::P => KeyCode::P,
        DQKeycode::Q => KeyCode::Q,
        DQKeycode::R => KeyCode::R,
        DQKeycode::S => KeyCode::S,
        DQKeycode::T => KeyCode::T,
        DQKeycode::U => KeyCode::U,
        DQKeycode::V => KeyCode::V,
        DQKeycode::W => KeyCode::W,
        DQKeycode::X => KeyCode::X,
        DQKeycode::Y => KeyCode::Y,
        DQKeycode::Z => KeyCode::Z,

        // Numbers
        DQKeycode::Key0 => KeyCode::Num0,
        DQKeycode::Key1 => KeyCode::Num1,
        DQKeycode::Key2 => KeyCode::Num2,
        DQKeycode::Key3 => KeyCode::Num3,
        DQKeycode::Key4 => KeyCode::Num4,
        DQKeycode::Key5 => KeyCode::Num5,
        DQKeycode::Key6 => KeyCode::Num6,
        DQKeycode::Key7 => KeyCode::Num7,
        DQKeycode::Key8 => KeyCode::Num8,
        DQKeycode::Key9 => KeyCode::Num9,

        // Keypad
        DQKeycode::Numpad0 => KeyCode::KeyPad0,
        DQKeycode::Numpad1 => KeyCode::KeyPad1,
        DQKeycode::Numpad2 => KeyCode::KeyPad2,
        DQKeycode::Numpad3 => KeyCode::KeyPad3,
        DQKeycode::Numpad4 => KeyCode::KeyPad4,
        DQKeycode::Numpad5 => KeyCode::KeyPad5,
        DQKeycode::Numpad6 => KeyCode::KeyPad6,
        DQKeycode::Numpad7 => KeyCode::KeyPad7,
        DQKeycode::Numpad8 => KeyCode::KeyPad8,
        DQKeycode::Numpad9 => KeyCode::KeyPad9,

        // Function keys
        DQKeycode::F1 => KeyCode::F1,
        DQKeycode::F2 => KeyCode::F2,
        DQKeycode::F3 => KeyCode::F3,
        DQKeycode::F4 => KeyCode::F4,
        DQKeycode::F5 => KeyCode::F5,
        DQKeycode::F6 => KeyCode::F6,
        DQKeycode::F7 => KeyCode::F7,
        DQKeycode::F8 => KeyCode::F8,
        DQKeycode::F9 => KeyCode::F9,
        DQKeycode::F10 => KeyCode::F10,
        DQKeycode::F11 => KeyCode::F11,
        DQKeycode::F12 => KeyCode::F12,
        DQKeycode::F13 => KeyCode::F13,
        DQKeycode::F14 => KeyCode::F14,
        DQKeycode::F15 => KeyCode::F15,
        DQKeycode::F16 => KeyCode::F16,
        DQKeycode::F17 => KeyCode::F17,
        DQKeycode::F18 => KeyCode::F18,
        DQKeycode::F19 => KeyCode::F19,
        DQKeycode::F20 => KeyCode::F20,

        // Modifiers
        DQKeycode::LShift => KeyCode::LeftShift,
        DQKeycode::RShift => KeyCode::RightShift,
        DQKeycode::LControl | DQKeycode::Command => KeyCode::LeftControl,
        DQKeycode::RControl | DQKeycode::RCommand => KeyCode::RightControl,
        DQKeycode::LAlt | DQKeycode::LOption => KeyCode::LeftAlt,
        DQKeycode::RAlt | DQKeycode::ROption => KeyCode::RightAlt,

        // Special keys
        DQKeycode::Space => KeyCode::Space,
        DQKeycode::Enter => KeyCode::Enter,
        DQKeycode::Escape => KeyCode::Escape,
        DQKeycode::Backspace => KeyCode::Backspace,
        DQKeycode::Tab => KeyCode::Tab,

        // Punctuation
        DQKeycode::Comma => KeyCode::Comma,
        DQKeycode::Dot => KeyCode::Period,
        DQKeycode::Minus => KeyCode::Minus,
        DQKeycode::Equal => KeyCode::Equal,
        DQKeycode::LeftBracket => KeyCode::LeftBracket,
        DQKeycode::RightBracket => KeyCode::RightBracket,
        DQKeycode::BackSlash => KeyCode::Backslash,
        DQKeycode::Semicolon => KeyCode::Semicolon,
        DQKeycode::Grave => KeyCode::Grave,
        DQKeycode::Slash => KeyCode::Slash,

        // Arrow keys
        DQKeycode::Up => KeyCode::UpArrow,
        DQKeycode::Down => KeyCode::DownArrow,
        DQKeycode::Left => KeyCode::LeftArrow,
        DQKeycode::Right => KeyCode::RightArrow,

        // Editing keys
        DQKeycode::Insert => KeyCode::Insert,
        DQKeycode::Delete => KeyCode::Delete,
        DQKeycode::Home => KeyCode::Home,
        DQKeycode::End => KeyCode::End,
        DQKeycode::PageUp => KeyCode::PageUp,
        DQKeycode::PageDown => KeyCode::PageDown,

        // Lock keys
        DQKeycode::CapsLock => KeyCode::CapsLock,

        // Keypad operations
        DQKeycode::NumpadDivide => KeyCode::KeyPadDivide,
        DQKeycode::NumpadMultiply => KeyCode::KeyPadMultiply,
        DQKeycode::NumpadSubtract => KeyCode::KeyPadSubtract,
        DQKeycode::NumpadAdd => KeyCode::KeyPadAdd,
        DQKeycode::NumpadDecimal => KeyCode::KeyPadDecimal,
        DQKeycode::Apostrophe => KeyCode::Apostrophe,
        DQKeycode::LMeta => KeyCode::LeftSuper,
        DQKeycode::RMeta => KeyCode::RightSuper,
        DQKeycode::NumpadEnter => KeyCode::KeyPadEnter,
        DQKeycode::NumpadEquals => KeyCode::KeyPadEqual,
    }
}

/// One-off function to check if a key is currently pressed
/// This respects the current keyboard layout
#[must_use]
#[cfg(feature = "keyboard_query")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub fn is_key_pressed(keycode: KeyCode) -> bool {
    mirl_keycode_to_keyboard_query_keycode(keycode).is_some_and(|dq_keycode| {
        let device_state = DeviceState::new();
        let keys: Vec<DQKeycode> = device_state.get_keys();
        keys.contains(&dq_keycode)
    })
}

/// Checks if every key is down
#[must_use]
#[cfg(feature = "keyboard_query")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub fn get_all_pressed_keys() -> Vec<KeyCode> {
    let device_state = DeviceState::new();
    let keys: Vec<DQKeycode> = device_state.get_keys();
    let mut pressed = Vec::new();
    for key in keys {
        pressed.push(keyboard_query_keycodes_to_mirls_keycode(key));
    }
    pressed
}
