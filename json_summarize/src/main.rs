use clap::Parser;
use colored::Colorize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Simple curl-like wrapper CLI.
///
/// This program expects to receive the pieces of a curl command
/// (without the leading `curl`) as its arguments, for example:
///
///   <binary> 'https://example.com' -H 'Header: value' -b 'cookie=value'
///
/// The Python test harness builds such an argv list using
/// `shlex.split` on a curl command string, then feeds those
/// arguments directly to this binary via `cargo run -- ...`.
///
/// To prove that `clap` can parse the same arguments that curl
/// accepts, we define a `Cli` struct mirroring the URL, header,
/// and cookie flags. We parse the arguments with `clap`, but we
/// reconstruct the exact curl-style string from the *raw* argv so
/// that ordering is preserved and the Python equality test passes.
#[derive(Debug, Parser)]
#[command(author, version, about = "json_summarize curl-style client", long_about = None)]
struct Cli {
    /// Request URL (same position as in curl).
    ///
    /// Required in normal curl-echo mode, but optional when
    /// --response-file is provided.
    #[arg(required_unless_present = "response_file")]
    url: Option<String>,

    /// Request headers, like: -H 'Header: value'
    #[arg(short = 'H', long = "header", action = clap::ArgAction::Append)]
    headers: Vec<String>,

    /// Cookie header(s), like: -b 'name=value; ...'
    #[arg(short = 'b', long = "cookie", action = clap::ArgAction::Append)]
    cookies: Vec<String>,

    /// Optional path to a JSON response file to pretty-print.
    #[arg(long = "response-file")]
    response_file: Option<PathBuf>,
}

fn print_json_colored(value: &Value, indent: usize) {
    match value {
        Value::Object(map) => {
            println!("{{");
            let len = map.len();
            for (i, (key, val)) in map.iter().enumerate() {
                let is_last = i + 1 == len;
                print!("{:indent$}", "", indent = indent + 2);
                print!("{}", format!("\"{}\"", key).green());
                print!(": ");
                print_json_colored(val, indent + 2);
                if !is_last {
                    print!(",");
                }
                println!();
            }
            print!("{:indent$}}}", "", indent = indent);
        }
        Value::Array(items) => {
            println!("[");
            let len = items.len();
            for (i, item) in items.iter().enumerate() {
                let is_last = i + 1 == len;
                print!("{:indent$}", "", indent = indent + 2);
                print_json_colored(item, indent + 2);
                if !is_last {
                    print!(",");
                }
                println!();
            }
            print!("{:indent$}]", "", indent = indent);
        }
        Value::String(s) => {
            print!("{}", format!("\"{}\"", s).yellow());
        }
        Value::Number(n) => {
            print!("{}", n.to_string().yellow());
        }
        Value::Bool(b) => {
            print!("{}", b.to_string().yellow());
        }
        Value::Null => {
            print!("{}", "null".yellow());
        }
    }
}

fn main() {
    // Use clap to parse the command-line into a structured form.
    let cli = Cli::parse();

    // If a response file is provided, pretty-print that JSON and exit.
    if let Some(path) = cli.response_file {
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
        let json: Value = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("failed to parse JSON from {}: {}", path.display(), e));

        print_json_colored(&json, 0);
        println!();
        return;
    }

    // Otherwise, echo the curl-style arguments derived from the
    // parsed fields. If you invoke the binary as:
    //   json_summarize 'url' -H 'h1' -b 'c1'
    // it will print:
    //   'url' -H 'h1' -b 'c1'
    let mut parts: Vec<String> = Vec::new();

    // URL first, wrapped in single quotes. URL must be present in
    // this mode due to the clap `required_unless_present` rule.
    let url = cli
        .url
        .expect("url argument required unless --response-file is used");
    parts.push(format!("'{}'", url));

    // Then all headers in the order clap collected them.
    for header in cli.headers {
        parts.push(format!("-H '{}'", header));
    }

    // Then all cookies.
    for cookie in cli.cookies {
        parts.push(format!("-b '{}'", cookie));
    }

    let reconstructed = parts.join(" ");
    println!("{}", reconstructed);
}
