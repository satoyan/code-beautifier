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
    // Ray.so-inspired HTML template
    format!(r#"<!DOCTYPE html>
<html lang=\"en\" class=\"dark\" style=\"color-scheme:dark\">
<head>
<meta charset=\"utf-8\"/>
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"/>
<meta name=\"theme-color\" content=\"#181818\"/>
<title>Code Beautifier</title>
<meta name=\"description\" content=\"Turn your code into beautiful images. Choose from a range of syntax colors, hide or show the background, and toggle between a dark and light window.\"/>
<meta name=\"keywords\" content=\"generate, create, convert, source, code, snippet, image, picture, share, export\"/>
<style>
body {{
  min-height: 100vh;
  background: linear-gradient(135deg, #18181b 0%, #232526 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0;
  font-family: 'Geist Mono', 'Fira Mono', 'JetBrains Mono', 'Menlo', 'Monaco', 'Consolas', monospace;
}}
.code-window {{
  background: rgba(24,24,27,0.95);
  border-radius: 18px;
  box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
  padding: 2.5rem 2rem 2rem 2rem;
  max-width: 900px;
  min-width: 320px;
  width: 90vw;
  position: relative;
  border: 1.5px solid #232526;
}}
.window-bar {{
  height: 18px;
  background: linear-gradient(90deg, #ff5f56, #ffbd2e, #27c93f);
  border-radius: 10px 10px 0 0;
  margin-bottom: 1.2rem;
  width: 80px;
  position: absolute;
  left: 24px;
  top: 12px;
  opacity: 0.7;
}}
pre {{
  margin: 0;
  font-family: inherit;
  font-size: 1.15rem;
  line-height: 1.7;
  background: none !important;
  color: #eaeaea;
  overflow-x: auto;
  border-radius: 12px;
  padding: 1.2em 1em;
  box-shadow: none;
}}
</style>
</head>
<body class=\"isolate\">
  <div class=\"code-window\">
    <div class=\"window-bar\"></div>
    {highlighted}
  </div>
</body>
</html>"#)
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
