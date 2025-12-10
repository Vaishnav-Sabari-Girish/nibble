use crate::error::{NibbleError, Result};
use clap::Args;
use ratatui::{
    style::{Color, Modifier, Style as RatatuiStyle},
    widgets::BorderType,
};

#[derive(Args, Debug, Clone)]
pub struct StyleConfig {
    /// Border style (rounded, double, thick, plain, none)
    #[arg(long, default_value = "rounded")]
    pub border: String,

    /// Border color
    #[arg(long)]
    pub border_color: Option<String>,

    /// Foreground color
    #[arg(long)]
    pub fg: Option<String>,

    /// Background color
    #[arg(long)]
    pub bg: Option<String>,

    /// Text modifiers (bold, italic, underline, dim) - can be used multiple times
    #[arg(long)]
    pub modifier: Vec<String>,
}

impl StyleConfig {
    pub fn border_type(&self) -> Result<BorderType> {
        parse_border_type(&self.border)
    }

    pub fn border_style(&self) -> Result<RatatuiStyle> {
        let mut style = RatatuiStyle::default();
        
        if let Some(ref color) = self.border_color {
            style = style.fg(parse_color(color)?);
        }
        
        Ok(style)
    }

    pub fn text_style(&self) -> Result<RatatuiStyle> {
        let mut style = RatatuiStyle::default();
        
        if let Some(ref color) = self.fg {
            style = style.fg(parse_color(color)?);
        }
        
        if let Some(ref color) = self.bg {
            style = style.bg(parse_color(color)?);
        }
        
        for modifier in &self.modifier {
            style = style.add_modifier(parse_modifier(modifier)?);
        }
        
        Ok(style)
    }

    pub fn gauge_style(&self) -> Result<RatatuiStyle> {
        let mut style = RatatuiStyle::default();
        
        if let Some(ref color) = self.fg {
            style = style.fg(parse_color(color)?);
        } else if let Some(ref color) = self.border_color {
            // Fallback to border color for gauge
            style = style.fg(parse_color(color)?);
        }
        
        style = style.add_modifier(Modifier::BOLD);
        
        Ok(style)
    }
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            border: "rounded".to_string(),
            border_color: None,
            fg: None,
            bg: None,
            modifier: Vec::new(),
        }
    }
}

pub fn parse_color(color: &str) -> Result<Color> {
    match color.to_lowercase().as_str() {
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "blue" => Ok(Color::Blue),
        "yellow" => Ok(Color::Yellow),
        "cyan" => Ok(Color::Cyan),
        "magenta" => Ok(Color::Magenta),
        "white" => Ok(Color::White),
        "black" => Ok(Color::Black),
        "gray" | "grey" => Ok(Color::Gray),
        "dark_gray" | "dark_grey" => Ok(Color::DarkGray),
        "light_red" => Ok(Color::LightRed),
        "light_green" => Ok(Color::LightGreen),
        "light_blue" => Ok(Color::LightBlue),
        "light_yellow" => Ok(Color::LightYellow),
        "light_cyan" => Ok(Color::LightCyan),
        "light_magenta" => Ok(Color::LightMagenta),
        _ => Err(NibbleError::InvalidColor(format!(
            "Unknown color '{}'. Valid colors: red, green, blue, yellow, cyan, magenta, white, black, gray, dark_gray, light_*",
            color
        ))),
    }
}

pub fn parse_border_type(border: &str) -> Result<BorderType> {
    match border.to_lowercase().as_str() {
        "rounded" => Ok(BorderType::Rounded),
        "double" => Ok(BorderType::Double),
        "thick" => Ok(BorderType::Thick),
        "plain" => Ok(BorderType::Plain),
        "none" => Ok(BorderType::Plain), // Handled by Borders::NONE elsewhere
        _ => Err(NibbleError::InvalidBorderType(format!(
            "Unknown border type '{}'. Valid types: rounded, double, thick, plain, none",
            border
        ))),
    }
}

pub fn parse_modifier(modifier: &str) -> Result<Modifier> {
    match modifier.to_lowercase().as_str() {
        "bold" => Ok(Modifier::BOLD),
        "italic" => Ok(Modifier::ITALIC),
        "underline" | "underlined" => Ok(Modifier::UNDERLINED),
        "dim" => Ok(Modifier::DIM),
        "crossed_out" | "crossed" => Ok(Modifier::CROSSED_OUT),
        "slow_blink" | "blink" => Ok(Modifier::SLOW_BLINK),
        "rapid_blink" => Ok(Modifier::RAPID_BLINK),
        "reversed" | "reverse" => Ok(Modifier::REVERSED),
        "hidden" => Ok(Modifier::HIDDEN),
        _ => Err(NibbleError::ConfigError(format!(
            "Unknown modifier '{}'. Valid modifiers: bold, italic, underline, dim, crossed_out, blink, reversed, hidden",
            modifier
        ))),
    }
}
