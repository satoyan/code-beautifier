pub struct CmdOptions {
    pub output_to_file: bool,
    pub output_filename: String,
    pub language: String,
}

/// Generates HTML with syntax highlighting for the given source code and language.
/// `language` should be a string like "C", "C++", "Java", "C#", "JavaScript", "TypeScript", "Dart", etc.
/// Returns a String containing the HTML code.
pub fn generate_html_from_source(source: &str, language: &str) -> String {
    let ps = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let ts = syntect::highlighting::ThemeSet::load_defaults();
    let theme = ts.themes.get("base16-ocean.dark").unwrap_or(&ts.themes["InspiredGitHub"]);
    let syntax = ps.find_syntax_by_token(language)
        .or_else(|| ps.find_syntax_by_name(language))
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let highlighted = syntect::html::highlighted_html_for_string(source, &ps, syntax, theme)
        .expect("Failed to generate HTML from source code");
    // Embed the template at compile time
    let template = include_str!("./template.html");
    template.replace("{{highlighted}}", &highlighted)
}

/// Prints the help message and exits the program.
pub fn print_help_and_exit() -> ! {
    eprintln!("Usage: code-beautifier [--language <LANG>] [--output <FILE>]\n\nOptions:\n  --language <LANG>   Set the language for syntax highlighting (default: Rust)\n  --output <FILE>     Write output HTML to <FILE> instead of stdout\n  -h, --help          Show this help message and exit");
    std::process::exit(1);
}

/// Parses command line arguments and returns CmdOptions
pub fn parse_args(args: &[String]) -> CmdOptions {
    let mut output_to_file = false;
    let mut output_filename = String::new();
    let mut language = String::from("Rust");
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => print_help_and_exit(),
            "--output" => {
                if i + 1 < args.len() {
                    output_to_file = true;
                    output_filename = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --output requires a filename argument.\n");
                    print_help_and_exit();
                }
            },
            "--language" => {
                if i + 1 < args.len() {
                    language = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --language requires a language name argument.\n");
                    print_help_and_exit();
                }
            },
            _ => {
                eprintln!("Error: Unrecognized option '{}'.\n", args[i]);
                print_help_and_exit();
            }
        }
        i += 1;
    }
    CmdOptions { output_to_file, output_filename, language }
}
