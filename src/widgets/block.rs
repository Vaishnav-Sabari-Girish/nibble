use crate::{error::{NibbleError, Result}, tui};
use clap::Args;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::{Color, Style},
    widgets::{Block as RatatuiBlock, BorderType, Borders, Padding},
    Frame,
};

#[derive(Args, Debug)]
pub struct BlockArgs {
    /// Title of the block
    #[arg(short, long, default_value = "")]
    pub title: String,

    /// Border style (rounded, double, thick, plain)
    #[arg(short, long, default_value = "rounded")]
    pub border: String,

    /// Border color (red, green, blue, yellow, cyan, magenta, white)
    #[arg(short = 'c', long)]
    pub border_color: Option<String>,

    /// Height of the block in lines
    #[arg(long, default_value = "5")]
    pub height: u16,

    /// Width as percentage of terminal (0-100)
    #[arg(short, long, default_value = "50")]
    pub width: u16,

    /// Padding inside the block
    #[arg(short, long, default_value = "1")]
    pub padding: u16,
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

    let border_type = parse_border_type(&args.border)?;

    let border_style = if let Some(ref color_str) = args.border_color {
        Style::default().fg(parse_color(color_str)?)
    } else {
        Style::default()
    };

    let block = RatatuiBlock::default()
        .title(args.title.as_str())
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(border_style)
        .padding(Padding::uniform(args.padding));

    frame.render_widget(block, area);
    Ok(())
}

fn parse_color(color: &str) -> Result<Color> {
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
        _ => Err(NibbleError::InvalidColor(format!(
            "Unknown color '{}'. Valid colors: red, green, blue, yellow, cyan, magenta, white, black, gray",
            color
        ))),
    }
}

fn parse_border_type(border: &str) -> Result<BorderType> {
    match border.to_lowercase().as_str() {
        "rounded" => Ok(BorderType::Rounded),
        "double" => Ok(BorderType::Double),
        "thick" => Ok(BorderType::Thick),
        "plain" => Ok(BorderType::Plain),
        _ => Err(NibbleError::InvalidBorderType(format!(
            "Unknown border type '{}'. Valid types: rounded, double, thick, plain",
            border
        ))),
    }
}
