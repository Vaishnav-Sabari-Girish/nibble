use crate::{error::NibbleError, style::StyleConfig, tui};
use clap::Args;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(Args, Debug)]
pub struct InputArgs {
    /// Placeholder text when input is empty
    #[arg(short, long, default_value = "")]
    pub placeholder: String,

    /// Initial value for the input
    #[arg(short = 'v', long, default_value = "")]
    pub value: String,

    /// Prompt text to display before the input
    #[arg(short = 'r', long, default_value = "")]
    pub prompt: String,

    /// Title of the input block
    #[arg(short, long, default_value = "")]
    pub title: String,

    /// Height of the input widget in lines
    #[arg(long, default_value = "3")]
    pub height: u16,

    /// Password mode (hide input with asterisks)
    #[arg(long)]
    pub password: bool,

    /// Character limit (max length)
    #[arg(short = 'm', long)]
    pub max_length: Option<usize>,

    /// Show character count
    #[arg(short = 'c', long)]
    pub show_count: bool,

    #[command(flatten)]
    pub style: StyleConfig,
}

pub fn run(args: InputArgs) -> anyhow::Result<()> {
    // Validate args
    if args.height == 0 {
        return Err(
            NibbleError::InvalidDimensions("Height must be greater than 0".to_string()).into(),
        );
    }

    let mut terminal = tui::init_inline(args.height)?;
    let mut input = Input::default().with_value(args.value.clone());

    let result = loop {
        terminal
            .draw(|frame| {
                if let Err(e) = render(frame, &args, &input) {
                    eprintln!("Render error: {}", e);
                }
            })
            .map_err(|e| NibbleError::RenderError(e.to_string()))?;

        if let Event::Key(key) = event::read()? {
            match handle_key_event(key, &mut input, &args) {
                InputAction::Continue => {}
                InputAction::Submit => {
                    break Some(input.value().to_string());
                }
                InputAction::Cancel => {
                    break None;
                }
            }
        }
    };

    // Clear and restore terminal FIRST
    terminal.clear()?;
    tui::restore()?;

    // THEN print the output (but NOT if it's a password)
    if let Some(value) = result {
        if !args.password {
            println!("{}", value);
        }
    }

    Ok(())
}

enum InputAction {
    Continue,
    Submit,
    Cancel,
}

fn handle_key_event(key: KeyEvent, input: &mut Input, args: &InputArgs) -> InputAction {
    match key.code {
        KeyCode::Enter => InputAction::Submit,
        KeyCode::Esc => InputAction::Cancel,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => InputAction::Cancel,
        _ => {
            // Check max length before allowing input
            if let Some(max) = args.max_length {
                if input.value().len() >= max
                    && !matches!(
                        key.code,
                        KeyCode::Backspace | KeyCode::Delete | KeyCode::Left | KeyCode::Right | KeyCode::Home | KeyCode::End
                    )
                {
                    return InputAction::Continue;
                }
            }

            // Handle the event with tui-input
            input.handle_event(&Event::Key(key));
            InputAction::Continue
        }
    }
}

fn render(frame: &mut Frame, args: &InputArgs, input: &Input) -> crate::error::Result<()> {
    let area = frame.area();

    // Create layout for prompt and input
    let (prompt_area, input_area) = if !args.prompt.is_empty() {
        let chunks =
            Layout::horizontal([Constraint::Length(args.prompt.len() as u16 + 2), Constraint::Min(1)])
                .split(area);
        (Some(chunks[0]), chunks[1])
    } else {
        (None, area)
    };

    let text_style = args.style.text_style()?;

    // Render prompt if present
    if let Some(prompt_rect) = prompt_area {
        let prompt_style = text_style.add_modifier(ratatui::style::Modifier::BOLD);
        let prompt = Paragraph::new(format!("{} ", args.prompt)).style(prompt_style);
        frame.render_widget(prompt, prompt_rect);
    }

    // Build the input line with cursor
    let cursor_pos = input.cursor();
    let mut spans = vec![];

    // Check if input is empty and we should show placeholder
    if input.value().is_empty() && !args.placeholder.is_empty() {
        // Show placeholder with cursor at start
        spans.push(Span::styled(
            " ",
            Style::default()
                .fg(ratatui::style::Color::DarkGray)
                .add_modifier(ratatui::style::Modifier::REVERSED),
        ));
        spans.push(Span::styled(
            args.placeholder.clone(),
            Style::default().fg(ratatui::style::Color::DarkGray),
        ));
    } else {
        // Get input value (mask if password mode)
        let display_value = if args.password {
            "*".repeat(input.value().len())
        } else {
            input.value().to_string()
        };

        let content_chars: Vec<char> = display_value.chars().collect();

        for (i, ch) in content_chars.iter().enumerate() {
            if i == cursor_pos {
                // Show cursor as reversed character
                spans.push(Span::styled(
                    ch.to_string(),
                    text_style.add_modifier(ratatui::style::Modifier::REVERSED),
                ));
            } else {
                spans.push(Span::styled(ch.to_string(), text_style));
            }
        }

        // If cursor is at the end, show block cursor
        if cursor_pos >= content_chars.len() {
            spans.push(Span::styled(
                " ",
                text_style.add_modifier(ratatui::style::Modifier::REVERSED),
            ));
        }
    }

    // Add character count if requested
    if args.show_count {
        let count_text = if let Some(max) = args.max_length {
            format!(" ({}/{})", input.value().len(), max)
        } else {
            format!(" ({})", input.value().len())
        };
        spans.push(Span::styled(
            count_text,
            Style::default().fg(ratatui::style::Color::DarkGray),
        ));
    }

    let input_widget = Paragraph::new(Line::from(spans));

    // Add block if title or border is specified
    let input_widget = if !args.title.is_empty() || args.style.border != "none" {
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
        input_widget.block(block)
    } else {
        input_widget
    };

    frame.render_widget(input_widget, input_area);
    Ok(())
}
