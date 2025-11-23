use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

mod data;
use data::{RawData, Session};

#[derive(Debug, Clone, PartialEq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Serialize)]
struct ShowtimeInfo {
    show_time: String,
    movie: String,
    theater: String,
    cinema_id: String,
    session_id: String,
    presentation_slug: String,
}

#[derive(Debug, Serialize)]
struct ShowtimeOutput {
    time_range: TimeRange,
    showtimes: Vec<ShowtimeInfo>,
}

#[derive(Debug, Serialize)]
struct TimeRange {
    start: String,
    end: String,
}

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

fn parse_args(args: &[String]) -> (Option<PathBuf>, OutputFormat) {
    let mut filepath = None;
    let mut output_format = OutputFormat::Text;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                if i + 1 < args.len() {
                    match args[i + 1].as_str() {
                        "json" => output_format = OutputFormat::Json,
                        "text" => output_format = OutputFormat::Text,
                        _ => eprintln!("Warning: Unknown output format '{}', defaulting to 'text'", args[i + 1]),
                    }
                    i += 2;
                } else {
                    eprintln!("Warning: --output flag requires a value, defaulting to 'text'");
                    i += 1;
                }
            }
            arg if !arg.starts_with("--") => {
                if filepath.is_none() {
                    filepath = Some(PathBuf::from(arg));
                }
                i += 1;
            }
            _ => {
                eprintln!("Warning: Unknown flag '{}'", args[i]);
                i += 1;
            }
        }
    }
    
    (filepath, output_format)
}

fn get_movie_schedule(
    filepath: Option<PathBuf>,
) -> Result<(HashMap<String, String>, HashMap<String, String>, RawData)> {
    let body = match filepath {
        Some(path) => read_from_file(path)?,
        None => fetch_from_api()?,
    };

    let data = RawData::from_json(&body);

    // Create a map of cinema IDs to names
    let mut cinema_names = std::collections::HashMap::new();
    for market in &data.data.market {
        for cinema in &market.cinemas {
            cinema_names.insert(cinema.id.clone(), cinema.name.clone());
        }
    }

    // Create a map of presentation slugs to movie titles
    let mut movie_titles = std::collections::HashMap::new();
    for pres in &data.data.presentations {
        movie_titles.insert(pres.slug.clone(), pres.show.title.clone());
    }

    Ok((cinema_names, movie_titles, data))
}

fn recently_showing_movies<'a>(
    data: &'a RawData,
    cinema_names: &'a HashMap<String, String>,
    now: DateTime<Local>,
    end_time: DateTime<Local>,
) -> Vec<&'a Session> {
    data.data
        .sessions
        .iter()
        .filter(|session| {
            // Parse the show time
            if let Ok(show_time) =
                NaiveDateTime::parse_from_str(&session.show_time_clt, "%Y-%m-%dT%H:%M:%S")
            {
                let show_time = Local.from_local_datetime(&show_time).unwrap();
                // Filter by time and exclude Staten Island theater
                let theater = cinema_names
                    .get(&session.cinema_id)
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown Theater");
                show_time > now && show_time <= end_time && theater != "Staten Island"
            } else {
                false
            }
        })
        .collect()
}

fn format_show_time(show_time_clt: &str) -> String {
    let show_time = show_time_clt.split('T').collect::<Vec<_>>();
    let date = show_time[0].split('-').collect::<Vec<_>>();
    let formatted_date = format!("{}/{}", date[1], date[2]); // MM/DD
    let time = show_time[1]
        .split(':')
        .take(2)
        .collect::<Vec<_>>()
        .join(":");
    format!("{} {}", formatted_date, time)
}

fn display_sessions_text<'a>(
    sessions: &Vec<&'a Session>,
    cinema_names: &HashMap<String, String>,
    movie_titles: &HashMap<String, String>,
) {
    for session in sessions {
        let datetime = format_show_time(&session.show_time_clt);

        let unknown_movie = String::from("Unknown Movie");
        let movie = movie_titles
            .get(&session.presentation_slug)
            .unwrap_or(&unknown_movie);

        let unknown_theater = String::from("Unknown Theater");
        let theater = cinema_names
            .get(&session.cinema_id)
            .unwrap_or(&unknown_theater);

        println!("{:^25} | {:<40} | {:<25}", datetime, movie, theater);
    }
}

fn display_sessions_json<'a>(
    sessions: &Vec<&'a Session>,
    cinema_names: &HashMap<String, String>,
    movie_titles: &HashMap<String, String>,
    now: DateTime<Local>,
    end_time: DateTime<Local>,
) {
    let showtimes: Vec<ShowtimeInfo> = sessions
        .iter()
        .map(|session| {
            let unknown_movie = String::from("Unknown Movie");
            let movie = movie_titles
                .get(&session.presentation_slug)
                .unwrap_or(&unknown_movie)
                .clone();

            let unknown_theater = String::from("Unknown Theater");
            let theater = cinema_names
                .get(&session.cinema_id)
                .unwrap_or(&unknown_theater)
                .clone();

            ShowtimeInfo {
                show_time: session.show_time_clt.clone(),
                movie,
                theater,
                cinema_id: session.cinema_id.clone(),
                session_id: session.session_id.clone(),
                presentation_slug: session.presentation_slug.clone(),
            }
        })
        .collect();

    let output = ShowtimeOutput {
        time_range: TimeRange {
            start: now.to_rfc3339(),
            end: end_time.to_rfc3339(),
        },
        showtimes,
    };

    match serde_json::to_string_pretty(&output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (filepath, output_format) = parse_args(&args);

    let (cinema_names, movie_titles, data) = get_movie_schedule(filepath)?;

    // Filter and sort sessions by show time
    let now = Local::now();
    let twelve_hours = Duration::hours(12);
    let end_time = now + twelve_hours;

    let mut sessions: Vec<_> =
        recently_showing_movies(&data, &cinema_names, now, end_time);

    // Sort filtered sessions
    sessions.sort_by(|a, b| a.show_time_clt.cmp(&b.show_time_clt));

    // Display based on output format
    match output_format {
        OutputFormat::Text => {
            // Print header
            println!(
                "\n{:^25} | {:<40} | {:<25}",
                "Show Time", "Movie", "Theater"
            );
            println!("{}", "-".repeat(94));

            // Print header with time range
            println!(
                "\nShowings between {} and {}",
                now.format("%m/%d %H:%M"),
                end_time.format("%m/%d %H:%M")
            );

            // Print each session
            display_sessions_text(&sessions, &cinema_names, &movie_titles);
        }
        OutputFormat::Json => {
            display_sessions_json(&sessions, &cinema_names, &movie_titles, now, end_time);
        }
    }

    Ok(())
}
