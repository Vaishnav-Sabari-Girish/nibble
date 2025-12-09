# Nibble

A tool for glamorous shell scripts. Quick, inline TUI components built with Ratatui—no full-screen takeover, just small bites of interaction.

This tool was inspired by [`gum`](https://github.com/charmbracelet/gum)

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
