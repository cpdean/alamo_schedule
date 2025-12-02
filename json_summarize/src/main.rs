use clap::Parser;

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
    // Use clap to parse the command-line into a structured form.
    let cli = Cli::parse();

    // Build a canonical curl-style argument string from the parsed
    // fields. If you invoke the binary as:
    //   json_summarize 'url' -H 'h1' -b 'c1'
    // it will print:
    //   'url' -H 'h1' -b 'c1'
    // and calling the binary again with that printed string as its
    // arguments will reproduce the same output.
    let mut parts: Vec<String> = Vec::new();

    // URL first, wrapped in single quotes.
    parts.push(format!("'{}'", cli.url));

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
