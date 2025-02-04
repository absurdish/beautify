use rand::Rng;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
impl RGB {
    fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

fn hex_to_rgb(hex: &str) -> Option<RGB> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(RGB::new(r, g, b))
}

fn is_hex_color(s: &str) -> bool {
    let s = s.trim_start_matches('#');
    s.len() == 6 && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_rgb_color(s: &str) -> bool {
    s.starts_with("rgb(") && s.ends_with(')')
}

fn parse_rgb(s: &str) -> Option<RGB> {
    let contents = s.trim_start_matches("rgb(").trim_end_matches(')');
    let parts: Vec<&str> = contents.split(',').collect();
    if parts.len() != 3 {
        return None;
    }

    let r = parts[0].trim().parse().ok()?;
    let g = parts[1].trim().parse().ok()?;
    let b = parts[2].trim().parse().ok()?;

    Some(RGB::new(r, g, b))
}

fn identify_color_type(s: &str) -> ColorType {
    if is_hex_color(s) {
        ColorType::Hex
    } else if is_rgb_color(s) {
        ColorType::RGB
    } else if s.starts_with("text_") || s.starts_with("bg_") {
        ColorType::Accent
    } else {
        ColorType::Default
    }
}

enum ColorType {
    Hex,
    RGB,
    Accent,
    Default,
}

/// implements methods for beautifying your code
pub trait Colors {
    // default colors
    /// sets text color to default terminal color
    fn text_default(&self) -> String;
    /// sets background to default terminal color
    fn bg_default(&self) -> String;

    /// sets text color to black (#000000)
    fn text_black(&self) -> String;
    /// sets background color to black (#000000)
    fn bg_black(&self) -> String;

    /// sets text color to red (#ff0000)
    fn text_red(&self) -> String;
    /// sets background color to red (#ff0000)
    fn bg_red(&self) -> String;

    /// sets text color to blue (#0000ff)
    fn text_blue(&self) -> String;
    /// sets background color to blue (#0000ff)
    fn bg_blue(&self) -> String;

    /// sets text color to green (#00ff00)
    fn text_green(&self) -> String;
    /// sets background color to green (#00ff00)
    fn bg_green(&self) -> String;

    /// sets text color to yellow (#ffff00)
    fn text_yellow(&self) -> String;
    /// sets background color to yellow (#ffff00)
    fn bg_yellow(&self) -> String;

    /// sets text color to magenta (#ff00ff)
    fn text_magenta(&self) -> String;
    /// sets background color to magenta (#ff00ff)
    fn bg_magenta(&self) -> String;

    /// sets text color to cyan (#00ffff)
    fn text_cyan(&self) -> String;
    /// sets background color to cyan (#00ffff)
    fn bg_cyan(&self) -> String;

    /// sets text color to white (#ffffff)
    fn text_white(&self) -> String;
    /// sets background color to white (#ffffff)
    fn bg_white(&self) -> String;

    // bright shades
    /// sets text color to gray (#808080)
    fn text_gray(&self) -> String;
    /// sets background color to gray (#808080)
    fn bg_gray(&self) -> String;

    /// sets text color to bright red (#ff5555)
    fn text_red_bright(&self) -> String;
    /// sets background color to bright red (#ff5555)
    fn bg_red_bright(&self) -> String;

    /// sets text color to bright blue (#5555ff)
    fn text_blue_bright(&self) -> String;
    /// sets background color to bright blue (#5555ff)
    fn bg_blue_bright(&self) -> String;

    /// sets text color to bright green (#55ff55)
    fn text_green_bright(&self) -> String;
    /// sets background color to bright green (#55ff55)
    fn bg_green_bright(&self) -> String;

    /// sets text color to bright yellow (#ffff55)
    fn text_yellow_bright(&self) -> String;
    /// sets background color to bright yellow (#ffff55)
    fn bg_yellow_bright(&self) -> String;

    /// sets text color to bright magenta (#ff55ff)
    fn text_magenta_bright(&self) -> String;
    /// sets background color to bright magenta (#ff55ff)
    fn bg_magenta_bright(&self) -> String;

    /// sets text color to bright cyan (#55ffff)
    fn text_cyan_bright(&self) -> String;
    /// sets background color to bright cyan (#55ffff)
    fn bg_cyan_bright(&self) -> String;

    /// sets text color to bright white (#ffffff)
    fn text_white_bright(&self) -> String;
    /// sets background color to bright white (#ffffff)
    fn bg_white_bright(&self) -> String;

    // accent colors - red
    /// sets text color to red-50 (#fef2f2)
    fn text_red_50(&self) -> String;
    /// sets background color to red-50 (#fef2f2)
    fn bg_red_50(&self) -> String;

    /// sets text color to red-100 (#fee2e2)
    fn text_red_100(&self) -> String;
    /// sets background color to red-100 (#fee2e2)
    fn bg_red_100(&self) -> String;

    /// sets text color to red-200 (#fecaca)
    fn text_red_200(&self) -> String;
    /// sets background color to red-200 (#fecaca)
    fn bg_red_200(&self) -> String;

    /// sets text color to red-300 (#fca5a5)
    fn text_red_300(&self) -> String;
    /// sets background color to red-300 (#fca5a5)
    fn bg_red_300(&self) -> String;

    /// sets text color to red-400 (#f87171)
    fn text_red_400(&self) -> String;
    /// sets background color to red-400 (#f87171)
    fn bg_red_400(&self) -> String;

    /// sets text color to red-500 (#ef4444)
    fn text_red_500(&self) -> String;
    /// sets background color to red-500 (#ef4444)
    fn bg_red_500(&self) -> String;

    /// sets text color to red-600 (#dc2626)
    fn text_red_600(&self) -> String;
    /// sets background color to red-600 (#dc2626)
    fn bg_red_600(&self) -> String;

    /// sets text color to red-700 (#b91c1c)
    fn text_red_700(&self) -> String;
    /// sets background color to red-700 (#b91c1c)
    fn bg_red_700(&self) -> String;

    /// sets text color to red-800 (#991b1b)
    fn text_red_800(&self) -> String;
    /// sets background color to red-800 (#991b1b)
    fn bg_red_800(&self) -> String;

    /// sets text color to red-900 (#7f1d1d)
    fn text_red_900(&self) -> String;
    /// sets background color to red-900 (#7f1d1d)
    fn bg_red_900(&self) -> String;

    /// sets text color to red-950 (#450a0a)
    fn text_red_950(&self) -> String;
    /// sets background color to red-950 (#450a0a)
    fn bg_red_950(&self) -> String;

    // accent colors - yellow
    /// sets text color to yellow-50 (#fefce8)
    fn text_yellow_50(&self) -> String;
    /// sets background color to yellow-50 (#fefce8)
    fn bg_yellow_50(&self) -> String;

    /// sets text color to yellow-100 (#fef9c3)
    fn text_yellow_100(&self) -> String;
    /// sets background color to yellow-100 (#fef9c3)
    fn bg_yellow_100(&self) -> String;

    /// sets text color to yellow-200 (#fef08a)
    fn text_yellow_200(&self) -> String;
    /// sets background color to yellow-200 (#fef08a)
    fn bg_yellow_200(&self) -> String;

    /// sets text color to yellow-300 (#fde047)
    fn text_yellow_300(&self) -> String;
    /// sets background color to yellow-300 (#fde047)
    fn bg_yellow_300(&self) -> String;

    /// sets text color to yellow-400 (#facc15)
    fn text_yellow_400(&self) -> String;
    /// sets background color to yellow-400 (#facc15)
    fn bg_yellow_400(&self) -> String;

    /// sets text color to yellow-500 (#eab308)
    fn text_yellow_500(&self) -> String;
    /// sets background color to yellow-500 (#eab308)
    fn bg_yellow_500(&self) -> String;

    /// sets text color to yellow-600 (#ca8a04)
    fn text_yellow_600(&self) -> String;
    /// sets background color to yellow-600 (#ca8a04)
    fn bg_yellow_600(&self) -> String;

    /// sets text color to yellow-700 (#a16207)
    fn text_yellow_700(&self) -> String;
    /// sets background color to yellow-700 (#a16207)
    fn bg_yellow_700(&self) -> String;

    /// sets text color to yellow-800 (#854d0e)
    fn text_yellow_800(&self) -> String;
    /// sets background color to yellow-800 (#854d0e)
    fn bg_yellow_800(&self) -> String;

    /// sets text color to yellow-900 (#713f12)
    fn text_yellow_900(&self) -> String;
    /// sets background color to yellow-900 (#713f12)
    fn bg_yellow_900(&self) -> String;

    /// sets text color to yellow-950 (#422006)
    fn text_yellow_950(&self) -> String;
    /// sets background color to yellow-950 (#422006)
    fn bg_yellow_950(&self) -> String;

    // accent colors - green
    /// sets text color to green-50 (#f0fdf4)
    fn text_green_50(&self) -> String;
    /// sets background color to green-50 (#f0fdf4)
    fn bg_green_50(&self) -> String;

    /// sets text color to green-100 (#dcfce7)
    fn text_green_100(&self) -> String;
    /// sets background color to green-100 (#dcfce7)
    fn bg_green_100(&self) -> String;

    /// sets text color to green-200 (#bbf7d0)
    fn text_green_200(&self) -> String;
    /// sets background color to green-200 (#bbf7d0)
    fn bg_green_200(&self) -> String;

    /// sets text color to green-300 (#86efac)
    fn text_green_300(&self) -> String;
    /// sets background color to green-300 (#86efac)
    fn bg_green_300(&self) -> String;

    /// sets text color to green-400 (#4ade80)
    fn text_green_400(&self) -> String;
    /// sets background color to green-400 (#4ade80)
    fn bg_green_400(&self) -> String;

    /// sets text color to green-500 (#22c55e)
    fn text_green_500(&self) -> String;
    /// sets background color to green-500 (#22c55e)
    fn bg_green_500(&self) -> String;

    /// sets text color to green-600 (#16a34a)
    fn text_green_600(&self) -> String;
    /// sets background color to green-600 (#16a34a)
    fn bg_green_600(&self) -> String;

    /// sets text color to green-700 (#15803d)
    fn text_green_700(&self) -> String;
    /// sets background color to green-700 (#15803d)
    fn bg_green_700(&self) -> String;

    /// sets text color to green-800 (#166534)
    fn text_green_800(&self) -> String;
    /// sets background color to green-800 (#166534)
    fn bg_green_800(&self) -> String;

    /// sets text color to green-900 (#14532d)
    fn text_green_900(&self) -> String;
    /// sets background color to green-900 (#14532d)
    fn bg_green_900(&self) -> String;

    /// sets text color to green-950 (#052e16)
    fn text_green_950(&self) -> String;
    /// sets background color to green-950 (#052e16)
    fn bg_green_950(&self) -> String;

    // accent colors - blue
    /// sets text color to blue-50 (#eff6ff)
    fn text_blue_50(&self) -> String;
    /// sets background color to blue-50 (#eff6ff)
    fn bg_blue_50(&self) -> String;

    /// sets text color to blue-100 (#dbeafe)
    fn text_blue_100(&self) -> String;
    /// sets background color to blue-100 (#dbeafe)
    fn bg_blue_100(&self) -> String;

    /// sets text color to blue-200 (#bfdbfe)
    fn text_blue_200(&self) -> String;
    /// sets background color to blue-200 (#bfdbfe)
    fn bg_blue_200(&self) -> String;

    /// sets text color to blue-300 (#93c5fd)
    fn text_blue_300(&self) -> String;
    /// sets background color to blue-300 (#93c5fd)
    fn bg_blue_300(&self) -> String;

    /// sets text color to blue-400 (#60a5fa)
    fn text_blue_400(&self) -> String;
    /// sets background color to blue-400 (#60a5fa)
    fn bg_blue_400(&self) -> String;

    /// sets text color to blue-500 (#3b82f6)
    fn text_blue_500(&self) -> String;
    /// sets background color to blue-500 (#3b82f6)
    fn bg_blue_500(&self) -> String;

    /// sets text color to blue-600 (#2563eb)
    fn text_blue_600(&self) -> String;
    /// sets background color to blue-600 (#2563eb)
    fn bg_blue_600(&self) -> String;

    /// sets text color to blue-700 (#1d4ed8)
    fn text_blue_700(&self) -> String;
    /// sets background color to blue-700 (#1d4ed8)
    fn bg_blue_700(&self) -> String;

    /// sets text color to blue-800 (#1e40af)
    fn text_blue_800(&self) -> String;
    /// sets background color to blue-800 (#1e40af)
    fn bg_blue_800(&self) -> String;

    /// sets text color to blue-900 (#1e3a8a)
    fn text_blue_900(&self) -> String;
    /// sets background color to blue-900 (#1e3a8a)
    fn bg_blue_900(&self) -> String;

    /// sets text color to blue-950 (#172554)
    fn text_blue_950(&self) -> String;
    /// sets background color to blue-950 (#172554)
    fn bg_blue_950(&self) -> String;

    // accent colors - purple
    /// sets text color to purple-50 (#faf5ff)
    fn text_purple_50(&self) -> String;
    /// sets background color to purple-50 (#faf5ff)
    fn bg_purple_50(&self) -> String;

    /// sets text color to purple-100 (#f3e8ff)
    fn text_purple_100(&self) -> String;
    /// sets background color to purple-100 (#f3e8ff)
    fn bg_purple_100(&self) -> String;

    /// sets text color to purple-200 (#e9d5ff)
    fn text_purple_200(&self) -> String;
    /// sets background color to purple-200 (#e9d5ff)
    fn bg_purple_200(&self) -> String;

    /// sets text color to purple-300 (#d8b4fe)
    fn text_purple_300(&self) -> String;
    /// sets background color to purple-300 (#d8b4fe)
    fn bg_purple_300(&self) -> String;

    /// sets text color to purple-400 (#c084fc)
    fn text_purple_400(&self) -> String;
    /// sets background color to purple-400 (#c084fc)
    fn bg_purple_400(&self) -> String;

    /// sets text color to purple-500 (#a855f7)
    fn text_purple_500(&self) -> String;
    /// sets background color to purple-500 (#a855f7)
    fn bg_purple_500(&self) -> String;

    /// sets text color to purple-600 (#9333ea)
    fn text_purple_600(&self) -> String;
    /// sets background color to purple-600 (#9333ea)
    fn bg_purple_600(&self) -> String;

    /// sets text color to purple-700 (#7e22ce)
    fn text_purple_700(&self) -> String;
    /// sets background color to purple-700 (#7e22ce)
    fn bg_purple_700(&self) -> String;

    /// sets text color to purple-800 (#6b21a8)
    fn text_purple_800(&self) -> String;
    /// sets background color to purple-800 (#6b21a8)
    fn bg_purple_800(&self) -> String;

    /// sets text color to purple-900 (#581c87)
    fn text_purple_900(&self) -> String;
    /// sets background color to purple-900 (#581c87)
    fn bg_purple_900(&self) -> String;

    /// sets text color to purple-950 (#3b0764)
    fn text_purple_950(&self) -> String;
    /// sets background color to purple-950 (#3b0764)
    fn bg_purple_950(&self) -> String;

    // accent colors - pink
    /// sets text color to pink-50 (#fdf2f8)
    fn text_pink_50(&self) -> String;
    /// sets background color to pink-50 (#fdf2f8)
    fn bg_pink_50(&self) -> String;

    /// sets text color to pink-100 (#fce7f3)
    fn text_pink_100(&self) -> String;
    /// sets background color to pink-100 (#fce7f3)
    fn bg_pink_100(&self) -> String;

    /// sets text color to pink-200 (#fbcfe8)
    fn text_pink_200(&self) -> String;
    /// sets background color to pink-200 (#fbcfe8)
    fn bg_pink_200(&self) -> String;

    /// sets text color to pink-300 (#f9a8d4)
    fn text_pink_300(&self) -> String;
    /// sets background color to pink-300 (#f9a8d4)
    fn bg_pink_300(&self) -> String;

    /// sets text color to pink-400 (#f472b6)
    fn text_pink_400(&self) -> String;
    /// sets background color to pink-400 (#f472b6)
    fn bg_pink_400(&self) -> String;

    /// sets text color to pink-500 (#ec4899)
    fn text_pink_500(&self) -> String;
    /// sets background color to pink-500 (#ec4899)
    fn bg_pink_500(&self) -> String;

    /// sets text color to pink-600 (#db2777)
    fn text_pink_600(&self) -> String;
    /// sets background color to pink-600 (#db2777)
    fn bg_pink_600(&self) -> String;

    /// sets text color to pink-700 (#be185d)
    fn text_pink_700(&self) -> String;
    /// sets background color to pink-700 (#be185d)
    fn bg_pink_700(&self) -> String;

    /// sets text color to pink-800 (#9d174d)
    fn text_pink_800(&self) -> String;
    /// sets background color to pink-800 (#9d174d)
    fn bg_pink_800(&self) -> String;

    /// sets text color to pink-900 (#831843)
    fn text_pink_900(&self) -> String;
    /// sets background color to pink-900 (#831843)
    fn bg_pink_900(&self) -> String;

    /// sets text color to pink-950 (#500724)
    fn text_pink_950(&self) -> String;
    /// sets background color to pink-950 (#500724)
    fn bg_pink_950(&self) -> String;

    // accent colors - black/gray
    /// sets text color to black-50 (#f9fafb)
    fn text_black_50(&self) -> String;
    /// sets background color to black-50 (#f9fafb)
    fn bg_black_50(&self) -> String;

    /// sets text color to black-100 (#f3f4f6)
    fn text_black_100(&self) -> String;
    /// sets background color to black-100 (#f3f4f6)
    fn bg_black_100(&self) -> String;

    /// sets text color to black-200 (#e5e7eb)
    fn text_black_200(&self) -> String;
    /// sets background color to black-200 (#e5e7eb)
    fn bg_black_200(&self) -> String;

    /// sets text color to black-300 (#d1d5db)
    fn text_black_300(&self) -> String;
    /// sets background color to black-300 (#d1d5db)
    fn bg_black_300(&self) -> String;

    /// sets text color to black-400 (#9ca3af)
    fn text_black_400(&self) -> String;
    /// sets background color to black-400 (#9ca3af)
    fn bg_black_400(&self) -> String;

    /// sets text color to black-500 (#6b7280)
    fn text_black_500(&self) -> String;
    /// sets background color to black-500 (#6b7280)
    fn bg_black_500(&self) -> String;

    /// sets text color to black-600 (#4b5563)
    fn text_black_600(&self) -> String;
    /// sets background color to black-600 (#4b5563)
    fn bg_black_600(&self) -> String;

    /// sets text color to black-700 (#374151)
    fn text_black_700(&self) -> String;
    /// sets background color to black-700 (#374151)
    fn bg_black_700(&self) -> String;

    /// sets text color to black-800 (#1f2937)
    fn text_black_800(&self) -> String;
    /// sets background color to black-800 (#1f2937)
    fn bg_black_800(&self) -> String;

    /// sets text color to black-900 (#111827)
    fn text_black_900(&self) -> String;
    /// sets background color to black-900 (#111827)
    fn bg_black_900(&self) -> String;

    /// sets text color to black-950 (#030712)
    fn text_black_950(&self) -> String;
    /// sets background color to black-950 (#030712)
    fn bg_black_950(&self) -> String;

    // gradient colors
    /// give the text gradient color by describing gradient steps
    /// argument can be hexagonal, rgb, default or an accent color
    ///
    /// it's better to use with longer texts for better results
    ///
    /// example:
    /// ```rs
    /// println!("{}", "hi!".text_gradient(["#f40420", "text_blue_500"]))
    /// ```
    fn text_gradient(&self, steps: &[&'static str]) -> String;

    /// give the background gradient color by describing gradient steps
    /// argument can be hexagonal, rgb, default or an accent color
    ///
    /// it's better to use with longer texts for better results
    ///
    /// example:
    /// ```rs
    /// println!("{}", "hi!".bg_gradient(["#f40420", "text_blue_500"]))
    /// ```
    fn bg_gradient(&self, steps: &[&'static str]) -> String;

    /// Makes text bold
    fn bold(&self) -> String;

    /// Makes text italic
    fn italic(&self) -> String;

    /// Adds underline to text
    fn underline(&self) -> String;

    /// Adds strikethrough to text
    fn strikethrough(&self) -> String;

    /// Makes text appear dimmer
    fn dim(&self) -> String;

    /// Makes text blink
    fn blink(&self) -> String;

    /// Swaps foreground and background colors
    fn reverse(&self) -> String;

    /// Makes text blink rapidly
    fn blink_fast(&self) -> String;

    /// Makes text blink slowly
    fn blink_slow(&self) -> String;

    /// Creates a fade-in effect using different brightness levels
    fn fade_in(&self) -> String;

    /// Creates a fade-out effect using different brightness levels
    fn fade_out(&self) -> String;

    /// Pads text with spaces on the left to reach given width
    fn pad_left(&self, width: usize) -> String;
    
    /// Pads text with spaces on the right to reach given width
    fn pad_right(&self, width: usize) -> String;

    /// Centers text within specified width
    fn center(&self, width: usize) -> String;

    /// Surrounds text with a box
    fn box_it(&self) -> String;

    /// Adds a single-line border around text
    fn border(&self) -> String;

    /// Adds a double-line border around text
    fn double_border(&self) -> String;

    // helpers
    fn code(&self, code: usize) -> String;
    fn codes(&self, initial: usize, codes: (usize, usize, usize)) -> String;
}

impl<T> Colors for T
where
    T: Display,
{
    // helpers
    fn code(&self, code: usize) -> String {
        let s = self.to_string();

        if s.contains("\x1B[") {
            // Split by reset code but keep the separators
            let parts: Vec<&str> = s.split_inclusive("\x1B[0m").collect();
            let mut result = String::new();

            for part in parts {
                if part.starts_with("\x1B[") {
                    // This part already has color formatting, keep it as is
                    result.push_str(part);
                } else {
                    // This part has no color formatting, apply new color
                    result.push_str(&format!("\x1B[{}m{}", code, part));
                }
            }
            result
        } else {
            // No existing formatting, apply color normally
            format!("\x1B[{}m{}\x1B[0m", code, s)
        }
    }

    fn codes(&self, initial: usize, (a, b, c): (usize, usize, usize)) -> String {
        let s = self.to_string();

        if s.contains("\x1B[") {
            let parts: Vec<&str> = s.split_inclusive("\x1B[0m").collect();
            let mut result = String::new();

            for part in parts {
                if part.starts_with("\x1B[") {
                    // Keep existing color formatting
                    result.push_str(part);
                } else {
                    // Apply new color to unformatted text
                    result.push_str(&format!("\x1B[{};2;{};{};{}m{}", initial, a, b, c, part));
                }
            }
            result
        } else {
            format!("\x1B[{};2;{};{};{}m{}\x1B[0m", initial, a, b, c, s)
        }
    }

    fn blink_fast(&self) -> String {
          // Some terminals support code 6 for rapid blink
          format!("\x1B[6m{}\x1B[0m", self)
      }
  
      fn blink_slow(&self) -> String {
          // Using standard blink (code 5)
          format!("\x1B[5m{}\x1B[0m", self)
      }
  
      fn fade_in(&self) -> String {
          let s = self.to_string();
          let mut result = String::new();
          let brightnesses = [232, 236, 240, 244, 248, 252]; // Using greyscale colors
          
          for (i, c) in s.chars().enumerate() {
              let brightness = brightnesses[i % brightnesses.len()];
              result.push_str(&format!("\x1B[38;5;{}m{}", brightness, c));
          }
          result.push_str("\x1B[0m");
          result
      }
  
      fn fade_out(&self) -> String {
          let s = self.to_string();
          let mut result = String::new();
          let brightnesses = [252, 248, 244, 240, 236, 232]; // Reverse greyscale
          
          for (i, c) in s.chars().enumerate() {
              let brightness = brightnesses[i % brightnesses.len()];
              result.push_str(&format!("\x1B[38;5;{}m{}", brightness, c));
          }
          result.push_str("\x1B[0m");
          result
      }

    fn pad_left(&self, width: usize) -> String {
         let s = self.to_string();
         let len = s.chars().count();
         if len >= width {
             return s;
         }
         format!("{}{}", " ".repeat(width - len), s)
     }
 
     fn pad_right(&self, width: usize) -> String {
         let s = self.to_string();
         let len = s.chars().count();
         if len >= width {
             return s;
         }
         format!("{}{}", s, " ".repeat(width - len))
     }

    fn center(&self, width: usize) -> String {
        let s = self.to_string();
        let len = s.chars().count();
        if len >= width {
            return s;
        }
        let left_pad = (width - len) / 2;
        let right_pad = width - len - left_pad;
        format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
    }

    fn box_it(&self) -> String {
        let s = self.to_string();
        let width = s.chars().count();
        format!("┌{}┐\n│{}│\n└{}┘", "─".repeat(width), s, "─".repeat(width))
    }

    fn border(&self) -> String {
        let s = self.to_string();
        let width = s.chars().count();
        format!("╭{}╮\n│{}│\n╰{}╯", "─".repeat(width), s, "─".repeat(width))
    }

    fn double_border(&self) -> String {
        let s = self.to_string();
        let width = s.chars().count();
        format!("╔{}╗\n║{}║\n╚{}╝", "═".repeat(width), s, "═".repeat(width))
    }

    fn bold(&self) -> String {
        self.code(1)
    }

    fn italic(&self) -> String {
        self.code(3)
    }

    fn underline(&self) -> String {
        self.code(4)
    }

    fn strikethrough(&self) -> String {
        self.code(9)
    }

    fn dim(&self) -> String {
        self.code(2)
    }

    fn blink(&self) -> String {
        self.code(5)
    }

    fn reverse(&self) -> String {
        self.code(7)
    }

    // default colors
    fn text_black(&self) -> String {
        self.code(30)
    }
    fn bg_black(&self) -> String {
        self.code(40)
    }

    fn text_red(&self) -> String {
        self.code(31)
    }
    fn bg_red(&self) -> String {
        self.code(41)
    }

    fn text_blue(&self) -> String {
        self.code(34)
    }
    fn bg_blue(&self) -> String {
        self.code(44)
    }

    fn text_green(&self) -> String {
        self.code(32)
    }
    fn bg_green(&self) -> String {
        self.code(42)
    }

    fn text_yellow(&self) -> String {
        self.code(33)
    }
    fn bg_yellow(&self) -> String {
        self.code(43)
    }

    fn text_magenta(&self) -> String {
        self.code(35)
    }
    fn bg_magenta(&self) -> String {
        self.code(45)
    }

    fn text_cyan(&self) -> String {
        self.code(36)
    }
    fn bg_cyan(&self) -> String {
        self.code(46)
    }

    fn text_white(&self) -> String {
        self.code(37)
    }
    fn bg_white(&self) -> String {
        self.code(47)
    }

    fn text_default(&self) -> String {
        self.code(39)
    }
    fn bg_default(&self) -> String {
        self.code(49)
    }

    // bright colors
    fn text_gray(&self) -> String {
        self.code(90)
    }
    fn bg_gray(&self) -> String {
        self.code(100)
    }

    fn text_red_bright(&self) -> String {
        self.code(91)
    }
    fn bg_red_bright(&self) -> String {
        self.code(101)
    }

    fn text_green_bright(&self) -> String {
        self.code(92)
    }
    fn bg_green_bright(&self) -> String {
        self.code(102)
    }

    fn text_yellow_bright(&self) -> String {
        self.code(93)
    }
    fn bg_yellow_bright(&self) -> String {
        self.code(103)
    }

    fn text_blue_bright(&self) -> String {
        self.code(94)
    }
    fn bg_blue_bright(&self) -> String {
        self.code(104)
    }

    fn text_magenta_bright(&self) -> String {
        self.code(95)
    }
    fn bg_magenta_bright(&self) -> String {
        self.code(105)
    }

    fn text_cyan_bright(&self) -> String {
        self.code(96)
    }
    fn bg_cyan_bright(&self) -> String {
        self.code(106)
    }

    fn text_white_bright(&self) -> String {
        self.code(97)
    }
    fn bg_white_bright(&self) -> String {
        self.code(107)
    }

    // color accents
    fn text_red_50(&self) -> String {
        self.codes(38, (254, 242, 242))
    }
    fn bg_red_50(&self) -> String {
        self.codes(48, (254, 242, 242))
    }
    fn text_red_100(&self) -> String {
        self.codes(38, (254, 226, 226))
    }
    fn bg_red_100(&self) -> String {
        self.codes(48, (254, 226, 226))
    }
    fn text_red_200(&self) -> String {
        self.codes(38, (254, 202, 202))
    }
    fn bg_red_200(&self) -> String {
        self.codes(48, (254, 202, 202))
    }
    fn text_red_300(&self) -> String {
        self.codes(38, (252, 165, 165))
    }
    fn bg_red_300(&self) -> String {
        self.codes(48, (252, 165, 165))
    }
    fn text_red_400(&self) -> String {
        self.codes(38, (248, 113, 113))
    }
    fn bg_red_400(&self) -> String {
        self.codes(48, (248, 113, 113))
    }
    fn text_red_500(&self) -> String {
        self.codes(38, (239, 68, 68))
    }
    fn bg_red_500(&self) -> String {
        self.codes(48, (239, 68, 68))
    }
    fn text_red_600(&self) -> String {
        self.codes(38, (220, 38, 38))
    }
    fn bg_red_600(&self) -> String {
        self.codes(48, (220, 38, 38))
    }
    fn text_red_700(&self) -> String {
        self.codes(38, (185, 28, 28))
    }
    fn bg_red_700(&self) -> String {
        self.codes(48, (185, 28, 28))
    }
    fn text_red_800(&self) -> String {
        self.codes(38, (153, 27, 27))
    }
    fn bg_red_800(&self) -> String {
        self.codes(48, (153, 27, 27))
    }
    fn text_red_900(&self) -> String {
        self.codes(38, (127, 29, 29))
    }
    fn bg_red_900(&self) -> String {
        self.codes(48, (127, 29, 29))
    }
    fn text_red_950(&self) -> String {
        self.codes(38, (69, 10, 10))
    }
    fn bg_red_950(&self) -> String {
        self.codes(48, (69, 10, 10))
    }

    fn text_yellow_50(&self) -> String {
        self.codes(38, (254, 252, 232))
    }
    fn bg_yellow_50(&self) -> String {
        self.codes(48, (254, 252, 232))
    }
    fn text_yellow_100(&self) -> String {
        self.codes(38, (254, 249, 195))
    }
    fn bg_yellow_100(&self) -> String {
        self.codes(48, (254, 249, 195))
    }
    fn text_yellow_200(&self) -> String {
        self.codes(38, (254, 240, 138))
    }
    fn bg_yellow_200(&self) -> String {
        self.codes(48, (254, 240, 138))
    }
    fn text_yellow_300(&self) -> String {
        self.codes(38, (253, 224, 71))
    }
    fn bg_yellow_300(&self) -> String {
        self.codes(48, (253, 224, 71))
    }
    fn text_yellow_400(&self) -> String {
        self.codes(38, (250, 204, 21))
    }
    fn bg_yellow_400(&self) -> String {
        self.codes(48, (250, 204, 21))
    }
    fn text_yellow_500(&self) -> String {
        self.codes(38, (234, 179, 8))
    }
    fn bg_yellow_500(&self) -> String {
        self.codes(48, (234, 179, 8))
    }
    fn text_yellow_600(&self) -> String {
        self.codes(38, (202, 138, 4))
    }
    fn bg_yellow_600(&self) -> String {
        self.codes(48, (202, 138, 4))
    }
    fn text_yellow_700(&self) -> String {
        self.codes(38, (161, 98, 7))
    }
    fn bg_yellow_700(&self) -> String {
        self.codes(48, (161, 98, 7))
    }
    fn text_yellow_800(&self) -> String {
        self.codes(38, (133, 77, 14))
    }
    fn bg_yellow_800(&self) -> String {
        self.codes(48, (133, 77, 14))
    }
    fn text_yellow_900(&self) -> String {
        self.codes(38, (113, 63, 18))
    }
    fn bg_yellow_900(&self) -> String {
        self.codes(48, (113, 63, 18))
    }
    fn text_yellow_950(&self) -> String {
        self.codes(38, (66, 32, 6))
    }
    fn bg_yellow_950(&self) -> String {
        self.codes(48, (66, 32, 6))
    }

    fn text_green_50(&self) -> String {
        self.codes(38, (240, 253, 244))
    }
    fn bg_green_50(&self) -> String {
        self.codes(48, (240, 253, 244))
    }
    fn text_green_100(&self) -> String {
        self.codes(38, (220, 252, 231))
    }
    fn bg_green_100(&self) -> String {
        self.codes(48, (220, 252, 231))
    }
    fn text_green_200(&self) -> String {
        self.codes(38, (187, 247, 208))
    }
    fn bg_green_200(&self) -> String {
        self.codes(48, (187, 247, 208))
    }
    fn text_green_300(&self) -> String {
        self.codes(38, (134, 239, 172))
    }
    fn bg_green_300(&self) -> String {
        self.codes(48, (134, 239, 172))
    }
    fn text_green_400(&self) -> String {
        self.codes(38, (74, 222, 128))
    }
    fn bg_green_400(&self) -> String {
        self.codes(48, (74, 222, 128))
    }
    fn text_green_500(&self) -> String {
        self.codes(38, (34, 197, 94))
    }
    fn bg_green_500(&self) -> String {
        self.codes(48, (34, 197, 94))
    }
    fn text_green_600(&self) -> String {
        self.codes(38, (22, 163, 74))
    }
    fn bg_green_600(&self) -> String {
        self.codes(48, (22, 163, 74))
    }
    fn text_green_700(&self) -> String {
        self.codes(38, (21, 128, 61))
    }
    fn bg_green_700(&self) -> String {
        self.codes(48, (21, 128, 61))
    }
    fn text_green_800(&self) -> String {
        self.codes(38, (22, 101, 52))
    }
    fn bg_green_800(&self) -> String {
        self.codes(48, (22, 101, 52))
    }
    fn text_green_900(&self) -> String {
        self.codes(38, (20, 83, 45))
    }
    fn bg_green_900(&self) -> String {
        self.codes(48, (20, 83, 45))
    }
    fn text_green_950(&self) -> String {
        self.codes(38, (5, 46, 22))
    }
    fn bg_green_950(&self) -> String {
        self.codes(48, (5, 46, 22))
    }

    fn text_blue_50(&self) -> String {
        self.codes(38, (239, 246, 255))
    }
    fn bg_blue_50(&self) -> String {
        self.codes(48, (239, 246, 255))
    }
    fn text_blue_100(&self) -> String {
        self.codes(38, (219, 234, 254))
    }
    fn bg_blue_100(&self) -> String {
        self.codes(48, (219, 234, 254))
    }
    fn text_blue_200(&self) -> String {
        self.codes(38, (191, 219, 254))
    }
    fn bg_blue_200(&self) -> String {
        self.codes(48, (191, 219, 254))
    }
    fn text_blue_300(&self) -> String {
        self.codes(38, (147, 197, 253))
    }
    fn bg_blue_300(&self) -> String {
        self.codes(48, (147, 197, 253))
    }
    fn text_blue_400(&self) -> String {
        self.codes(38, (96, 165, 250))
    }
    fn bg_blue_400(&self) -> String {
        self.codes(48, (96, 165, 250))
    }
    fn text_blue_500(&self) -> String {
        self.codes(38, (59, 130, 246))
    }
    fn bg_blue_500(&self) -> String {
        self.codes(48, (59, 130, 246))
    }
    fn text_blue_600(&self) -> String {
        self.codes(38, (37, 99, 235))
    }
    fn bg_blue_600(&self) -> String {
        self.codes(48, (37, 99, 235))
    }
    fn text_blue_700(&self) -> String {
        self.codes(38, (29, 78, 216))
    }
    fn bg_blue_700(&self) -> String {
        self.codes(48, (29, 78, 216))
    }
    fn text_blue_800(&self) -> String {
        self.codes(38, (30, 64, 175))
    }
    fn bg_blue_800(&self) -> String {
        self.codes(48, (30, 64, 175))
    }
    fn text_blue_900(&self) -> String {
        self.codes(38, (30, 58, 138))
    }
    fn bg_blue_900(&self) -> String {
        self.codes(48, (30, 58, 138))
    }
    fn text_blue_950(&self) -> String {
        self.codes(38, (23, 37, 84))
    }
    fn bg_blue_950(&self) -> String {
        self.codes(48, (23, 37, 84))
    }

    fn text_purple_50(&self) -> String {
        self.codes(38, (250, 245, 255))
    }
    fn bg_purple_50(&self) -> String {
        self.codes(48, (250, 245, 255))
    }
    fn text_purple_100(&self) -> String {
        self.codes(38, (243, 232, 255))
    }
    fn bg_purple_100(&self) -> String {
        self.codes(48, (243, 232, 255))
    }
    fn text_purple_200(&self) -> String {
        self.codes(38, (233, 213, 255))
    }
    fn bg_purple_200(&self) -> String {
        self.codes(48, (233, 213, 255))
    }
    fn text_purple_300(&self) -> String {
        self.codes(38, (216, 180, 254))
    }
    fn bg_purple_300(&self) -> String {
        self.codes(48, (216, 180, 254))
    }
    fn text_purple_400(&self) -> String {
        self.codes(38, (192, 132, 252))
    }
    fn bg_purple_400(&self) -> String {
        self.codes(48, (192, 132, 252))
    }
    fn text_purple_500(&self) -> String {
        self.codes(38, (168, 85, 247))
    }
    fn bg_purple_500(&self) -> String {
        self.codes(48, (168, 85, 247))
    }
    fn text_purple_600(&self) -> String {
        self.codes(38, (147, 51, 234))
    }
    fn bg_purple_600(&self) -> String {
        self.codes(48, (147, 51, 234))
    }
    fn text_purple_700(&self) -> String {
        self.codes(38, (126, 34, 206))
    }
    fn bg_purple_700(&self) -> String {
        self.codes(48, (126, 34, 206))
    }
    fn text_purple_800(&self) -> String {
        self.codes(38, (107, 33, 168))
    }
    fn bg_purple_800(&self) -> String {
        self.codes(48, (107, 33, 168))
    }
    fn text_purple_900(&self) -> String {
        self.codes(38, (88, 28, 135))
    }
    fn bg_purple_900(&self) -> String {
        self.codes(48, (88, 28, 135))
    }
    fn text_purple_950(&self) -> String {
        self.codes(38, (59, 7, 100))
    }
    fn bg_purple_950(&self) -> String {
        self.codes(48, (59, 7, 100))
    }

    fn text_pink_50(&self) -> String {
        self.codes(38, (253, 242, 248))
    }
    fn bg_pink_50(&self) -> String {
        self.codes(48, (253, 242, 248))
    }
    fn text_pink_100(&self) -> String {
        self.codes(38, (252, 231, 243))
    }
    fn bg_pink_100(&self) -> String {
        self.codes(48, (252, 231, 243))
    }
    fn text_pink_200(&self) -> String {
        self.codes(38, (251, 207, 232))
    }
    fn bg_pink_200(&self) -> String {
        self.codes(48, (251, 207, 232))
    }
    fn text_pink_300(&self) -> String {
        self.codes(38, (249, 168, 212))
    }
    fn bg_pink_300(&self) -> String {
        self.codes(48, (249, 168, 212))
    }
    fn text_pink_400(&self) -> String {
        self.codes(38, (244, 114, 182))
    }
    fn bg_pink_400(&self) -> String {
        self.codes(48, (244, 114, 182))
    }
    fn text_pink_500(&self) -> String {
        self.codes(38, (236, 72, 153))
    }
    fn bg_pink_500(&self) -> String {
        self.codes(48, (236, 72, 153))
    }
    fn text_pink_600(&self) -> String {
        self.codes(38, (219, 39, 119))
    }
    fn bg_pink_600(&self) -> String {
        self.codes(48, (219, 39, 119))
    }
    fn text_pink_700(&self) -> String {
        self.codes(38, (190, 24, 93))
    }
    fn bg_pink_700(&self) -> String {
        self.codes(48, (190, 24, 93))
    }
    fn text_pink_800(&self) -> String {
        self.codes(38, (157, 23, 77))
    }
    fn bg_pink_800(&self) -> String {
        self.codes(48, (157, 23, 77))
    }
    fn text_pink_900(&self) -> String {
        self.codes(38, (131, 24, 67))
    }
    fn bg_pink_900(&self) -> String {
        self.codes(48, (131, 24, 67))
    }
    fn text_pink_950(&self) -> String {
        self.codes(38, (80, 7, 36))
    }
    fn bg_pink_950(&self) -> String {
        self.codes(48, (80, 7, 36))
    }

    fn text_black_50(&self) -> String {
        self.codes(38, (249, 250, 251))
    }
    fn bg_black_50(&self) -> String {
        self.codes(48, (249, 250, 251))
    }
    fn text_black_100(&self) -> String {
        self.codes(38, (243, 244, 246))
    }
    fn bg_black_100(&self) -> String {
        self.codes(48, (243, 244, 246))
    }
    fn text_black_200(&self) -> String {
        self.codes(38, (229, 231, 235))
    }
    fn bg_black_200(&self) -> String {
        self.codes(48, (229, 231, 235))
    }
    fn text_black_300(&self) -> String {
        self.codes(38, (209, 213, 219))
    }
    fn bg_black_300(&self) -> String {
        self.codes(48, (209, 213, 219))
    }
    fn text_black_400(&self) -> String {
        self.codes(38, (156, 163, 175))
    }
    fn bg_black_400(&self) -> String {
        self.codes(48, (156, 163, 175))
    }
    fn text_black_500(&self) -> String {
        self.codes(38, (107, 114, 128))
    }
    fn bg_black_500(&self) -> String {
        self.codes(48, (107, 114, 128))
    }
    fn text_black_600(&self) -> String {
        self.codes(38, (75, 85, 99))
    }
    fn bg_black_600(&self) -> String {
        self.codes(48, (75, 85, 99))
    }
    fn text_black_700(&self) -> String {
        self.codes(38, (55, 65, 81))
    }
    fn bg_black_700(&self) -> String {
        self.codes(48, (55, 65, 81))
    }
    fn text_black_800(&self) -> String {
        self.codes(38, (31, 41, 55))
    }
    fn bg_black_800(&self) -> String {
        self.codes(48, (31, 41, 55))
    }
    fn text_black_900(&self) -> String {
        self.codes(38, (17, 24, 39))
    }
    fn bg_black_900(&self) -> String {
        self.codes(48, (17, 24, 39))
    }
    fn text_black_950(&self) -> String {
        self.codes(38, (3, 7, 18))
    }
    fn bg_black_950(&self) -> String {
        self.codes(48, (3, 7, 18))
    }

    // gradient
    fn text_gradient(&self, steps: &[&'static str]) -> String {
        if steps.is_empty() {
            return self.to_string();
        }

        let text = self.to_string();
        if text.is_empty() {
            return text;
        }

        let mut result = String::new();
        let char_count = text.chars().count();

        let mut colors: Vec<RGB> = Vec::new();
        for step in steps {
            match identify_color_type(step) {
                ColorType::Hex => {
                    if let Some(rgb) = hex_to_rgb(step) {
                        colors.push(rgb);
                    }
                }
                ColorType::RGB => {
                    if let Some(rgb) = parse_rgb(step) {
                        colors.push(rgb);
                    }
                }
                _ => {}
            }
        }

        if colors.len() < 2 {
            return text;
        }

        for (i, c) in text.chars().enumerate() {
            let position = i as f32 / (char_count - 1) as f32;

            let segment = position * (colors.len() - 1) as f32;
            let index = segment.floor() as usize;
            let next_index = (index + 1).min(colors.len() - 1);

            let blend = segment.fract();

            let current_color = &colors[index];
            let next_color = &colors[next_index];

            let r = lerp(current_color.r as f32, next_color.r as f32, blend) as u8;
            let g = lerp(current_color.g as f32, next_color.g as f32, blend) as u8;
            let b = lerp(current_color.b as f32, next_color.b as f32, blend) as u8;

            result.push_str(&format!("\x1B[38;2;{};{};{}m{}", r, g, b, c));
        }

        result.push_str("\x1B[0m");
        result
    }

    fn bg_gradient(&self, steps: &[&'static str]) -> String {
        if steps.is_empty() {
            return self.to_string();
        }

        let text = self.to_string();
        if text.is_empty() {
            return text;
        }

        let mut result = String::new();
        let char_count = text.chars().count();

        let mut colors: Vec<RGB> = Vec::new();
        for step in steps {
            match identify_color_type(step) {
                ColorType::Hex => {
                    if let Some(rgb) = hex_to_rgb(step) {
                        colors.push(rgb);
                    }
                }
                ColorType::RGB => {
                    if let Some(rgb) = parse_rgb(step) {
                        colors.push(rgb);
                    }
                }
                _ => {}
            }
        }

        if colors.len() < 2 {
            return text;
        }

        for (i, c) in text.chars().enumerate() {
            let position = i as f32 / (char_count - 1) as f32;

            let segment = position * (colors.len() - 1) as f32;
            let index = segment.floor() as usize;
            let next_index = (index + 1).min(colors.len() - 1);

            let blend = segment.fract();

            let current_color = &colors[index];
            let next_color = &colors[next_index];

            let r = lerp(current_color.r as f32, next_color.r as f32, blend) as u8;
            let g = lerp(current_color.g as f32, next_color.g as f32, blend) as u8;
            let b = lerp(current_color.b as f32, next_color.b as f32, blend) as u8;

            result.push_str(&format!("\x1B[48;2;{};{};{}m{}", r, g, b, c));
        }

        result.push_str("\x1B[0m");
        result
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}
