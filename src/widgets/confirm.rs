use crate::{error::NibbleError, style::StyleConfig, tui};
use clap::Args;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::widgets::Widget;
use ratatui::{
    layout::{Constraint, Layout, Alignment},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::buttons::Button;

#[derive(Args, Debug)]
pub struct ConfirmArgs {
    /// Text for confirmation (Optional)
    #[arg(short = 't', long, default_value = "Are you sure ?")]
    pub text: String,

    /// Affirmative option Text
    #[arg(short = 'a', long, default_value = "Yes")]
    pub affirmative: String,

    /// Negative option text
    #[arg(short = 'n', long, default_value = "No")]
    pub negative: String,

    /// Button height
    #[arg(short = 'h', long, default_value = "5")]
    pub height: u16,

    /// Default button to be highlighted/selected ("true", "false")
    #[arg(long, default_value = "false")]
    pub default_no: bool,

    #[command(flatten)]
    pub style: StyleConfig
}

pub fn run(args: ConfirmArgs) -> anyhow::Result<()> {
    // Validate args
    if args.height == 0 {
        return Err(
            NibbleError::InvalidDimensions("Height must be greater than 0".to_string()).into(),
        );
    }

    let mut terminal = tui::init_inline(args.height)?;
    let mut selected = !args.default_no;

    let result = loop {
        terminal
            .draw(|frame| {
                if let Err(e) = render(frame, &args, selected) {
                    eprintln!("Render Error: {}", e);
                }
            })
            .map_err(|e| NibbleError::RenderError(e.to_string()))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                // Toggle selection
                KeyCode::Left | KeyCode::Char('h') => selected = true,
                KeyCode::Right | KeyCode::Char('l') => selected = false,
                KeyCode::Tab => selected = !selected,

                // Quick selection
                KeyCode::Char('y') |  KeyCode::Char('Y') => {
                    break true;
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    break false;
                }

                // Submit current selection
                KeyCode::Enter => {
                    break selected;
                }

                // Cancel
                KeyCode::Esc | KeyCode::Char('q') => {
                    break false;
                }

                KeyCode::Char('c') if key.modifiers.contains(ratatui::crossterm::event::KeyModifiers::CONTROL) => {
                    break false;
                }

                _ => {}
            }
        }

    };

    terminal.clear()?;
    tui::restore()?;

    if result {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn render(frame: &mut Frame, args: &ConfirmArgs, selected: bool) -> crate::error::Result<()> {
    let area = frame.area();
    let text_style = args.style.text_style()?;

    // Create main layout 
    let chunks = Layout::vertical([
        Constraint::Length(1),    // Question text
        Constraint::Length(1),    // Spacing
        Constraint::Length(3),    // Buttons area
    ])
        .split(area);

    let question = Paragraph::new(args.text.as_str())
        .alignment(Alignment::Center)
        .style(text_style);

    frame.render_widget(question, chunks[0]);

    // Create button layout (centered with fixed width)
    let button_width = args.affirmative.len().max(args.negative.len()) as u16 + 4;
    // +4 is for borders and padding
    let total_width = button_width * 2 + 2;     // Two buttons + gap

    let button_area = centered_rect(chunks[2], total_width, 3);

    let button_chunks = Layout::horizontal([
        Constraint::Length(button_width),
        Constraint::Length(2),            // Gap
        Constraint::Length(button_width),
    ])
        .split(button_area);

    // Render yes button
    let yes_button = Button::new(&args.affirmative)
        .selected(selected)
        .style(text_style);

    frame.render_widget(yes_button, button_chunks[0]);

    // Render No button
    let no_button = Button::new(&args.negative)
        .selected(!selected)
        .style(text_style);
    frame.render_widget(no_button, button_chunks[2]);

    Ok(())
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect { 
        x, 
        y, 
        width: width.min(area.width),
        height: height.min(area.height)
    }
}
