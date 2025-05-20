use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::html::highlighted_html_for_string;
use std::env;
use std::fs::File;
use std::io::Write;

/// Generates HTML with syntax highlighting for the given source code and language.
/// `language` should be a string like "C", "C++", "Java", "C#", "JavaScript", "TypeScript", "Dart", etc.
/// Returns a String containing the HTML code.
pub fn generate_html_from_source(source: &str, language: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    // Try to find the syntax by name or extension
    let syntax = ps.find_syntax_by_token(language)
        .or_else(|| ps.find_syntax_by_name(language))
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let theme = &ts.themes["InspiredGitHub"];
    highlighted_html_for_string(source, &ps, syntax, theme)
        .expect("Failed to generate HTML from source code")
}

/// Struct to hold parsed command line options
struct CmdOptions {
    output_to_file: bool,
    output_filename: String,
    language: String,
}

/// Parses command line arguments and returns CmdOptions
fn parse_args(args: &[String]) -> CmdOptions {
    let mut output_to_file = false;
    let mut output_filename = String::new();
    let mut language = String::from("Rust");
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--output" && i + 1 < args.len() {
            output_to_file = true;
            output_filename = args[i + 1].clone();
            i += 1;
        } else if args[i] == "--language" && i + 1 < args.len() {
            language = args[i + 1].clone();
            i += 1;
        }
        i += 1;
    }
    CmdOptions { output_to_file, output_filename, language }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = r#"fn main() { println!(\"Hello, world!\"); }"#;
    let opts = parse_args(&args);
    let html = generate_html_from_source(code, &opts.language);

    if opts.output_to_file {
        let mut file = File::create(&opts.output_filename)
            .expect("Failed to create output file");
        file.write_all(html.as_bytes())
            .expect("Failed to write HTML to file");
    } else {
        println!("{}", html);
    }
}
