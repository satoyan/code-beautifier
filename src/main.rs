use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::html::highlighted_html_for_string;

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

fn main() {
    let code = r#"fn main() { println!(\"Hello, world!\"); }"#;
    let html = generate_html_from_source(code, "Rust");
    println!("{}", html);
}
