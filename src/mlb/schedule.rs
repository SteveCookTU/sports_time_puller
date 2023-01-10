use crate::mlb::teams::Team;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Schedule {
    pub dates: Vec<ScheduleDate>,
}

#[derive(Deserialize)]
pub struct ScheduleDate {
    pub games: Vec<ScheduleGame>,
}

#[derive(Deserialize)]
pub struct ScheduleGame {
    #[serde(rename = "gamePk")]
    pub game_pk: u32,
    pub status: GameStatus,
    pub teams: Teams,
    pub broadcasts: Vec<BroadcastInfo>,
}

#[derive(Deserialize)]
pub struct GameStatus {
    #[serde(rename = "detailedState")]
    pub detailed_state: String,
}

#[derive(Deserialize)]
pub struct Teams {
    pub home: HomeTeam,
    pub away: AwayTeam,
}

#[derive(Deserialize)]
pub struct HomeTeam {
    pub team: Team,
}

#[derive(Deserialize)]
pub struct AwayTeam {
    pub team: Team,
}

#[derive(Deserialize)]
pub struct BroadcastInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub broadcast_type: String,
    #[serde(rename = "homeAway")]
    pub home_away: String,
}
