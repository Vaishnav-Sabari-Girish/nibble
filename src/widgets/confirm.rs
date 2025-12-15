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
    #[arg(short = 'h', long, default_value = "3")]
    pub height: u16,

    /// Default button to be highlighted/selected
    #[arg(short = 'd', long, default_value = "true")]
    pub default: bool,

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
    let mut selected = args.default;

    let result = loop {
        terminal
            .draw(|frame| {
                if let Err(e) = render(frame, &args, selected) {
                    eprintln!("Render Error: {}", e);
                }
            })
                .map_err(|e| NibbleError::RenderError(e.to_string()))?;
    };

    Ok(())
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
}
