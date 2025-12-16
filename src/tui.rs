use crate::error::{NibbleError, Result};
use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal, TerminalOptions, Viewport};
use std::io::{stdout, Stdout};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize terminal with inline viewport
pub fn init_inline(height: u16) -> Result<Tui> {
    if height == 0 {
        return Err(NibbleError::InvalidDimensions(
            "Height must be greater than 0".to_string(),
        ));
    }

    enable_raw_mode().map_err(|e| {
        NibbleError::TerminalInit(format!("Failed to enable raw mode: {}", e))
    })?;

    // Hide cursor during TUI rendering
    stdout()
        .execute(cursor::Hide)
        .map_err(|e| NibbleError::TerminalInit(format!("Failed to hide cursor: {}", e)))?;

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(height),
        },
    )
    .map_err(|e| NibbleError::TerminalInit(format!("Failed to create terminal: {}", e)))?;

    Ok(terminal)
}

/// Initialize terminal with fullscreen (alternate screen)
pub fn init_fullscreen() -> Result<Tui> {
    enable_raw_mode().map_err(|e| {
        NibbleError::TerminalInit(format!("Failed to enable raw mode: {}", e))
    })?;

    stdout()
        .execute(EnterAlternateScreen)
        .map_err(|e| NibbleError::TerminalInit(format!("Failed to enter alternate screen: {}", e)))?;

    stdout()
        .execute(cursor::Hide)
        .map_err(|e| NibbleError::TerminalInit(format!("Failed to hide cursor: {}", e)))?;

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)
        .map_err(|e| NibbleError::TerminalInit(format!("Failed to create terminal: {}", e)))?;

    Ok(terminal)
}

/// Restore terminal to normal mode
pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen).ok();
    // Show cursor again
    stdout()
        .execute(cursor::Show)
        .map_err(|e| NibbleError::TerminalInit(format!("Failed to show cursor: {}", e)))?;
    Ok(())
}
