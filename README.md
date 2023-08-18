# Shodan Trends TUI

[![Crates.io](https://img.shields.io/crates/v/strend.svg)](https://crates.io/crates/strend)
[![License](https://img.shields.io/crates/l/mit)](./LICENSE)
[![Twitter](https://img.shields.io/twitter/follow/shodanhq.svg?logo=twitter)](https://twitter.com/shodanhq)

## Search and visualize Shodan historical data in the terminal.

``strend`` fetches data via the [Shodan Trends](https://trends.shodan.io) API, please check [docs](https://developer.shodan.io/api) for more information.

![Sample](sample.png)

## Installation

Grab the [latest release](https://github.com/thoongnv/trends-rs/releases) for your operating system or install it from [crates.io](https://crates.io/crates/strend).

```shell
cargo install strend
```

## Usage

The ``strend`` command can be launched with or without query.

```shell
$ strend
$ strend --query "product:nginx port:443" --facets country:10
$ strend --help
Search and visualize Shodan historical data in the terminal.

Usage: strend [OPTIONS] [COMMAND]

Commands:
  init  Initialize Shodan API key, grab it from https://account.shodan.io
  help  Print this message or the help of the given subcommand(s)

Options:
      --query <QUERY>    Search query used to search the historical database, e.g. "product:nginx port:443"
      --facets <FACETS>  A comma-separated list of properties to get summary information on, e.g. country:10
  -h, --help             Print help
  -V, --version          Print version
```

## Debugging

Our application rendered to `stderr`, so we could use `println!("dump variable: {:?}", variable);` in code and then pine the output to a log file.

```shell
$ cargo run > debug.log
$ tail -f debug.log
```
