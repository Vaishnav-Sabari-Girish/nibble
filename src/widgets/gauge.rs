use crate::{
    error::{NibbleError, Result},
    style::StyleConfig,
    tui,
};
use clap::Args;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Gauge as RatatuiGauge},
    Frame,
};
use std::time::{Duration, Instant};

#[derive(Args, Debug)]
pub struct GaugeArgs {
    /// Target value (0-100) - gauge will animate from 0 to this value
    #[arg(short = 'v', long, default_value = "0")]
    pub value: u16,

    /// Label text to display on the gauge
    #[arg(short, long, default_value = "")]
    pub label: String,

    /// Title of the gauge block
    #[arg(short, long, default_value = "")]
    pub title: String,

    /// Height of the gauge in lines
    #[arg(long, default_value = "3")]
    pub height: u16,

    /// How fast to update in milliseconds (time between each increment)
    #[arg(long, default_value = "50")]
    pub time: u64,

    /// Use percentage symbol
    #[arg(short, long)]
    pub percentage: bool,

    #[command(flatten)]
    pub style: StyleConfig,
}

pub fn run(args: GaugeArgs) -> anyhow::Result<()> {
    // Validate args
    if args.height == 0 {
        return Err(NibbleError::InvalidDimensions("Height must be greater than 0".to_string()).into());
    }

    if args.value > 100 {
        return Err(NibbleError::InvalidDimensions("Value must be between 0 and 100".to_string()).into());
    }

    if args.time == 0 {
        return Err(NibbleError::InvalidDimensions("Time must be greater than 0".to_string()).into());
    }

    let mut terminal = tui::init_inline(args.height)?;

    // Always animate from 0 to target value
    let mut current_value = 0u16;
    let target_value = args.value;
    let update_interval = Duration::from_millis(args.time);
    let mut last_update = Instant::now();

    loop {
        // Render current state
        terminal
            .draw(|frame| {
                if let Err(e) = render(frame, &args, current_value) {
                    eprintln!("Render error: {}", e);
                }
            })
            .map_err(|e| NibbleError::RenderError(e.to_string()))?;

        // Check if we've reached the target
        if current_value >= target_value {
            // Wait for user input after reaching target
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => break,
                    _ => {}
                }
            }
        } else {
            // Check for events with timeout
            if event::poll(update_interval)?
            && let Event::Key(key) = event::read()?
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                tui::restore()?;
                return Ok(());
            }

            // Update progress if enough time has passed
            if last_update.elapsed() >= update_interval {
                current_value = (current_value + 1).min(target_value);
                last_update = Instant::now();
            }
        }
    }

    tui::restore()?;
    Ok(())
}

fn render(frame: &mut Frame, args: &GaugeArgs, current_value: u16) -> Result<()> {
    let area = frame.area();

let gauge_style = args.style.gauge_style()?;

// Get the gauge color (foreground color from style)
let gauge_color = if let Some(ref color_str) = args.style.fg {
    crate::style::parse_color(color_str)?
} else if let Some(ref color_str) = args.style.border_color {
    crate::style::parse_color(color_str)?
} else {
    Color::Cyan // Default gauge color
};

// Invert text color for better visibility
// If gauge is filled, text should be dark; if empty, text should be light
let label_style = Style::default().fg(invert_color(gauge_color));

let label = if args.label.is_empty() {
    if args.percentage {
        format!("{}%", current_value)
    } else {
        format!("{}/100", current_value)
    }
} else {
    // Show current value in custom label
    if args.percentage {
        format!("{} {}%", args.label, current_value)
    } else {
        format!("{} {}/100", args.label, current_value)
    }
};

let mut gauge = RatatuiGauge::default()
    .gauge_style(gauge_style)
    .label(ratatui::text::Span::styled(label, label_style))
    .percent(current_value);

// Add block if title or border is specified
if !args.title.is_empty() || args.style.border != "none" {
    let border_type = args.style.border_type()?;
    let border_style = args.style.border_style()?;
    let block = Block::default()
        .title(args.title.as_str())
        .borders(if args.style.border == "none" {
            Borders::NONE
        } else {
                Borders::ALL
            })
        .border_type(border_type)
        .border_style(border_style);
    gauge = gauge.block(block);
}

frame.render_widget(gauge, area);
Ok(())
}

/// Invert color for better contrast
fn invert_color(color: Color) -> Color {
    match color {
        // Light colors -> dark text
        Color::White | Color::LightRed | Color::LightGreen | Color::LightBlue
        | Color::LightYellow | Color::LightCyan | Color::LightMagenta => Color::Black,

        // Dark colors -> light text
        Color::Black | Color::DarkGray => Color::White,

        // Medium colors -> use opposite
        Color::Red => Color::White,
        Color::Green => Color::Black,
        Color::Blue => Color::White,
        Color::Yellow => Color::Black,
        Color::Cyan => Color::Black,
        Color::Magenta => Color::White,
        Color::Gray => Color::Black,

        // Default
        _ => Color::Black,
    }
}
