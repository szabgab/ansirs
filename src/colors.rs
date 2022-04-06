// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, AnsiFlags, IntoAnsi};

/// Error type used when parsing a color.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorParseError {
    /// Bad characters were found in the color string.
    BadChars,
    /// The color string had too few or too many characters.
    WrongLength,
    /// The color string segment could not be parsed into a valid decimal number.
    ParseIntError(std::num::ParseIntError),
    /// Other errors (with message).
    Unknown(String),
}

/// Trait used to facilitate converting various types to a color.
pub trait ToColor {
    /// Perform the conversion.
    fn to_color(&self) -> Color;
}

/// Wrapper struct around a (u8, u8, u8) tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Create a new color from the given RGB values.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Attempt to create a new color from the given hexadecimal string.
    pub fn from_hex<S: AsRef<str>>(input: S) -> Result<Self, ColorParseError> {
        fn convert(input: &str) -> Result<u8, ColorParseError> {
            u8::from_str_radix(input, 16).map_err(ColorParseError::ParseIntError)
        }

        let mut string = input.as_ref();

        if string.starts_with('#') {
            string = &string[1..];
        }

        if string.len() != 3 && string.len() != 6 {
            return Err(ColorParseError::WrongLength);
        }

        if !string.is_ascii() {}

        let is_double = string.len() == 6;

        let mut chars = string.chars();

        let mut rgb = [0u8, 0u8, 0u8];
        for idx in &mut rgb {
            *idx = if is_double {
                let f = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;
                let s = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;

                convert(&format!("{}{}", f, s))?
            } else {
                let c = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;

                convert(&format!("{}{}", c, c))?
            };
        }

        Ok(Self(rgb[0], rgb[1], rgb[2]))
    }

    /// Create a hex string from this color.
    pub fn as_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }

    /// Get the RGB tuple of this color.
    pub const fn rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    /// Get the **Red** value of this color.
    pub const fn r(&self) -> u8 {
        self.0
    }

    /// Get the **Green** value of this color.
    pub const fn g(&self) -> u8 {
        self.1
    }

    /// Get the **Blue** value of this color.
    pub const fn b(&self) -> u8 {
        self.2
    }

    pub fn into_ansi(self) -> Ansi {
        Ansi {
            fg: Some(self),
            bg: None,
            flags: AnsiFlags::empty(),
        }
    }
}

impl ToColor for Color {
    fn to_color(&self) -> Color {
        *self
    }
}

/// Enum containing known named colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Colors {
    Maroon,
    DarkRed,
    Brown,
    Firebrick,
    Crimson,
    Red,
    Tomato,
    Coral,
    IndianRed,
    LightCoral,
    DarkSalmon,
    Salmon,
    LightSalmon,
    OrangeRed,
    DarkOrange,
    Orange,
    Gold,
    DarkGoldenRod,
    GoldenRod,
    PaleGoldenRod,
    DarkKhaki,
    Khaki,
    Olive,
    Yellow,
    YellowGreen,
    DarkOliveGreen,
    OliveDrab,
    LawnGreen,
    Chartreuse,
    GreenYellow,
    DarkGreen,
    Green,
    ForestGreen,
    Lime,
    LimeGreen,
    LightGreen,
    PaleGreen,
    DarkSeaGreen,
    MediumSpringGreen,
    SpringGreen,
    SeaGreen,
    MediumAquaMarine,
    MediumSeaGreen,
    LightSeaGreen,
    DarkSlateGray,
    Teal,
    DarkCyan,
    Aqua,
    Cyan,
    LightCyan,
    DarkTurquoise,
    Turquoise,
    MediumTurquoise,
    PaleTurquoise,
    AquaMarine,
    PowderBlue,
    CadetBlue,
    SteelBlue,
    CornFlowerBlue,
    DeepSkyBlue,
    DodgerBlue,
    LightBlue,
    SkyBlue,
    LightSkyBlue,
    MidnightBlue,
    Navy,
    DarkBlue,
    MediumBlue,
    Blue,
    RoyalBlue,
    BlueViolet,
    Indigo,
    DarkSlateBlue,
    SlateBlue,
    MediumSlateBlue,
    MediumPurple,
    DarkMagenta,
    DarkViolet,
    DarkOrchid,
    MediumOrchid,
    Purple,
    Thistle,
    Plum,
    Violet,
    Magenta,
    Fuschia,
    Orchid,
    MediumVioletRed,
    PaleVioletRed,
    DeepPink,
    HotPink,
    LightPink,
    Pink,
    AntiqueWhite,
    Beige,
    Bisque,
    BlanchedAlmond,
    Wheat,
    CornSilk,
    LemonChiffon,
    LightGoldenRodYellow,
    LightYellow,
    SaddleBrown,
    Sienna,
    Chocolate,
    Peru,
    SandyBrown,
    BurlyWood,
    Tan,
    RosyBrown,
    Moccasin,
    NavajoWhite,
    PeachPuff,
    MistyRose,
    LavenderBlush,
    Linen,
    OldLace,
    PapayaWhip,
    SeaShell,
    MintCream,
    SlateGray,
    LightSlateGray,
    LightSteelBlue,
    Lavender,
    FloralWhite,
    AliceBlue,
    GhostWhite,
    Honeydew,
    Ivory,
    Azure,
    Snow,
    Black,
    DimGray,
    DimGrey,
    Gray,
    Grey,
    DarkGray,
    DarkGrey,
    Silver,
    LightGray,
    LightGrey,
    Gainsboro,
    WhiteSmoke,
    White,
}

impl Colors {
    /// Get the name of this color.
    pub fn name(&self) -> &str {
        match self {
            Self::Maroon => "Maroon",
            Self::DarkRed => "DarkRed",
            Self::Brown => "Brown",
            Self::Firebrick => "Firebrick",
            Self::Crimson => "Crimson",
            Self::Red => "Red",
            Self::Tomato => "Tomato",
            Self::Coral => "Coral",
            Self::IndianRed => "IndianRed",
            Self::LightCoral => "LightCoral",
            Self::DarkSalmon => "DarkSalmon",
            Self::Salmon => "Salmon",
            Self::LightSalmon => "LightSalmon",
            Self::OrangeRed => "OrangeRed",
            Self::DarkOrange => "DarkOrange",
            Self::Orange => "Orange",
            Self::Gold => "Gold",
            Self::DarkGoldenRod => "DarkGoldenRod",
            Self::GoldenRod => "GoldenRod",
            Self::PaleGoldenRod => "PaleGoldenRod",
            Self::DarkKhaki => "DarkKhaki",
            Self::Khaki => "Khaki",
            Self::Olive => "Olive",
            Self::Yellow => "Yellow",
            Self::YellowGreen => "YellowGreen",
            Self::DarkOliveGreen => "DarkOliveGreen",
            Self::OliveDrab => "OliveDrab",
            Self::LawnGreen => "LawnGreen",
            Self::Chartreuse => "Chartreuse",
            Self::GreenYellow => "GreenYellow",
            Self::DarkGreen => "DarkGreen",
            Self::Green => "Green",
            Self::ForestGreen => "ForestGreen",
            Self::Lime => "Lime",
            Self::LimeGreen => "LimeGreen",
            Self::LightGreen => "LightGreen",
            Self::PaleGreen => "PaleGreen",
            Self::DarkSeaGreen => "DarkSeaGreen",
            Self::MediumSpringGreen => "MediumSpringGreen",
            Self::SpringGreen => "SpringGreen",
            Self::SeaGreen => "SeaGreen",
            Self::MediumAquaMarine => "MediumAquaMarine",
            Self::MediumSeaGreen => "MediumSeaGreen",
            Self::LightSeaGreen => "LightSeaGreen",
            Self::DarkSlateGray => "DarkSlateGray",
            Self::Teal => "Teal",
            Self::DarkCyan => "DarkCyan",
            Self::Aqua => "Aqua",
            Self::Cyan => "Cyan",
            Self::LightCyan => "LightCyan",
            Self::DarkTurquoise => "DarkTurquoise",
            Self::Turquoise => "Turquoise",
            Self::MediumTurquoise => "MediumTurquoise",
            Self::PaleTurquoise => "PaleTurquoise",
            Self::AquaMarine => "AquaMarine",
            Self::PowderBlue => "PowderBlue",
            Self::CadetBlue => "CadetBlue",
            Self::SteelBlue => "SteelBlue",
            Self::CornFlowerBlue => "CornFlowerBlue",
            Self::DeepSkyBlue => "DeepSkyBlue",
            Self::DodgerBlue => "DodgerBlue",
            Self::LightBlue => "LightBlue",
            Self::SkyBlue => "SkyBlue",
            Self::LightSkyBlue => "LightSkyBlue",
            Self::MidnightBlue => "MidnightBlue",
            Self::Navy => "Navy",
            Self::DarkBlue => "DarkBlue",
            Self::MediumBlue => "MediumBlue",
            Self::Blue => "Blue",
            Self::RoyalBlue => "RoyalBlue",
            Self::BlueViolet => "BlueViolet",
            Self::Indigo => "Indigo",
            Self::DarkSlateBlue => "DarkSlateBlue",
            Self::SlateBlue => "SlateBlue",
            Self::MediumSlateBlue => "MediumSlateBlue",
            Self::MediumPurple => "MediumPurple",
            Self::DarkMagenta => "DarkMagenta",
            Self::DarkViolet => "DarkViolet",
            Self::DarkOrchid => "DarkOrchid",
            Self::MediumOrchid => "MediumOrchid",
            Self::Purple => "Purple",
            Self::Thistle => "Thistle",
            Self::Plum => "Plum",
            Self::Violet => "Violet",
            Self::Magenta => "Magenta",
            Self::Fuschia => "Fuschia",
            Self::Orchid => "Orchid",
            Self::MediumVioletRed => "MediumVioletRed",
            Self::PaleVioletRed => "PaleVioletRed",
            Self::DeepPink => "DeepPink",
            Self::HotPink => "HotPink",
            Self::LightPink => "LightPink",
            Self::Pink => "Pink",
            Self::AntiqueWhite => "AntiqueWhite",
            Self::Beige => "Beige",
            Self::Bisque => "Bisque",
            Self::BlanchedAlmond => "BlanchedAlmond",
            Self::Wheat => "Wheat",
            Self::CornSilk => "CornSilk",
            Self::LemonChiffon => "LemonChiffon",
            Self::LightGoldenRodYellow => "LightGoldenRodYellow",
            Self::LightYellow => "LightYellow",
            Self::SaddleBrown => "SaddleBrown",
            Self::Sienna => "Sienna",
            Self::Chocolate => "Chocolate",
            Self::Peru => "Peru",
            Self::SandyBrown => "SandyBrown",
            Self::BurlyWood => "BurlyWood",
            Self::Tan => "Tan",
            Self::RosyBrown => "RosyBrown",
            Self::Moccasin => "Moccasin",
            Self::NavajoWhite => "NavajoWhite",
            Self::PeachPuff => "PeachPuff",
            Self::MistyRose => "MistyRose",
            Self::LavenderBlush => "LavenderBlush",
            Self::Linen => "Linen",
            Self::OldLace => "OldLace",
            Self::PapayaWhip => "PapayaWhip",
            Self::SeaShell => "SeaShell",
            Self::MintCream => "MintCream",
            Self::SlateGray => "SlateGray",
            Self::LightSlateGray => "LightSlateGray",
            Self::LightSteelBlue => "LightSteelBlue",
            Self::Lavender => "Lavender",
            Self::FloralWhite => "FloralWhite",
            Self::AliceBlue => "AliceBlue",
            Self::GhostWhite => "GhostWhite",
            Self::Honeydew => "Honeydew",
            Self::Ivory => "Ivory",
            Self::Azure => "Azure",
            Self::Snow => "Snow",
            Self::Black => "Black",
            Self::DimGray => "DimGray",
            Self::DimGrey => "DimGrey",
            Self::Gray => "Gray",
            Self::Grey => "Grey",
            Self::DarkGray => "DarkGray",
            Self::DarkGrey => "DarkGrey",
            Self::Silver => "Silver",
            Self::LightGray => "LightGray",
            Self::LightGrey => "LightGrey",
            Self::Gainsboro => "Gainsboro",
            Self::WhiteSmoke => "WhiteSmoke",
            Self::White => "White",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::AliceBlue => Self::AntiqueWhite,
            Self::AntiqueWhite => Self::Aqua,
            Self::Aqua => Self::AquaMarine,
            Self::AquaMarine => Self::Azure,
            Self::Azure => Self::Beige,
            Self::Beige => Self::Bisque,
            Self::Bisque => Self::Black,
            Self::Black => Self::BlanchedAlmond,
            Self::BlanchedAlmond => Self::Blue,
            Self::Blue => Self::BlueViolet,
            Self::BlueViolet => Self::Brown,
            Self::Brown => Self::BurlyWood,
            Self::BurlyWood => Self::CadetBlue,
            Self::CadetBlue => Self::Chartreuse,
            Self::Chartreuse => Self::Chocolate,
            Self::Chocolate => Self::Coral,
            Self::Coral => Self::CornFlowerBlue,
            Self::CornFlowerBlue => Self::CornSilk,
            Self::CornSilk => Self::Crimson,
            Self::Crimson => Self::Cyan,
            Self::Cyan => Self::DarkBlue,
            Self::DarkBlue => Self::DarkCyan,
            Self::DarkCyan => Self::DarkGoldenRod,
            Self::DarkGoldenRod => Self::DarkGray,
            Self::DarkGray => Self::DarkGreen,
            Self::DarkGreen => Self::DarkGrey,
            Self::DarkGrey => Self::DarkKhaki,
            Self::DarkKhaki => Self::DarkMagenta,
            Self::DarkMagenta => Self::DarkOliveGreen,
            Self::DarkOliveGreen => Self::DarkOrange,
            Self::DarkOrange => Self::DarkOrchid,
            Self::DarkOrchid => Self::DarkRed,
            Self::DarkRed => Self::DarkSalmon,
            Self::DarkSalmon => Self::DarkSeaGreen,
            Self::DarkSeaGreen => Self::DarkSlateBlue,
            Self::DarkSlateBlue => Self::DarkSlateGray,
            Self::DarkSlateGray => Self::DarkTurquoise,
            Self::DarkTurquoise => Self::DarkViolet,
            Self::DarkViolet => Self::DeepPink,
            Self::DeepPink => Self::DeepSkyBlue,
            Self::DeepSkyBlue => Self::DimGray,
            Self::DimGray => Self::DimGrey,
            Self::DimGrey => Self::DodgerBlue,
            Self::DodgerBlue => Self::Firebrick,
            Self::Firebrick => Self::FloralWhite,
            Self::FloralWhite => Self::ForestGreen,
            Self::ForestGreen => Self::Fuschia,
            Self::Fuschia => Self::Gainsboro,
            Self::Gainsboro => Self::GhostWhite,
            Self::GhostWhite => Self::Gold,
            Self::Gold => Self::GoldenRod,
            Self::GoldenRod => Self::Gray,
            Self::Gray => Self::Green,
            Self::Green => Self::GreenYellow,
            Self::GreenYellow => Self::Grey,
            Self::Grey => Self::Honeydew,
            Self::Honeydew => Self::HotPink,
            Self::HotPink => Self::IndianRed,
            Self::IndianRed => Self::Indigo,
            Self::Indigo => Self::Ivory,
            Self::Ivory => Self::Khaki,
            Self::Khaki => Self::Lavender,
            Self::Lavender => Self::LavenderBlush,
            Self::LavenderBlush => Self::LawnGreen,
            Self::LawnGreen => Self::LemonChiffon,
            Self::LemonChiffon => Self::LightBlue,
            Self::LightBlue => Self::LightCoral,
            Self::LightCoral => Self::LightCyan,
            Self::LightCyan => Self::LightGoldenRodYellow,
            Self::LightGoldenRodYellow => Self::LightGray,
            Self::LightGray => Self::LightGreen,
            Self::LightGreen => Self::LightGrey,
            Self::LightGrey => Self::LightPink,
            Self::LightPink => Self::LightSalmon,
            Self::LightSalmon => Self::LightSeaGreen,
            Self::LightSeaGreen => Self::LightSkyBlue,
            Self::LightSkyBlue => Self::LightSlateGray,
            Self::LightSlateGray => Self::LightSteelBlue,
            Self::LightSteelBlue => Self::LightYellow,
            Self::LightYellow => Self::Lime,
            Self::Lime => Self::LimeGreen,
            Self::LimeGreen => Self::Linen,
            Self::Linen => Self::Magenta,
            Self::Magenta => Self::Maroon,
            Self::Maroon => Self::MediumAquaMarine,
            Self::MediumAquaMarine => Self::MediumBlue,
            Self::MediumBlue => Self::MediumOrchid,
            Self::MediumOrchid => Self::MediumPurple,
            Self::MediumPurple => Self::MediumSeaGreen,
            Self::MediumSeaGreen => Self::MediumSlateBlue,
            Self::MediumSlateBlue => Self::MediumSpringGreen,
            Self::MediumSpringGreen => Self::MediumTurquoise,
            Self::MediumTurquoise => Self::MediumVioletRed,
            Self::MediumVioletRed => Self::MidnightBlue,
            Self::MidnightBlue => Self::MintCream,
            Self::MintCream => Self::MistyRose,
            Self::MistyRose => Self::Moccasin,
            Self::Moccasin => Self::NavajoWhite,
            Self::NavajoWhite => Self::Navy,
            Self::Navy => Self::OldLace,
            Self::OldLace => Self::Olive,
            Self::Olive => Self::OliveDrab,
            Self::OliveDrab => Self::Orange,
            Self::Orange => Self::OrangeRed,
            Self::OrangeRed => Self::Orchid,
            Self::Orchid => Self::PaleGoldenRod,
            Self::PaleGoldenRod => Self::PaleGreen,
            Self::PaleGreen => Self::PaleTurquoise,
            Self::PaleTurquoise => Self::PaleVioletRed,
            Self::PaleVioletRed => Self::PapayaWhip,
            Self::PapayaWhip => Self::PeachPuff,
            Self::PeachPuff => Self::Peru,
            Self::Peru => Self::Pink,
            Self::Pink => Self::Plum,
            Self::Plum => Self::PowderBlue,
            Self::PowderBlue => Self::Purple,
            Self::Purple => Self::Red,
            Self::Red => Self::RosyBrown,
            Self::RosyBrown => Self::RoyalBlue,
            Self::RoyalBlue => Self::SaddleBrown,
            Self::SaddleBrown => Self::Salmon,
            Self::Salmon => Self::SandyBrown,
            Self::SandyBrown => Self::SeaGreen,
            Self::SeaGreen => Self::SeaShell,
            Self::SeaShell => Self::Sienna,
            Self::Sienna => Self::Silver,
            Self::Silver => Self::SkyBlue,
            Self::SkyBlue => Self::SlateBlue,
            Self::SlateBlue => Self::SlateGray,
            Self::SlateGray => Self::Snow,
            Self::Snow => Self::SpringGreen,
            Self::SpringGreen => Self::SteelBlue,
            Self::SteelBlue => Self::Tan,
            Self::Tan => Self::Teal,
            Self::Teal => Self::Thistle,
            Self::Thistle => Self::Tomato,
            Self::Tomato => Self::Turquoise,
            Self::Turquoise => Self::Violet,
            Self::Violet => Self::Wheat,
            Self::Wheat => Self::White,
            Self::White => Self::WhiteSmoke,
            Self::WhiteSmoke => Self::Yellow,
            Self::Yellow => Self::YellowGreen,
            Self::YellowGreen => Self::AliceBlue,
        }
    }

    /// Get the RGB values of this color.
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Colors::Maroon => (128, 0, 0),
            Colors::DarkRed => (139, 0, 0),
            Colors::Brown => (165, 42, 42),
            Colors::Firebrick => (178, 34, 34),
            Colors::Crimson => (220, 20, 60),
            Colors::Red => (255, 0, 0),
            Colors::Tomato => (255, 99, 71),
            Colors::Coral => (255, 127, 80),
            Colors::IndianRed => (205, 92, 92),
            Colors::LightCoral => (240, 128, 128),
            Colors::DarkSalmon => (233, 150, 122),
            Colors::Salmon => (250, 128, 114),
            Colors::LightSalmon => (255, 160, 122),
            Colors::OrangeRed => (255, 69, 0),
            Colors::DarkOrange => (255, 140, 0),
            Colors::Orange => (255, 165, 0),
            Colors::Gold => (255, 215, 0),
            Colors::DarkGoldenRod => (184, 134, 11),
            Colors::GoldenRod => (218, 165, 32),
            Colors::PaleGoldenRod => (238, 232, 170),
            Colors::DarkKhaki => (189, 183, 107),
            Colors::Khaki => (240, 230, 140),
            Colors::Olive => (128, 128, 0),
            Colors::Yellow => (255, 255, 0),
            Colors::YellowGreen => (154, 205, 50),
            Colors::DarkOliveGreen => (85, 107, 47),
            Colors::OliveDrab => (107, 142, 35),
            Colors::LawnGreen => (124, 252, 0),
            Colors::Chartreuse => (127, 255, 0),
            Colors::GreenYellow => (173, 255, 47),
            Colors::DarkGreen => (0, 100, 0),
            Colors::Green => (0, 128, 0),
            Colors::ForestGreen => (34, 139, 34),
            Colors::Lime => (0, 255, 0),
            Colors::LimeGreen => (50, 205, 50),
            Colors::LightGreen => (144, 238, 144),
            Colors::PaleGreen => (152, 251, 152),
            Colors::DarkSeaGreen => (143, 188, 143),
            Colors::MediumSpringGreen => (0, 250, 154),
            Colors::SpringGreen => (0, 255, 127),
            Colors::SeaGreen => (46, 139, 87),
            Colors::MediumAquaMarine => (102, 205, 170),
            Colors::MediumSeaGreen => (60, 179, 113),
            Colors::LightSeaGreen => (32, 178, 170),
            Colors::DarkSlateGray => (47, 79, 79),
            Colors::Teal => (0, 128, 128),
            Colors::DarkCyan => (0, 139, 139),
            Colors::Aqua => (0, 255, 255),
            Colors::Cyan => (0, 255, 255),
            Colors::LightCyan => (224, 255, 255),
            Colors::DarkTurquoise => (0, 206, 209),
            Colors::Turquoise => (64, 224, 208),
            Colors::MediumTurquoise => (72, 209, 204),
            Colors::PaleTurquoise => (175, 238, 238),
            Colors::AquaMarine => (127, 255, 212),
            Colors::PowderBlue => (176, 224, 230),
            Colors::CadetBlue => (95, 158, 160),
            Colors::SteelBlue => (70, 130, 180),
            Colors::CornFlowerBlue => (100, 149, 237),
            Colors::DeepSkyBlue => (0, 191, 255),
            Colors::DodgerBlue => (30, 144, 255),
            Colors::LightBlue => (173, 216, 230),
            Colors::SkyBlue => (135, 206, 235),
            Colors::LightSkyBlue => (135, 206, 250),
            Colors::MidnightBlue => (25, 25, 112),
            Colors::Navy => (0, 0, 128),
            Colors::DarkBlue => (0, 0, 139),
            Colors::MediumBlue => (0, 0, 205),
            Colors::Blue => (0, 0, 255),
            Colors::RoyalBlue => (65, 105, 225),
            Colors::BlueViolet => (138, 43, 226),
            Colors::Indigo => (75, 0, 130),
            Colors::DarkSlateBlue => (72, 61, 139),
            Colors::SlateBlue => (106, 90, 205),
            Colors::MediumSlateBlue => (123, 104, 238),
            Colors::MediumPurple => (147, 112, 219),
            Colors::DarkMagenta => (139, 0, 139),
            Colors::DarkViolet => (148, 0, 211),
            Colors::DarkOrchid => (153, 50, 204),
            Colors::MediumOrchid => (186, 85, 211),
            Colors::Purple => (128, 0, 128),
            Colors::Thistle => (216, 191, 216),
            Colors::Plum => (221, 160, 221),
            Colors::Violet => (238, 130, 238),
            Colors::Magenta => (255, 0, 255),
            Colors::Fuschia => (255, 0, 255),
            Colors::Orchid => (218, 112, 214),
            Colors::MediumVioletRed => (199, 21, 133),
            Colors::PaleVioletRed => (219, 112, 147),
            Colors::DeepPink => (255, 20, 147),
            Colors::HotPink => (255, 105, 180),
            Colors::LightPink => (255, 182, 193),
            Colors::Pink => (255, 192, 203),
            Colors::AntiqueWhite => (250, 235, 215),
            Colors::Beige => (245, 245, 220),
            Colors::Bisque => (255, 228, 196),
            Colors::BlanchedAlmond => (255, 235, 205),
            Colors::Wheat => (245, 222, 179),
            Colors::CornSilk => (255, 248, 220),
            Colors::LemonChiffon => (255, 250, 205),
            Colors::LightGoldenRodYellow => (250, 250, 210),
            Colors::LightYellow => (255, 255, 224),
            Colors::SaddleBrown => (139, 69, 19),
            Colors::Sienna => (160, 82, 45),
            Colors::Chocolate => (210, 105, 30),
            Colors::Peru => (205, 133, 63),
            Colors::SandyBrown => (244, 164, 96),
            Colors::BurlyWood => (222, 184, 135),
            Colors::Tan => (210, 180, 140),
            Colors::RosyBrown => (188, 143, 143),
            Colors::Moccasin => (255, 228, 181),
            Colors::NavajoWhite => (255, 222, 173),
            Colors::PeachPuff => (255, 218, 185),
            Colors::MistyRose => (255, 228, 225),
            Colors::LavenderBlush => (255, 240, 245),
            Colors::Linen => (250, 240, 230),
            Colors::OldLace => (253, 245, 230),
            Colors::PapayaWhip => (255, 239, 213),
            Colors::SeaShell => (255, 245, 238),
            Colors::MintCream => (245, 255, 250),
            Colors::SlateGray => (112, 128, 144),
            Colors::LightSlateGray => (119, 136, 153),
            Colors::LightSteelBlue => (176, 196, 222),
            Colors::Lavender => (230, 230, 250),
            Colors::FloralWhite => (255, 250, 240),
            Colors::AliceBlue => (240, 248, 255),
            Colors::GhostWhite => (248, 248, 255),
            Colors::Honeydew => (240, 255, 240),
            Colors::Ivory => (255, 255, 240),
            Colors::Azure => (240, 255, 255),
            Colors::Snow => (255, 250, 250),
            Colors::Black => (0, 0, 0),
            Colors::DimGray => (105, 105, 105),
            Colors::DimGrey => (105, 105, 105),
            Colors::Gray => (128, 128, 128),
            Colors::Grey => (128, 128, 128),
            Colors::DarkGray => (169, 169, 169),
            Colors::DarkGrey => (169, 169, 169),
            Colors::Silver => (192, 192, 192),
            Colors::LightGray => (211, 211, 211),
            Colors::LightGrey => (211, 211, 211),
            Colors::Gainsboro => (220, 220, 220),
            Colors::WhiteSmoke => (245, 245, 245),
            Colors::White => (255, 255, 255),
        }
    }

    pub fn into_color(self) -> Color {
        let (r, g, b) = self.rgb();
        Color::from_rgb(r, g, b)
    }

    pub fn all() -> impl Iterator<Item = Self> {
        Self::AliceBlue.into_iter()
    }
}

impl IntoIterator for Colors {
    type Item = Self;

    type IntoIter = iter::ColorsIter;

    fn into_iter(self) -> Self::IntoIter {
        iter::ColorsIter::starting_with(self)
    }
}

pub mod iter {
    pub struct ColorsIter {
        current: Option<super::Colors>,
    }

    impl Default for ColorsIter {
        fn default() -> Self {
            Self {
                current: Some(Self::FIRST),
            }
        }
    }

    impl ColorsIter {
        const FIRST: super::Colors = super::Colors::AliceBlue;
        const LAST: super::Colors = super::Colors::YellowGreen;

        pub fn new() -> Self {
            Default::default()
        }

        pub fn starting_with(color: super::Colors) -> Self {
            Self {
                current: Some(color),
            }
        }
    }

    impl Iterator for ColorsIter {
        type Item = super::Colors;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.current.take();
            self.current = current.and_then(|c| {
                let next = c.next();
                if next == Self::FIRST {
                    None
                } else {
                    Some(next)
                }
            });

            current
        }
    }
}

impl ToColor for Colors {
    fn to_color(&self) -> Color {
        self.rgb().into()
    }
}

impl IntoAnsi for Colors {
    fn into_ansi(self) -> Ansi {
        Ansi {
            fg: Some(self.into_color()),
            bg: None,
            flags: AnsiFlags::empty(),
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self(rgb.0, rgb.1, rgb.2)
    }
}

impl ToColor for (u8, u8, u8) {
    fn to_color(&self) -> Color {
        Color(self.0, self.1, self.2)
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    /// Attempts to parse the given string as a hex string into a [`Color`].
    fn try_from(input: &str) -> Result<Self, ColorParseError> {
        Color::from_hex(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_convert_1() {
        let color1 = Color::from_hex("#FF0000").unwrap();
        let color2 = Color::from_hex("FF0000").unwrap();
        let color3 = Color::from_hex("#f00").unwrap();
        assert_eq!(color1.rgb(), (255, 0, 0));
        assert_eq!(color2.rgb(), (255, 0, 0));
        assert_eq!(color3.rgb(), (255, 0, 0));
    }

    #[test]
    #[should_panic]
    fn hex_convert_too_small_panics() {
        let _ = Color::from_hex("#FF00").unwrap();
    }

    #[test]
    #[should_panic]
    fn hex_convert_too_big_panics() {
        let _ = Color::from_hex("#FF00000").unwrap();
    }

    #[test]
    #[should_panic]
    fn hex_convert_bad_char_panics() {
        let _ = Color::from_hex("#FF000G").unwrap();
    }
}
