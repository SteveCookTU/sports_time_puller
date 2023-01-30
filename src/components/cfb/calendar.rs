use serde::Deserialize;

pub async fn get_calendar(year: i32) -> Vec<CalendarWeek> {
    if let Ok(response) = reqwest::get(format!("https://war-helper.com/calendar?year={year}")).await
    {
        response.json::<Vec<CalendarWeek>>().await.unwrap()
    } else {
        vec![]
    }
}

#[derive(Deserialize, Default)]
pub struct CalendarWeek {
    pub season: String,
    pub week: u8,
    #[serde(rename = "seasonType")]
    pub season_type: String,
    #[serde(rename = "firstGameStart")]
    pub first_game_start: String,
    #[serde(rename = "lastGameStart")]
    pub last_game_start: String,
}
