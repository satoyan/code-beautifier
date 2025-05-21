use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;
use code_beautifier::{generate_html_from_source, parse_args};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut code = String::new();
    // Read from stdin if there is input, otherwise use the default string
    if atty::isnt(atty::Stream::Stdin) {
        io::stdin().read_to_string(&mut code).expect("Failed to read from stdin");
    } else {
        code = r#"fn main() { println!(\"Hello, world!\"); }"#.to_string();
    }
    // Parse --copy-to-clipboard option
    let copy_to_clipboard = args.iter().any(|arg| arg == "--copy-to-clipboard");
    let opts = parse_args(&args);
    let html = generate_html_from_source(&code, &opts.language);

    let output_file = if opts.output_to_file {
        let mut file = File::create(&opts.output_filename)
            .expect("Failed to create output file");
        file.write_all(html.as_bytes())
            .expect("Failed to write HTML to file");
        opts.output_filename.clone()
    } else {
        // Write to a temp file in /tmp and open with default browser
        let tmpfile = "/tmp/code-beautifier-output.html";
        let mut file = File::create(tmpfile).expect("Failed to create temp output file");
        file.write_all(html.as_bytes()).expect("Failed to write HTML to temp file");
        // Open with default browser
        if let Err(e) = Command::new("xdg-open").arg(tmpfile).status() {
            eprintln!("[WARN] Could not open browser: {}", e);
        }
        tmpfile.to_string()
    };

    // Only run html2clip.js if --copy-to-clipboard is specified
    if copy_to_clipboard {
        if let Ok(status) = Command::new("node")
            .arg("html2clip.js")
            .arg(&output_file)
            .status() {
            if !status.success() {
                eprintln!('[WARN] Could not copy image to clipboard (node/html2clip.js failed)');
            }
        } else {
            eprintln!('[WARN] Could not run node/html2clip.js. Is Node.js installed?');
        }
    }
}
