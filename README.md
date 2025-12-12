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
