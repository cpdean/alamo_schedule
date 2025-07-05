use anyhow::{Context, Result};
use std::path::PathBuf;

mod data;
use data::RawData;

fn fetch_from_api() -> Result<String> {
    let url = "https://drafthouse.com/s/mother/v2/schedule/market/nyc";
    
    let response = attohttpc::get(url)
        .header("accept", "application/json, text/plain, */*")
        .header("accept-language", "en-US,en;q=0.9")
        .header("referer", "https://drafthouse.com/nyc")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .send()
        .context("Failed to make HTTP request")?;

    if !response.is_success() {
        anyhow::bail!("HTTP error: {}", response.status());
    }

    response.text().context("Failed to read response body")
}

fn read_from_file(path: PathBuf) -> Result<String> {
    std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    let body = match args.get(1) {
        Some(filepath) => read_from_file(filepath.into())?,
        None => fetch_from_api()?,
    };

    let data = RawData::from_json(&body);
    
    println!("Market Information:");
    for market in &data.data.market {
        println!("  {}", market.name);
        println!("  Status: {}", market.status);
        println!("  Cinemas:");
        for cinema in &market.cinemas {
            println!("    - {} ({})", cinema.name, cinema.slug);
        }
    }

    println!("\nUpcoming Movies:");
    for pres in &data.data.presentations {
        println!("  - {}", pres.show.title);
    }

    println!("\nFound {} total sessions", data.data.sessions.len());

    Ok(())
}
