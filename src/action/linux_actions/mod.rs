mod x11_actions;
pub use x11_actions::*;
// impl SystemFontProvider for LinuxOs {
//     fn get_system_font_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//         let candidates = [
//             "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
//             "/usr/share/fonts/TTF/DejaVuSans.ttf",
//         ];

//         for path in candidates {
//             if let Ok(bytes) = fs::read(path) {
//                 return Ok(bytes);
//             }
//         }

//         Err("no system font found".into())
//     }
// }
