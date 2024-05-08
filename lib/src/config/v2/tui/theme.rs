#![allow(clippy::module_name_repetitions)]

use std::{fmt::Display, num::ParseIntError};

use serde::{Deserialize, Serialize};
use tuirealm::props::Color;

use crate::config::{
    v1::AlacrittyColor,
    yaml_theme::{YAMLTheme, YAMLThemeBright, YAMLThemeCursor, YAMLThemeNormal, YAMLThemePrimary},
};

// TODO: combine Theme & Color?

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)] // allow missing fields and fill them with the `..Self::default()` in this struct
pub struct ThemeColorWrap {
    pub colors: Colors,
    pub theme: ThemeColors,
}

impl ThemeColorWrap {
    fn get_color_from_theme(&self, color: ColorTermusic) -> Color {
        match color {
            ColorTermusic::Reset => Color::Reset,
            ColorTermusic::Foreground => self.theme.primary.foreground.into(),
            ColorTermusic::Background => self.theme.primary.background.into(),
            ColorTermusic::Black => self.theme.normal.black.into(),
            ColorTermusic::Red => self.theme.normal.red.into(),
            ColorTermusic::Green => self.theme.normal.green.into(),
            ColorTermusic::Yellow => self.theme.normal.yellow.into(),
            ColorTermusic::Blue => self.theme.normal.blue.into(),
            ColorTermusic::Magenta => self.theme.normal.magenta.into(),
            ColorTermusic::Cyan => self.theme.normal.cyan.into(),
            ColorTermusic::White => self.theme.normal.white.into(),
            ColorTermusic::LightBlack => self.theme.bright.black.into(),
            ColorTermusic::LightRed => self.theme.bright.red.into(),
            ColorTermusic::LightGreen => self.theme.bright.green.into(),
            ColorTermusic::LightYellow => self.theme.bright.yellow.into(),
            ColorTermusic::LightBlue => self.theme.bright.blue.into(),
            ColorTermusic::LightMagenta => self.theme.bright.magenta.into(),
            ColorTermusic::LightCyan => self.theme.bright.cyan.into(),
            ColorTermusic::LightWhite => self.theme.bright.white.into(),
        }
    }

    #[inline]
    pub fn library_foreground(&self) -> Color {
        self.get_color_from_theme(self.colors.library_foreground)
    }

    #[inline]
    pub fn library_background(&self) -> Color {
        self.get_color_from_theme(self.colors.library_background)
    }

    #[inline]
    pub fn library_highlight(&self) -> Color {
        self.get_color_from_theme(self.colors.library_highlight)
    }

    #[inline]
    pub fn library_border(&self) -> Color {
        self.get_color_from_theme(self.colors.library_border)
    }

    #[inline]
    pub fn playlist_foreground(&self) -> Color {
        self.get_color_from_theme(self.colors.playlist_foreground)
    }

    #[inline]
    pub fn playlist_background(&self) -> Color {
        self.get_color_from_theme(self.colors.playlist_background)
    }

    #[inline]
    pub fn playlist_highlight(&self) -> Color {
        self.get_color_from_theme(self.colors.playlist_highlight)
    }

    #[inline]
    pub fn playlist_border(&self) -> Color {
        self.get_color_from_theme(self.colors.playlist_border)
    }

    #[inline]
    pub fn progress_foreground(&self) -> Color {
        self.get_color_from_theme(self.colors.progress_foreground)
    }

    #[inline]
    pub fn progress_background(&self) -> Color {
        self.get_color_from_theme(self.colors.progress_background)
    }

    #[inline]
    pub fn progress_border(&self) -> Color {
        self.get_color_from_theme(self.colors.progress_border)
    }

    #[inline]
    pub fn lyric_foreground(&self) -> Color {
        self.get_color_from_theme(self.colors.lyric_foreground)
    }

    #[inline]
    pub fn lyric_background(&self) -> Color {
        self.get_color_from_theme(self.colors.lyric_background)
    }

    #[inline]
    pub fn lyric_border(&self) -> Color {
        self.get_color_from_theme(self.colors.lyric_border)
    }

    #[inline]
    pub fn important_popup_foreground(&self) -> Color {
        self.get_color_from_theme(self.colors.important_popup_foreground)
    }

    #[inline]
    pub fn important_popup_background(&self) -> Color {
        self.get_color_from_theme(self.colors.important_popup_background)
    }

    #[inline]
    pub fn important_popup_border(&self) -> Color {
        self.get_color_from_theme(self.colors.important_popup_border)
    }

    #[inline]
    pub fn fallback_foreground(&self) -> Color {
        self.get_color_from_theme(self.colors.fallback_foreground)
    }

    #[inline]
    pub fn fallback_background(&self) -> Color {
        self.get_color_from_theme(self.colors.fallback_background)
    }

    #[inline]
    pub fn fallback_highlight(&self) -> Color {
        self.get_color_from_theme(self.colors.fallback_highlight)
    }

    #[inline]
    pub fn fallback_border(&self) -> Color {
        self.get_color_from_theme(self.colors.fallback_border)
    }
}

/// All values correspond to the Theme's selected color for that
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum ColorTermusic {
    /// Reset to Terminal default (resulting color will depend on what context it is set)
    Reset = 0,
    Foreground = 1,
    Background = 2,
    Black = 3,
    Red = 4,
    Green = 5,
    Yellow = 6,
    Blue = 7,
    Magenta = 8,
    Cyan = 9,
    White = 10,
    LightBlack = 11,
    LightRed = 12,
    LightGreen = 13,
    LightYellow = 14,
    LightBlue = 15,
    LightMagenta = 16,
    LightCyan = 17,
    LightWhite = 18,
}

impl AsRef<str> for ColorTermusic {
    fn as_ref(&self) -> &str {
        match self {
            ColorTermusic::Reset => "reset",
            ColorTermusic::Foreground => "foreground",
            ColorTermusic::Background => "background",
            ColorTermusic::Black => "black",
            ColorTermusic::Red => "red",
            ColorTermusic::Green => "green",
            ColorTermusic::Yellow => "yellow",
            ColorTermusic::Blue => "blue",
            ColorTermusic::Magenta => "magenta",
            ColorTermusic::Cyan => "cyan",
            ColorTermusic::White => "white",
            ColorTermusic::LightBlack => "bright_black",
            ColorTermusic::LightRed => "bright_red",
            ColorTermusic::LightGreen => "bright_green",
            ColorTermusic::LightYellow => "bright_yellow",
            ColorTermusic::LightBlue => "bright_blue",
            ColorTermusic::LightMagenta => "bright_magenta",
            ColorTermusic::LightCyan => "bright_cyan",
            ColorTermusic::LightWhite => "bright_white",
        }
    }
}

impl ColorTermusic {
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

/// What color to use for specific things, will use the colors from the specified Theme
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Colors {
    /// Music Library foreground color (text)
    pub library_foreground: ColorTermusic,
    /// Music Library background color (background)
    pub library_background: ColorTermusic,
    /// Music Library border color (when focused)
    pub library_border: ColorTermusic,
    /// Music Library selected node highlight color
    pub library_highlight: ColorTermusic,

    /// Playlist foreground color (text)
    pub playlist_foreground: ColorTermusic,
    /// Playlist background color (text)
    pub playlist_background: ColorTermusic,
    /// Playlist border color (when focused)
    pub playlist_border: ColorTermusic,
    /// Playlist selected node highlight color
    pub playlist_highlight: ColorTermusic,

    /// Lyrics foreground color (text)
    pub lyric_foreground: ColorTermusic,
    /// Lyrics background color (background)
    pub lyric_background: ColorTermusic,
    /// Lyrics border color (when focused)
    pub lyric_border: ColorTermusic,

    /// Track Progressbar foreground color (text)
    pub progress_foreground: ColorTermusic,
    /// Track Progressbar background color (background)
    pub progress_background: ColorTermusic,
    /// Track Progressbar border (always)
    pub progress_border: ColorTermusic,

    /// Important Popup (like Error or Delete) foreground color (text)
    pub important_popup_foreground: ColorTermusic,
    /// Important Popup (like Error or Delete) background color (background)
    pub important_popup_background: ColorTermusic,
    /// Important Popup (like Error or Delete) border color (always)
    pub important_popup_border: ColorTermusic,

    // Generic is when there is no specific config entry for it, like the "AskQuit" popup
    /// Generic foreground color (text)
    pub fallback_foreground: ColorTermusic,
    /// Generic background color (background)
    pub fallback_background: ColorTermusic,
    /// Generic border color (always)
    pub fallback_border: ColorTermusic,
    /// Generic Highlight color
    pub fallback_highlight: ColorTermusic,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            library_foreground: ColorTermusic::Foreground,
            library_background: ColorTermusic::Reset,
            library_border: ColorTermusic::Blue,
            library_highlight: ColorTermusic::LightYellow,

            playlist_foreground: ColorTermusic::Foreground,
            playlist_background: ColorTermusic::Reset,
            playlist_border: ColorTermusic::Blue,
            playlist_highlight: ColorTermusic::LightYellow,

            lyric_foreground: ColorTermusic::Foreground,
            lyric_background: ColorTermusic::Reset,
            lyric_border: ColorTermusic::Blue,

            progress_foreground: ColorTermusic::LightBlack,
            progress_background: ColorTermusic::Reset,
            progress_border: ColorTermusic::Blue,

            important_popup_foreground: ColorTermusic::Red,
            important_popup_background: ColorTermusic::Reset,
            important_popup_border: ColorTermusic::Red,

            fallback_foreground: ColorTermusic::Foreground,
            fallback_background: ColorTermusic::Reset,
            fallback_border: ColorTermusic::Blue,
            fallback_highlight: ColorTermusic::LightYellow,
        }
    }
}

// TODO: consider upgrading this with "thiserror"
/// Error for when [`ThemeColor`] parsing fails
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeColorParseError {
    ParseIntError(ParseIntError),
    IncorrectLength(usize),
}

impl Display for ThemeColorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alternate = f.alternate();
        write!(
            f,
            "Failed to parse color because of {}",
            match self {
                Self::ParseIntError(v) =>
                    if alternate {
                        format!("{v:#}")
                    } else {
                        format!("{v}")
                    },
                Self::IncorrectLength(length) => format!("Incorrect length {length}, expected 6"),
            }
        )
    }
}

/// The rgb colors
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ThemeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ThemeColor {
    /// Convert from a prefix + 6 length string
    pub fn from_hex(val: &str) -> Result<Self, ThemeColorParseError> {
        let without_prefix = val.trim_start_matches('#');

        // not in a format we support
        if without_prefix.len() != 6 {
            return Err(ThemeColorParseError::IncorrectLength(without_prefix.len()));
        }

        let r = u8::from_str_radix(&without_prefix[0..=1], 16)
            .map_err(ThemeColorParseError::ParseIntError)?;
        let g = u8::from_str_radix(&without_prefix[2..=3], 16)
            .map_err(ThemeColorParseError::ParseIntError)?;
        let b = u8::from_str_radix(&without_prefix[4..=5], 16)
            .map_err(ThemeColorParseError::ParseIntError)?;

        Ok(Self { r, g, b })
    }

    /// Convert to hex prefix + 6 length string
    #[inline]
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl TryFrom<String> for ThemeColor {
    type Error = ThemeColorParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_hex(&value)
    }
}

impl TryFrom<&str> for ThemeColor {
    type Error = ThemeColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_hex(value)
    }
}

impl From<AlacrittyColor> for ThemeColor {
    fn from(value: AlacrittyColor) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<ThemeColor> for String {
    fn from(val: ThemeColor) -> Self {
        ThemeColor::to_hex(&val)
    }
}

impl From<ThemeColor> for Color {
    fn from(val: ThemeColor) -> Self {
        Color::Rgb(val.r, val.g, val.b)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)] // allow missing fields and fill them with the `..Self::default()` in this struct
pub struct ThemeColors {
    pub name: String,
    pub author: String,
    pub primary: ThemePrimary,
    pub cursor: ThemeCursor,
    pub normal: ThemeNormal,
    pub bright: ThemeBright,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            name: default_name(),
            author: default_author(),
            primary: ThemePrimary::default(),
            cursor: ThemeCursor::default(),
            normal: ThemeNormal::default(),
            bright: ThemeBright::default(),
        }
    }
}

// TODO: consider upgrading this with "thiserror"
/// Error for when [`ThemeColors`] parsing fails
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeColorsParseError {
    ThemeColor(ThemeColorParseError),
}

impl Display for ThemeColorsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alternate = f.alternate();
        write!(
            f,
            "Failed to parse Theme because of {}",
            match self {
                Self::ThemeColor(v) =>
                    if alternate {
                        format!("{v:#}")
                    } else {
                        format!("{v}")
                    },
            }
        )
    }
}

impl From<ThemeColorParseError> for ThemeColorsParseError {
    fn from(value: ThemeColorParseError) -> Self {
        Self::ThemeColor(value)
    }
}

impl TryFrom<YAMLTheme> for ThemeColors {
    type Error = ThemeColorsParseError;

    fn try_from(value: YAMLTheme) -> Result<Self, Self::Error> {
        let colors = value.colors;
        Ok(Self {
            name: colors.name,
            author: colors.author,
            primary: colors.primary.try_into()?,
            cursor: colors.cursor.try_into()?,
            normal: colors.normal.try_into()?,
            bright: colors.bright.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ThemePrimary {
    pub background: ThemeColor,
    pub foreground: ThemeColor,
}

impl Default for ThemePrimary {
    fn default() -> Self {
        Self {
            background: default_000(),
            foreground: default_fff(),
        }
    }
}

impl TryFrom<YAMLThemePrimary> for ThemePrimary {
    type Error = ThemeColorsParseError;

    fn try_from(value: YAMLThemePrimary) -> Result<Self, Self::Error> {
        Ok(Self {
            background: value.background.try_into()?,
            foreground: value.foreground.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)] // allow missing fields and fill them with the `..Self::default()` in this struct
pub struct ThemeCursor {
    pub text: ThemeColor,
    pub cursor: ThemeColor,
}

impl Default for ThemeCursor {
    fn default() -> Self {
        Self {
            text: default_fff(),
            cursor: default_fff(),
        }
    }
}

impl TryFrom<YAMLThemeCursor> for ThemeCursor {
    type Error = ThemeColorsParseError;

    fn try_from(value: YAMLThemeCursor) -> Result<Self, Self::Error> {
        Ok(Self {
            text: value.text.try_into()?,
            cursor: value.cursor.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)] // allow missing fields and fill them with the `..Self::default()` in this struct
pub struct ThemeNormal {
    pub black: ThemeColor,
    pub red: ThemeColor,
    pub green: ThemeColor,
    pub yellow: ThemeColor,
    pub blue: ThemeColor,
    pub magenta: ThemeColor,
    pub cyan: ThemeColor,
    pub white: ThemeColor,
}

impl Default for ThemeNormal {
    fn default() -> Self {
        Self {
            black: default_000(),
            red: ThemeColor::from_hex("#ff0000").unwrap(),
            green: ThemeColor::from_hex("#00ff00").unwrap(),
            yellow: ThemeColor::from_hex("#ffff00").unwrap(),
            blue: ThemeColor::from_hex("#0000ff").unwrap(),
            magenta: ThemeColor::from_hex("#ff00ff").unwrap(),
            cyan: ThemeColor::from_hex("#00ffff").unwrap(),
            white: default_fff(),
        }
    }
}

impl TryFrom<YAMLThemeNormal> for ThemeNormal {
    type Error = ThemeColorsParseError;

    fn try_from(value: YAMLThemeNormal) -> Result<Self, Self::Error> {
        Ok(Self {
            black: value.black.try_into()?,
            red: value.red.try_into()?,
            green: value.green.try_into()?,
            yellow: value.yellow.try_into()?,
            blue: value.blue.try_into()?,
            magenta: value.magenta.try_into()?,
            cyan: value.cyan.try_into()?,
            white: value.white.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)] // allow missing fields and fill them with the `..Self::default()` in this struct
pub struct ThemeBright {
    pub black: ThemeColor,
    pub red: ThemeColor,
    pub green: ThemeColor,
    pub yellow: ThemeColor,
    pub blue: ThemeColor,
    pub magenta: ThemeColor,
    pub cyan: ThemeColor,
    pub white: ThemeColor,
}

impl Default for ThemeBright {
    fn default() -> Self {
        Self {
            black: ThemeColor::from_hex("#777777").unwrap(),
            red: default_000(),
            green: default_000(),
            yellow: default_000(),
            blue: default_000(),
            magenta: default_000(),
            cyan: default_000(),
            white: default_000(),
        }
    }
}

impl TryFrom<YAMLThemeBright> for ThemeBright {
    type Error = ThemeColorsParseError;

    fn try_from(value: YAMLThemeBright) -> Result<Self, Self::Error> {
        Ok(Self {
            black: value.black.try_into()?,
            red: value.red.try_into()?,
            green: value.green.try_into()?,
            yellow: value.yellow.try_into()?,
            blue: value.blue.try_into()?,
            magenta: value.magenta.try_into()?,
            cyan: value.cyan.try_into()?,
            white: value.white.try_into()?,
        })
    }
}

#[inline]
fn default_name() -> String {
    "empty name".to_string()
}

#[inline]
fn default_author() -> String {
    "empty author".to_string()
}

#[inline]
fn default_000() -> ThemeColor {
    ThemeColor::from_hex("#000000").unwrap()
}

#[inline]
fn default_fff() -> ThemeColor {
    ThemeColor::from_hex("#FFFFFF").unwrap()
}

mod v1_interop {
    use super::{
        ColorTermusic, Colors, ThemeBright, ThemeColorWrap, ThemeColors, ThemeCursor, ThemeNormal,
        ThemePrimary,
    };
    use crate::config::v1;

    impl From<v1::ColorTermusic> for ColorTermusic {
        fn from(value: v1::ColorTermusic) -> Self {
            match value {
                v1::ColorTermusic::Reset => Self::Reset,
                v1::ColorTermusic::Foreground => Self::Foreground,
                v1::ColorTermusic::Background => Self::Background,
                v1::ColorTermusic::Black => Self::Black,
                v1::ColorTermusic::Red => Self::Red,
                v1::ColorTermusic::Green => Self::Green,
                v1::ColorTermusic::Yellow => Self::Yellow,
                v1::ColorTermusic::Blue => Self::Blue,
                v1::ColorTermusic::Magenta => Self::Magenta,
                v1::ColorTermusic::Cyan => Self::Cyan,
                v1::ColorTermusic::White => Self::White,
                v1::ColorTermusic::LightBlack => Self::LightBlack,
                v1::ColorTermusic::LightRed => Self::LightRed,
                v1::ColorTermusic::LightGreen => Self::LightGreen,
                v1::ColorTermusic::LightYellow => Self::LightYellow,
                v1::ColorTermusic::LightBlue => Self::LightBlue,
                v1::ColorTermusic::LightMagenta => Self::LightMagenta,
                v1::ColorTermusic::LightCyan => Self::LightCyan,
                v1::ColorTermusic::LightWhite => Self::LightWhite,
            }
        }
    }

    impl From<v1::StyleColorSymbol> for Colors {
        fn from(value: v1::StyleColorSymbol) -> Self {
            Self {
                library_foreground: value.library_foreground.into(),
                library_background: value.library_background.into(),
                library_border: value.library_border.into(),
                library_highlight: value.library_highlight.into(),

                playlist_foreground: value.playlist_foreground.into(),
                playlist_background: value.playlist_background.into(),
                playlist_border: value.playlist_border.into(),
                playlist_highlight: value.playlist_highlight.into(),

                lyric_foreground: value.lyric_foreground.into(),
                lyric_background: value.lyric_background.into(),
                lyric_border: value.lyric_border.into(),

                progress_foreground: value.progress_foreground.into(),
                progress_background: value.progress_background.into(),
                progress_border: value.progress_border.into(),

                important_popup_foreground: value.important_popup_foreground.into(),
                important_popup_background: value.important_popup_background.into(),
                important_popup_border: value.important_popup_border.into(),

                fallback_foreground: value.fallback_foreground.into(),
                fallback_background: value.fallback_background.into(),
                fallback_border: value.fallback_border.into(),
                fallback_highlight: value.fallback_highlight.into(),
            }
        }
    }

    impl From<v1::Alacritty> for ThemeColors {
        fn from(value: v1::Alacritty) -> Self {
            Self {
                name: value.name,
                author: value.author,
                primary: ThemePrimary {
                    background: value.background.into(),
                    foreground: value.foreground.into(),
                },
                cursor: ThemeCursor {
                    text: value.text.into(),
                    cursor: value.cursor.into(),
                },
                normal: ThemeNormal {
                    black: value.black.into(),
                    red: value.red.into(),
                    green: value.green.into(),
                    yellow: value.yellow.into(),
                    blue: value.blue.into(),
                    magenta: value.magenta.into(),
                    cyan: value.cyan.into(),
                    white: value.white.into(),
                },
                bright: ThemeBright {
                    black: value.light_black.into(),
                    red: value.light_red.into(),
                    green: value.light_green.into(),
                    yellow: value.light_yellow.into(),
                    blue: value.light_blue.into(),
                    magenta: value.light_magenta.into(),
                    cyan: value.light_cyan.into(),
                    white: value.light_white.into(),
                },
            }
        }
    }

    impl From<v1::StyleColorSymbol> for ThemeColorWrap {
        fn from(value: v1::StyleColorSymbol) -> Self {
            Self {
                theme: value.alacritty_theme.clone().into(),
                colors: value.into(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_convert_default_without_error() {
            let converted: ThemeColorWrap = v1::StyleColorSymbol::default().into();

            assert_eq!(
                converted.colors,
                Colors {
                    library_foreground: ColorTermusic::Foreground,
                    library_background: ColorTermusic::Reset,
                    library_border: ColorTermusic::Blue,
                    library_highlight: ColorTermusic::LightYellow,
                    playlist_foreground: ColorTermusic::Foreground,
                    playlist_background: ColorTermusic::Reset,
                    playlist_border: ColorTermusic::Blue,
                    playlist_highlight: ColorTermusic::LightYellow,
                    lyric_foreground: ColorTermusic::Foreground,
                    lyric_background: ColorTermusic::Reset,
                    lyric_border: ColorTermusic::Blue,
                    progress_foreground: ColorTermusic::LightBlack,
                    progress_background: ColorTermusic::Reset,
                    progress_border: ColorTermusic::Blue,

                    important_popup_foreground: ColorTermusic::Yellow,
                    important_popup_background: ColorTermusic::Reset,
                    important_popup_border: ColorTermusic::Yellow,

                    fallback_foreground: ColorTermusic::Foreground,
                    fallback_background: ColorTermusic::Reset,
                    fallback_border: ColorTermusic::Blue,
                    fallback_highlight: ColorTermusic::LightYellow
                }
            );

            assert_eq!(
                converted.theme,
                ThemeColors {
                    name: "default".into(),
                    author: "Larry Hao".into(),
                    primary: ThemePrimary {
                        background: "#101421".try_into().unwrap(),
                        foreground: "#fffbf6".try_into().unwrap()
                    },
                    cursor: ThemeCursor {
                        text: "#1E1E1E".try_into().unwrap(),
                        cursor: "#FFFFFF".try_into().unwrap()
                    },
                    normal: ThemeNormal {
                        black: "#2e2e2e".try_into().unwrap(),
                        red: "#eb4129".try_into().unwrap(),
                        green: "#abe047".try_into().unwrap(),
                        yellow: "#f6c744".try_into().unwrap(),
                        blue: "#47a0f3".try_into().unwrap(),
                        magenta: "#7b5cb0".try_into().unwrap(),
                        cyan: "#64dbed".try_into().unwrap(),
                        white: "#e5e9f0".try_into().unwrap()
                    },
                    bright: ThemeBright {
                        black: "#565656".try_into().unwrap(),
                        red: "#ec5357".try_into().unwrap(),
                        green: "#c0e17d".try_into().unwrap(),
                        yellow: "#f9da6a".try_into().unwrap(),
                        blue: "#49a4f8".try_into().unwrap(),
                        magenta: "#a47de9".try_into().unwrap(),
                        cyan: "#99faf2".try_into().unwrap(),
                        white: "#ffffff".try_into().unwrap()
                    }
                }
            );
        }
    }
}
