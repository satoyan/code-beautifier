use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
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
    let opts = parse_args(&args);
    let html = generate_html_from_source(&code, &opts.language);

    if opts.output_to_file {
        let mut file = File::create(&opts.output_filename)
            .expect("Failed to create output file");
        file.write_all(html.as_bytes())
            .expect("Failed to write HTML to file");
    } else {
        println!("{}", html);
    }
}
