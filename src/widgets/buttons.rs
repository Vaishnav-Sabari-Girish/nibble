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

        // Render the label centered inside the button
        if inner.height > 0 && inner.width > 0 {
            let label_len = self.label.len() as u16;

            // Center horizontally
            let x = inner.x + (inner.width.saturating_sub(label_len)) / 2;
            // Center vertically
            let y = inner.y + inner.height / 2;

            // Write each character of the label
            for(i, ch) in self.label.chars().enumerate() {
                let cell_x = x + i as u16;
                if cell_x < inner.right() && y < inner.bottom() {
                    buf[(cell_x, y)]
                        .set_char(ch)
                        .set_style(button_style);
                }
            }
        }
    }
}
