use crate::{error::{NibbleError, Result}, style::StyleConfig, tui};
use clap::Args;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    widgets::{Block as RatatuiBlock, Borders, Padding},
    Frame,
};

#[derive(Args, Debug)]
pub struct BlockArgs {
    /// Title of the block
    #[arg(short, long, default_value = "")]
    pub title: String,

    /// Height of the block in lines
    #[arg(long, default_value = "5")]
    pub height: u16,

    /// Width as percentage of terminal (0-100)
    #[arg(short, long, default_value = "50")]
    pub width: u16,

    /// Padding inside the block
    #[arg(short, long, default_value = "1")]
    pub padding: u16,

    #[command(flatten)]
    pub style: StyleConfig,
}

pub fn run(args: BlockArgs) -> anyhow::Result<()> {
    // Validate args
    if args.height == 0 {
        return Err(NibbleError::InvalidDimensions(
            "Height must be greater than 0".to_string(),
        )
        .into());
    }

    if args.width > 100 {
        return Err(NibbleError::InvalidDimensions(
            "Width must be between 0 and 100".to_string(),
        )
        .into());
    }

    let mut terminal = tui::init_inline(args.height)?;

    let result = loop {
        terminal
            .draw(|frame| {
                if let Err(e) = render(frame, &args) {
                    eprintln!("Render error: {}", e);
                }
            })
            .map_err(|e| NibbleError::RenderError(e.to_string()))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => break Ok(()),
                _ => {}
            }
        }
    };

    tui::restore()?;
    result
}

fn render(frame: &mut Frame, args: &BlockArgs) -> Result<()> {
    let area = frame.area();

    let border_type = args.style.border_type()?;
    let border_style = args.style.border_style()?;

    let block = RatatuiBlock::default()
        .title(args.title.as_str())
        .borders(if args.style.border == "none" {
            Borders::NONE
        } else {
            Borders::ALL
        })
        .border_type(border_type)
        .border_style(border_style)
        .padding(Padding::uniform(args.padding));

    frame.render_widget(block, area);
    Ok(())
}
