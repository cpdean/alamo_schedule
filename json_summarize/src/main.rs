use clap::Parser;

/// Simple curl-like wrapper CLI.
#[derive(Debug, Parser)]
#[command(author, version, about = "json_summarize curl-style client", long_about = None)]
struct Cli {
    /// Request URL (same position as in curl)
    url: String,

    /// Request headers, like: -H 'Header: value'
    #[arg(short = 'H', long = "header", action = clap::ArgAction::Append)]
    headers: Vec<String>,

    /// Cookie header(s), like: -b 'name=value; ...'
    #[arg(short = 'b', long = "cookie", action = clap::ArgAction::Append)]
    cookies: Vec<String>,
}

fn main() {
    let _args = Cli::parse();

    println!("hello world");
}
