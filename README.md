# SkyDecode

SkyDecode is a terminal-based CLI tool written in Rust that fetches and decodes METAR weather data for multiple airports. It provides both a human-readable formatted table and the raw METAR strings for aviation enthusiasts, pilots, or developers who need quick weather information.

## Features

- Fetch METAR data for one or multiple airports simultaneously.
- Display decoded METAR data in a clean, CLI table.
- Optional `--raw` mode to show raw METAR strings instead of decoded data.
- Automatically adapts table layout to your terminal width.
- Lightweight and fast, written entirely in Rust.

## Installation

1. Make sure you have Rust installed. If not, install Rust from [rustup.rs](https://rustup.rs/).
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/SkyDecode.git
   ```
3. Navigate to the project folder:
   ```bash
   cd SkyDecode
   ```
4. Build and run:
   ```bash
   cargo run -- --ids LTFM,LTBA,EGLL
   ```
5. To show raw METAR data:
   ```bash
   cargo run -- --ids LTFM,LTBA,EGLL --raw
   ```

## Usage

```text
USAGE:
    SkyDecode --ids <ICAO_CODES> [--raw]

OPTIONS:
    --ids <ICAO_CODES>    Comma-separated list of airport ICAO codes (e.g., LTFM,LTBA)
    --raw                  Show raw METAR strings instead of decoded data
```

Example:

```bash
cargo run -- --ids LTFM,EGLL --raw
```

This will fetch METARs for LTFM and EGLL airports and print the raw METAR strings.

## Dependencies

- [reqwest](https://crates.io/crates/reqwest) – for HTTP requests
- [serde](https://crates.io/crates/serde) – for data serialization
- [tabled](https://crates.io/crates/tabled) – for terminal table formatting
- [terminal_size](https://crates.io/crates/terminal_size) – for dynamic terminal width adjustment

## License

This project is licensed under the **GPL-3.0 License**.

