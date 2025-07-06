use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone};
use std::collections::HashMap;
use std::path::PathBuf;

mod data;
use data::{RawData, Session};

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

fn get_movie_schedule(
    args: Vec<String>,
) -> Result<(HashMap<String, String>, HashMap<String, String>, RawData)> {
    let body = match args.get(1) {
        Some(filepath) => read_from_file(filepath.into())?,
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

fn display_sessions<'a>(
    sessions: &Vec<&'a Session>,
    cinema_names: &HashMap<String, String>,
    movie_titles: &HashMap<String, String>,
) {
    for session in sessions {
        let show_time = session.show_time_clt.split('T').collect::<Vec<_>>();
        let date = show_time[0].split('-').collect::<Vec<_>>();
        let formatted_date = format!("{}/{}", date[1], date[2]); // MM/DD
        let time = show_time[1]
            .split(':')
            .take(2)
            .collect::<Vec<_>>()
            .join(":");
        let datetime = format!("{} {}", formatted_date, time);

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

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let (cinema_names, movie_titles, data) = get_movie_schedule(args)?;

    // Print header
    println!(
        "\n{:^25} | {:<40} | {:<25}",
        "Show Time", "Movie", "Theater"
    );
    println!("{}", "-".repeat(94));

    // Filter and sort sessions by show time
    let now = Local::now();
    let twelve_hours = Duration::hours(12);

    let mut sessions: Vec<_> =
        recently_showing_movies(&data, &cinema_names, now, now + twelve_hours);

    // Sort filtered sessions
    sessions.sort_by(|a, b| a.show_time_clt.cmp(&b.show_time_clt));

    // Print header with time range
    let end_time = now + twelve_hours;
    println!(
        "\nShowings between {} and {}",
        now.format("%m/%d %H:%M"),
        end_time.format("%m/%d %H:%M")
    );

    // Print each session
    display_sessions(&sessions, &cinema_names, &movie_titles);

    Ok(())
}
