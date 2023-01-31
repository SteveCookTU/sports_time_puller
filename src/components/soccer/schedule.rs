use chrono::NaiveDate;
use serde::Deserialize;

pub async fn get_matches(start_date: NaiveDate, end_date: NaiveDate) -> Vec<Match> {
    if let Ok(response) = reqwest::get(format!(
        "https://sportapi.mlssoccer.com/api/matches?culture=en-us&dateFrom={start_date}&dateTo={end_date}"
    ))
    .await
    {
        response.json::<Vec<Match>>().await.unwrap_or_default()
    } else {
        vec![]
    }
}

#[derive(Deserialize, Default)]
pub struct Match {
    #[serde(rename = "optaId")]
    pub opta_id: usize,
    pub home: MatchTeam,
    pub away: MatchTeam,
    pub competition: Competition,
    pub broadcasters: Vec<MatchBroadcaster>,
    #[serde(rename = "homeClubBroadcasters")]
    pub home_club_broadcasters: Vec<MatchBroadcaster>,
    #[serde(rename = "awayClubBroadcasters")]
    pub away_club_broadcasters: Vec<MatchBroadcaster>,
}

#[derive(Deserialize, Default)]
pub struct Competition {
    pub name: String,
}

#[derive(Deserialize, Default)]
pub struct MatchTeam {
    #[serde(rename = "fullName")]
    pub full_name: String,
}

#[derive(Deserialize, Default)]
pub struct MatchBroadcaster {
    #[serde(rename = "broadcasterType")]
    pub broadcaster_type: String,
    #[serde(rename = "broadcasterName")]
    pub broadcaster_name: String,
}
