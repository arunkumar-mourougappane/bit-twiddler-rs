# Bit Twiddler

A cross-platform developer toolbox built with [Tauri v2](https://tauri.app) (Rust backend, native WebView UI).  
Install it once, use it everywhere — no Node.js or Electron runtime required.

## Install

```sh
cargo install bit-twiddler
```

Requires the system WebView:
- **macOS** — built-in (WKWebView)
- **Linux** — `libwebkit2gtk-4.1` (`apt install libwebkit2gtk-4.1-dev` on Debian/Ubuntu)
- **Windows** — WebView2 (ships with Windows 10/11; installer available from Microsoft)

## Tools included

| Category | Tools |
|---|---|
| **Encoding** | Base64, URL encode/decode, HTML entities, PEM inspector |
| **Hashing** | MD5 · SHA-1 · SHA-256 · SHA-512 (text or file with live watch) |
| **Numbers** | Bit manipulator, endianness converter, Q-format, C array generator |
| **Networking** | CIDR calculator, HTTP status codes, protocol reference |
| **Cryptography** | UUID generator, password generator, QR code |
| **Data formats** | JSON formatter, YAML ↔ JSON, SQL formatter, JWT decoder |
| **Text** | Case converter, string utilities, regex tester, Lorem Ipsum |
| **Dev reference** | Baud rates, MIME types, epoch converter, color picker, resistor calc |
| **Markup** | Markdown preview, COBS encoder, diff viewer |
| **System** | Cron expression helper, unit converter, type size limits |

## Usage

After `cargo install`, run:

```sh
bit-twiddler
```

The app opens in a native window. Navigate tools via the sidebar; use `Ctrl+K` (or `Cmd+K`) to search.

## Building from source

```sh
git clone https://github.com/arunkumar-mourougappane/bit-twiddler
cd bit-twiddler/src-tauri
cargo build --release
```

**Optional — rebuild Tailwind CSS** (requires the standalone Tailwind CLI binary next to `Cargo.toml`):

```sh
# macOS arm64 example
curl -sLo src-tauri/tailwindcss-macos-arm64 \
  https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
chmod +x src-tauri/tailwindcss-macos-arm64
cargo build --release   # build.rs picks it up automatically
```

The pre-generated `ui/styles.css` is committed, so the Tailwind step is skipped when the binary is absent.

## License

MIT — see [LICENSE](LICENSE).
