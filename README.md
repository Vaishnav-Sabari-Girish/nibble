<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Nibble](#nibble)
  - [Changelog](#changelog)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Widgets](#widgets)
      - [Block](#block)
      - [Gauge](#gauge)
      - [Table](#table)
      - [Input](#input)
      - [Confirm](#confirm)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# Nibble

A tool for glamorous shell scripts. Quick, inline TUI components built with Ratatui—no full-screen takeover, just small bites of interaction.

This tool was inspired by [`gum`](https://github.com/charmbracelet/gum)

[![Crates.io](https://img.shields.io/crates/v/nibble-rs.svg)](https://crates.io/crates/nibble-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-APACHE)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.70.0-blue.svg)](https://rust-lang.org)

## Changelog 

To view the changelog go to [CHANGELOG.md](./CHANGELOG.md) for more details

## Installation 

``` bash
cargo install nibble-rs
```

## Usage 

### Widgets 

Currently `nibble` supports the following widgets

#### Block

```bash
nibble block --title "Block Widget" --border rounded --border-color cyan --height 7
```

Gives you this output

![block](https://vhs.charm.sh/vhs-MBt8IwfQgYwznfq5h72Rm.gif)

#### Gauge

```bash
nibble gauge -v 75 --title "Progress" --border double --fg green --modifier bold --time 120
```

Gives you this output 

![gauge](https://vhs.charm.sh/vhs-5RKyIF079btFrL1EFz948L.gif)

#### Table

Display tabular data inline with customizable styling and multiple data formats.

This below command 

```bash
nibble table --data "Name,Age,City;Alice,30,NYC;Bob,25,LA;Carol,28,SF" -t "Users" --highlight-header
```

Will output this 

![table](https://vhs.charm.sh/vhs-1Aql3bxeY6rs8auZSFcQbx.gif)

**Inline Data:**

```bash
nibble table --data "Name,Age,City;Alice,30,NYC;Bob,25,LA;Carol,28,SF" --title "Users" --highlight-header
```

**From CSV File:**

```bash
nibble table --file data.csv --title "CSV Data" --height 12 --border rounded --fg cyan
```

**From JSON File:**

```bash
nibble table --file users.json --title "Users" --highlight-header --border double
```

**With Custom Column Widths:**

```bash
nibble table -d "Product,Price,Stock;Laptop,999,50;Mouse,25,200" -w "50,25,25" -t "Inventory"
```

**Supported Formats:**
- **Inline data**: Use semicolons (`;`) for rows and commas (`,`) for columns
- **CSV files**: Standard comma-separated values
- **JSON files**: Array of objects `[{"name": "Alice", ...}]` or array of arrays `[["Name", "Age"], ["Alice", 30]]`

**Key Options:**
- `--data, -d`: Inline data string
- `--file, -f`: Path to CSV or JSON file
- `--headers`: Custom column headers (comma-separated)
- `--widths, -w`: Column widths as percentages (comma-separated, must sum to ≤100)
- `--highlight-header`: Bold the header row
- `--height`: Table height in lines (default: 10)
- `--title, -t`: Title for the table block

**Example JSON (array of objects):**

```json
[
  {"name": "Alice", "age": 30, "city": "NYC"},
  {"name": "Bob", "age": 25, "city": "LA"}
]
```

**Example JSON (array of arrays):**

```json
[
  ["Name", "Age", "City"],
  ["Alice", 30, "NYC"],
  ["Bob", 25, "LA"]
]
```

#### Input

Single-line text input field with support for prompts, placeholders, passwords, and character limits.

**Basic Input:**

```bash
nibble input --prompt "Name:" --placeholder "Enter your name"
```

[![basic input](https://asciinema.org/a/2SMRB0ZiHwErPCeXC9mqgVQlQ.svg)](https://asciinema.org/a/2SMRB0ZiHwErPCeXC9mqgVQlQ)

**Password Input:**

```bash
nibble input --prompt "Password:" --password --border double
```

[![pwd](https://asciinema.org/a/2SMRB0ZiHwErPCeXC9mqgVQlQ.svg)](https://asciinema.org/a/2SMRB0ZiHwErPCeXC9mqgVQlQ)

**With Character Limit and Counter:**

```bash
nibble input --prompt "Username:" --max-length 20 --show-count --border rounded --fg cyan
```

**Pre-filled Value:**

```bash
nibble input --prompt "Email:" --value "user@example.com" --show-count
```

**Styled Input:**

```bash
nibble input --title "User Input" --prompt "City:" --placeholder "New York" --border thick --fg green
```

**Key Options:**
- `--prompt, -r`: Label text displayed before the input field
- `--placeholder, -p`: Placeholder text shown when input is empty
- `--value, -v`: Initial/pre-filled value
- `--title, -t`: Title for the input block
- `--password`: Masks input with asterisks (does not print output)
- `--max-length, -m`: Character limit
- `--show-count, -c`: Display character counter
- `--height`: Widget height in lines (default: 3)

#### Confirm

Interactive confirmation prompt with Yes/No buttons. Returns exit code `0` for Yes and `1` for No.

**Basic Confirm**

```bash
# Default confirm
nibble confirm 
```

[![basic_confirm](https://asciinema.org/a/AMhbu8acmuu0hMIEf4SnWtjIc.svg)](https://asciinema.org/a/AMhbu8acmuu0hMIEf4SnWtjIc)

**Custom Text with Custom options**

```bash
# Custom Text
nibble confirm --text "Continue ?"

# Custom options
nibble confirm --text "Continue ?" --affirmative "Proceed" --negative "Cancel"
```

[![custom_confirm](https://asciinema.org/a/eWOAq5YWufMuZjkyJopa5xbg9.svg)](https://asciinema.org/a/eWOAq5YWufMuZjkyJopa5xbg9)

**Select No by default**

```bash
nibble confirm --text "Continue ?" --affirmative "Proceed" --negative "Cancel" --default-no
```

[![no_default](https://asciinema.org/a/whYMpn1pt7gIANXSCzesjDSsT.svg)](https://asciinema.org/a/whYMpn1pt7gIANXSCzesjDSsT)
