# Code Beautifier

A CLI tool to generate beautiful HTML (and PNG images) of your source code with syntax highlighting, inspired by ray.so.

## Features
- Syntax highlighting for many languages (via syntect)
- Beautiful, modern HTML output
- Copy code image as PNG directly to clipboard (no browser needed)
- Output to file or stdout

## Prerequisites
- **Rust** (for building the CLI)
- **Node.js** (for headless image/clipboard automation)
- **npm** (for installing Node.js dependencies)

### Node.js dependencies (install once):

```
npm install puppeteer clipboardy
```

## Usage

1. **Generate HTML from source code:**
   ```bash
   cargo run -- --language Rust --output output.html < your_code.rs
   ```
   Or, for other languages:
   ```bash
   cargo run -- --language Python --output output.html < your_code.py
   ```

2. **Copy code image to clipboard:**
   ```bash
   node html2clip.js output.html
   ```
   Or, just run the CLI (it will call the script automatically):
   ```bash
   cargo run -- --language Rust --output output.html < your_code.rs
   # The image will be copied to your clipboard automatically
   ```

## Keyboard Shortcut (in browser)
If you open the generated HTML in a browser, press **Ctrl+C** (or **Cmd+C** on Mac) to copy the code window as a PNG image to your clipboard.

## Notes
- The HTML template is embedded in the binary at build time.
- The Node.js script (`html2clip.js`) is required for headless clipboard automation.
- Works on Linux, macOS, and Windows (with Node.js and Chrome/Chromium).

---

Enjoy beautiful code sharing!
