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


pub struct Button<'a> {
    pub label: &'a str,
    pub selected: bool,
    style: Style,
}

impl<'a> Button<'a>  {
    pub fn new(label: &'a str) -> Self {
        Self  {
            label: label,
            selected: false,
            style: Style::default()
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for Button<'a>  {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Apply selected style (reversed) or normal style
        let button_style = if self.selected {
            self.style.add_modifier(Modifier::REVERSED)
        } else {
            self.style
        };

        // Button block with borders
        let block = Block::default()
            .borders(Borders::ALL)
            .style(button_style);

        // Render the block
        let inner = block.inner(area);
        block.render(area, buf);

        // Render the label centered inside the button
        let label = Line::from(Span::styled(self.label, button_style));

        // Center the text vertically and horizontally
        if inner.height > 0 && inner.width > 0 {
            let label_width = self.label.len() as u16;
            let x = inner.x + (inner.width.saturating_sub(label_width)) / 2;
            let y = inner.y + inner.height / 2;

            if x < inner.right() && y < inner.bottom() {
                buf.set_line(x, y, &label, label_width);
            }
        }
    }
}
