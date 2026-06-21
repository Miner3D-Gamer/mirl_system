#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A struct holding information about the underlying os
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct MacActions {}

// impl SystemFontProvider for MacOs {
//     fn get_system_font_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//         let path = "/System/Library/Fonts/SFNS.ttf";
//         Ok(fs::read(path)?)
//     }
// }
