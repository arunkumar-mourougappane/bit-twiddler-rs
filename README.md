# Bit Twiddler

[![CI](https://github.com/arunkumar-mourougappane/bit-twiddler-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/arunkumar-mourougappane/bit-twiddler-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/bit-twiddler.svg)](https://crates.io/crates/bit-twiddler)

A cross-platform developer toolbox built with [Tauri v2](https://tauri.app) — Rust backend, native WebView UI, no Node.js or Electron runtime required.

## Install

**Via Cargo:**

```sh
cargo install bit-twiddler
bit-twiddler
```

**Via GitHub Releases:** download the installer for your platform from the [Releases](https://github.com/arunkumar-mourougappane/bit-twiddler-rs/releases) page.

### System requirements

| Platform | Requirement |
|---|---|
| macOS | macOS 11+ (WebKit built-in) |
| Linux | `libwebkit2gtk-4.1` — `sudo apt install libwebkit2gtk-4.1-dev` |
| Windows | WebView2 (ships with Windows 10/11) |

## Tools

| Category | Tools |
|---|---|
| **Security & Encoding** | Base64, URL encode/decode, HTML entities, Hash (MD5/SHA-1/256/512), JWT decoder, PEM inspector, Password generator |
| **Data Formats** | JSON formatter, YAML ↔ JSON converter, SQL formatter, HTML/XML beautifier |
| **Date & Time** | Epoch converter, Unit converter, Cron expression parser |
| **Networking** | CIDR calculator, HTTP status reference, MIME types |
| **Low-Level / Embedded** | Bit manipulator & masks, Endian swapper, CRC calculator, C-array formatter, Q-format (fixed-point), COBS framing, Baud rate error, Resistor color codes, Integer type limits, Protocol reference |
| **Development** | Regex tester, Diff viewer, QR code generator, Markdown preview, UUID generator |
| **Text & Content** | Case converter, String utilities, Text transformer, Naming helper, Lorem Ipsum, Color picker |

## Keyboard shortcuts

| Shortcut | Action |
|---|---|
| `/` | Focus sidebar search |
| `⌘ 1–9` / `Ctrl 1–9` | Jump to tool by position |

## Building from source

```sh
git clone https://github.com/arunkumar-mourougappane/bit-twiddler-rs.git
cd bit-twiddler-rs/src-tauri
cargo build --release
```

**Optional — rebuild Tailwind CSS** (requires the [standalone Tailwind CLI](https://github.com/tailwindlabs/tailwindcss/releases/latest) binary placed next to `Cargo.toml`):

```sh
# macOS arm64 example
curl -sLo src-tauri/tailwindcss-macos-arm64 \
  https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
chmod +x src-tauri/tailwindcss-macos-arm64
cargo build --release
```

The pre-generated `ui/styles.css` is committed, so the Tailwind step is skipped when the binary is absent.

## CI

| Job | Platforms |
|---|---|
| Format & Clippy | Linux x86_64 |
| Build & Test | Linux x86_64 · macOS arm64 · Windows x86_64 |

## License

MIT — see [LICENSE](LICENSE).
