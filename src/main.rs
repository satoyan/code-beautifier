use std::env;
use std::fs::File;
use std::io::Write;
mod lib;
use lib::{generate_html_from_source, parse_args};

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
