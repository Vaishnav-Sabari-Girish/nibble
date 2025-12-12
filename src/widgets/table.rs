use crate::{
    error::{NibbleError, Result},
    style::StyleConfig,
    tui,
};
use clap::Args;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::Constraint,
    style::Style,
    widgets::{Block, Borders, Row, Table as RatatuiTable},
    Frame,
};
use std::fs;

#[derive(Args, Debug)]
pub struct TableArgs {
    /// Table data as comma-separated values (use ';' for rows, ',' for columns)
    /// Example: "Name,Age,City;Alice,30,NYC;Bob,25,LA"
    #[arg(short, long, conflicts_with = "file")]
    pub data: Option<String>,

    /// Path to data file (CSV or JSON)
    #[arg(short, long, conflicts_with = "data")]
    pub file: Option<String>,

    /// Column headers (comma-separated). If not provided, first row is used as header
    #[arg(long)]
    pub headers: Option<String>,

    /// Title of the table block
    #[arg(short, long, default_value = "")]
    pub title: String,

    /// Height of the table in lines
    #[arg(long, default_value = "10")]
    pub height: u16,

    /// Column widths as percentages (comma-separated, must sum to 100 or less)
    /// Example: "30,40,30" for 3 columns
    #[arg(short = 'w', long)]
    pub widths: Option<String>,

    /// Separator character for inline data (default: ';' for rows, ',' for columns)
    #[arg(long, default_value = ";")]
    pub row_separator: String,

    #[arg(long, default_value = ",")]
    pub col_separator: String,

    /// Highlight header row
    #[arg(long)]
    pub highlight_header: bool,

    #[command(flatten)]
    pub style: StyleConfig,
}

pub fn run(args: TableArgs) -> anyhow::Result<()> {
    // Validate args
    if args.height == 0 {
        return Err(
            NibbleError::InvalidDimensions("Height must be greater than 0".to_string()).into(),
        );
    }

    if args.data.is_none() && args.file.is_none() {
        return Err(NibbleError::ConfigError(
            "Either --data or --file must be provided".to_string(),
        )
        .into());
    }

    // Parse table data
    let table_data = parse_table_data(&args)?;

    if table_data.is_empty() {
        return Err(NibbleError::ConfigError("Table data is empty".to_string()).into());
    }

    let mut terminal = tui::init_inline(args.height)?;

    let result = loop {
        terminal
            .draw(|frame| {
                if let Err(e) = render(frame, &args, &table_data) {
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

fn parse_table_data(args: &TableArgs) -> Result<Vec<Vec<String>>> {
    if let Some(ref file_path) = args.file {
        parse_file(file_path)
    } else if let Some(ref data) = args.data {
        parse_inline_data(data, &args.row_separator, &args.col_separator)
    } else {
        Err(NibbleError::ConfigError(
            "No data source provided".to_string(),
        ))
    }
}

fn parse_file(path: &str) -> Result<Vec<Vec<String>>> {
    let content = fs::read_to_string(path)
        .map_err(|e| NibbleError::ConfigError(format!("Failed to read file: {}", e)))?;

    if path.ends_with(".json") {
        parse_json(&content)
    } else if path.ends_with(".csv") {
        parse_csv(&content)
    } else {
        // Try CSV by default
        parse_csv(&content)
    }
}

fn parse_csv(content: &str) -> Result<Vec<Vec<String>>> {
    let mut rows = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let row: Vec<String> = line
            .split(',')
            .map(|cell| cell.trim().to_string())
            .collect();

        rows.push(row);
    }

    if rows.is_empty() {
        return Err(NibbleError::ConfigError("CSV file is empty".to_string()));
    }

    Ok(rows)
}

fn parse_json(content: &str) -> Result<Vec<Vec<String>>> {
    // Parse JSON array of objects or array of arrays
    let json: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| NibbleError::ConfigError(format!("Invalid JSON: {}", e)))?;

    match json {
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                return Err(NibbleError::ConfigError("JSON array is empty".to_string()));
            }

            // Check if it's array of arrays or array of objects
            if let Some(first) = arr.first() {
                if first.is_array() {
                    // Array of arrays
                    parse_json_array_of_arrays(&arr)
                } else if first.is_object() {
                    // Array of objects
                    parse_json_array_of_objects(&arr)
                } else {
                    Err(NibbleError::ConfigError(
                        "JSON must be array of arrays or array of objects".to_string(),
                    ))
                }
            } else {
                Err(NibbleError::ConfigError("JSON array is empty".to_string()))
            }
        }
        _ => Err(NibbleError::ConfigError(
            "JSON root must be an array".to_string(),
        )),
    }
}

fn parse_json_array_of_arrays(arr: &[serde_json::Value]) -> Result<Vec<Vec<String>>> {
    let mut rows = Vec::new();

    for item in arr {
        if let serde_json::Value::Array(row_arr) = item {
            let row: Vec<String> = row_arr
                .iter()
                .map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => "null".to_string(),
                    _ => v.to_string(),
                })
                .collect();
            rows.push(row);
        }
    }

    Ok(rows)
}

fn parse_json_array_of_objects(arr: &[serde_json::Value]) -> Result<Vec<Vec<String>>> {
    let mut rows = Vec::new();

    // Extract headers from first object
    if let Some(serde_json::Value::Object(first_obj)) = arr.first() {
        let headers: Vec<String> = first_obj.keys().cloned().collect();
        rows.push(headers.clone());

        // Extract values for each object
        for item in arr {
            if let serde_json::Value::Object(obj) = item {
                let row: Vec<String> = headers
                    .iter()
                    .map(|key| {
                        obj.get(key)
                            .map(|v| match v {
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                serde_json::Value::Null => "null".to_string(),
                                _ => v.to_string(),
                            })
                            .unwrap_or_default()
                    })
                    .collect();
                rows.push(row);
            }
        }
    }

    Ok(rows)
}

fn parse_inline_data(data: &str, row_sep: &str, col_sep: &str) -> Result<Vec<Vec<String>>> {
    let mut rows = Vec::new();

    for row_str in data.split(row_sep) {
        if row_str.trim().is_empty() {
            continue;
        }

        let row: Vec<String> = row_str
            .split(col_sep)
            .map(|cell| cell.trim().to_string())
            .collect();

        rows.push(row);
    }

    if rows.is_empty() {
        return Err(NibbleError::ConfigError("Inline data is empty".to_string()));
    }

    Ok(rows)
}

fn render(frame: &mut Frame, args: &TableArgs, data: &[Vec<String>]) -> Result<()> {
    let area = frame.area();

    if data.is_empty() {
        return Ok(());
    }

    // Determine number of columns
    let num_cols = data.iter().map(|row| row.len()).max().unwrap_or(0);

    // Parse column widths or use equal distribution
    let widths = if let Some(ref width_str) = args.widths {
        parse_widths(width_str, num_cols)?
    } else {
        // Equal distribution
        vec![Constraint::Percentage((100 / num_cols as u16).max(1)); num_cols]
    };

    // Separate headers and rows
    let (header_data, row_data) = if let Some(ref custom_headers) = args.headers {
        let headers: Vec<String> = custom_headers
            .split(',')
            .map(|h| h.trim().to_string())
            .collect();
        (headers, data.to_vec())
    } else if data.len() > 1 {
        // Use first row as headers
        (data[0].clone(), data[1..].to_vec())
    } else {
        // No separate headers
        (vec![], data.to_vec())
    };

    let text_style = args.style.text_style()?;

    // Create header row
    let header_style = if args.highlight_header {
        text_style.add_modifier(ratatui::style::Modifier::BOLD)
    } else {
        text_style
    };

    let header_cells: Vec<_> = header_data.iter().map(|h| h.as_str()).collect();
    let header = Row::new(header_cells).style(header_style);

    // Create data rows
    let rows: Vec<Row> = row_data
        .iter()
        .map(|row| {
            let cells: Vec<_> = row.iter().map(|c| c.as_str()).collect();
            Row::new(cells).style(text_style)
        })
        .collect();

    // Create table
    let mut table = RatatuiTable::new(rows, widths).header(header);

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
        table = table.block(block);
    }

    frame.render_widget(table, area);
    Ok(())
}

fn parse_widths(width_str: &str, num_cols: usize) -> Result<Vec<Constraint>> {
    let widths: std::result::Result<Vec<u16>, _> = width_str
        .split(',')
        .map(|w| {
            w.trim()
                .parse::<u16>()
                .map_err(|_| NibbleError::ConfigError(format!("Invalid width value: {}", w)))
        })
        .collect();

    let widths = widths?;

    if widths.len() != num_cols {
        return Err(NibbleError::ConfigError(format!(
            "Number of widths ({}) doesn't match number of columns ({})",
            widths.len(),
            num_cols
        )));
    }

    let sum: u16 = widths.iter().sum();
    if sum > 100 {
        return Err(NibbleError::ConfigError(format!(
            "Column widths sum to {}%, must be 100% or less",
            sum
        )));
    }

    Ok(widths
        .iter()
        .map(|&w| Constraint::Percentage(w))
        .collect())
}
