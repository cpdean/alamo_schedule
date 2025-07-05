use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    pub data: RawDataContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDataContent {
    pub presentations: Vec<Presentation>,
    pub market: Vec<Market>,
    pub sessions: Vec<Session>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub status: String,
    pub cinemas: Vec<Cinema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cinema {
    pub id: String,
    #[serde(rename = "loyaltyCinemaId")]
    pub loyalty_cinema_id: String,
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "cinemaId")]
    pub cinema_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "presentationSlug")]
    pub presentation_slug: String,
    #[serde(rename = "legacySlug")]
    pub legacy_slug: Option<String>,
    #[serde(rename = "showTimeClt")]
    pub show_time_clt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presentation {
    pub slug: String,
    #[serde(rename = "legacySlug")]
    pub legacy_slug: Option<String>,
    pub show: Show,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Show {
    pub slug: String,
    #[serde(rename = "legacySlug")]
    pub legacy_slug: Option<String>,
    pub title: String,
}

impl RawData {
    pub fn from_json(json: &str) -> RawData {
        serde_json::from_str(json).unwrap()
    }
}