use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Game {
    #[serde(rename = "gameData")]
    pub game_data: GameData,
}

#[derive(Deserialize, Default)]
pub struct GameData {
    pub teams: GameTeams,
    pub datetime: GameDateTime,
}

#[derive(Deserialize, Default)]
pub struct GameDateTime {
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "endDateTime")]
    pub end_date_time: String,
}

#[derive(Deserialize, Default)]
pub struct GameTeams {
    pub home: Home,
}

#[derive(Deserialize, Default)]
pub struct Home {
    pub venue: Venue,
}

#[derive(Deserialize, Default)]
pub struct Venue {
    #[serde(rename = "timeZone")]
    pub time_zone: VenueTimeZone,
}

#[derive(Deserialize, Default)]
pub struct VenueTimeZone {
    pub offset: i8,
    pub tz: String,
}
