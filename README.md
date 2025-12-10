# Nibble

A tool for glamorous shell scripts. Quick, inline TUI components built with Ratatui—no full-screen takeover, just small bites of interaction.

This tool was inspired by [`gum`](https://github.com/charmbracelet/gum)

[![Crates.io](https://img.shields.io/crates/v/nibble-rs.svg)](https://crates.io/crates/nibble-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-APACHE)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.70.0-blue.svg)](https://rust-lang.org)

## Installation 

```bash
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
