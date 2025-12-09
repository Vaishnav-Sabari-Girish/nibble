use thiserror::Error;

#[derive(Error, Debug)]
pub enum NibbleError {
    #[error("Terminal initialization failed: {0}")]
    TerminalInit(String),

    #[error("Rendering failed: {0}")]
    RenderError(String),

    #[error("Invalid color: {0}")]
    InvalidColor(String),

    #[error("Invalid border type: {0}")]
    InvalidBorderType(String),

    #[error("Invalid dimensions: {0}")]
    InvalidDimensions(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Widget configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, NibbleError>;
