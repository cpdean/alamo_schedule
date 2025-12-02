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
    let args = Cli::parse();

    let mut parts = Vec::new();

    // URL first, wrapped in single quotes to match original curl command
    parts.push(format!("'{}'", args.url));

    // Headers as repeated -H 'Header: value'
    for header in args.headers.iter() {
        parts.push(format!("-H '{}'", header));
    }

    // Cookies as repeated -b 'cookie=value; ...'
    for cookie in args.cookies.iter() {
        parts.push(format!("-b '{}'", cookie));
    }

    let reconstructed = parts.join(" ");
    println!("{}", reconstructed);
}
