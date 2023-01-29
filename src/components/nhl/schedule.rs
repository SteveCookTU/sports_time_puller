use crate::components::nhl::teams::Team;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Schedule {
    pub dates: Vec<ScheduleDate>,
}

#[derive(Deserialize, Default)]
pub struct ScheduleDate {
    pub date: String,
    pub games: Vec<ScheduleGame>,
}

#[derive(Deserialize, Default)]
pub struct ScheduleGame {
    #[serde(rename = "gamePk")]
    pub game_pk: usize,
    pub status: GameStatus,
    pub teams: Teams,
    pub broadcasts: Option<Vec<Broadcast>>,
}

#[derive(Deserialize, Default)]
pub struct GameStatus {
    #[serde(rename = "detailedState")]
    pub detailed_state: String,
}

#[derive(Deserialize, Default)]
pub struct Teams {
    pub away: TeamOuter,
    pub home: TeamOuter,
}

#[derive(Deserialize, Default)]
pub struct TeamOuter {
    pub team: Team,
}

#[derive(Deserialize, Default)]
pub struct Broadcast {
    pub id: u16,
    pub name: String,
    #[serde(rename = "type")]
    pub home_away: String,
}
