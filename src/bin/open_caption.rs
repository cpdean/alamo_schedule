use alamo_schedule::data::RawData;
use anyhow::Result;
use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
use chrono_tz::America::New_York;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let body = match args.get(1) {
        Some(filepath) => fs::read_to_string(filepath)?,
        None => {
            eprintln!("Please provide a JSON file path");
            std::process::exit(1);
        }
    };

    let data = RawData::from_json(&body);

    // Create lookup maps
    let mut cinema_names = std::collections::HashMap::new();
    for market in &data.data.market {
        for cinema in &market.cinemas {
            cinema_names.insert(cinema.id.clone(), cinema.name.clone());
        }
    }

    let mut movie_titles = std::collections::HashMap::new();
    for pres in &data.data.presentations {
        movie_titles.insert(pres.slug.clone(), pres.show.title.clone());
    }

    // Filter and sort sessions in Eastern time
    let now_utc = Utc::now();
    let now = now_utc.with_timezone(&New_York);
    let fourteen_days = Duration::days(14);
    let end_date = now + fourteen_days;

    let mut sessions: Vec<_> = data
        .data
        .sessions
        .iter()
        .filter(|session| {
            // Check if it's an open-caption showing
            if session.format_slug.as_deref() != Some("open-caption") {
                return false;
            }

            // Parse the show time (API times are Eastern)
            if let Ok(show_time_naive) =
                NaiveDateTime::parse_from_str(&session.show_time_clt, "%Y-%m-%dT%H:%M:%S")
            {
                let show_time = New_York
                    .from_local_datetime(&show_time_naive)
                    .single()
                    .unwrap();
                show_time > now && show_time <= end_date
            } else {
                false
            }
        })
        .collect();

    // Sort filtered sessions
    sessions.sort_by(|a, b| a.show_time_clt.cmp(&b.show_time_clt));

    // Print header
    println!("\n🎬 Open Caption Showings (next 14 days)");
    println!("{:^25} | {:<40} | {:<25}", "Show Time", "Movie", "Theater");
    println!("{}", "-".repeat(94));

    // Print each session
    for session in &sessions {
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

    Ok(())
}
