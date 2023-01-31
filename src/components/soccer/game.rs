use serde::Deserialize;

pub async fn get_game(game_id: usize) -> Game {
    if let Ok(response) = reqwest::get(format!(
        "https://stats-api.mlssoccer.com/v1/matches?&match_game_id={game_id}"
    ))
    .await
    {
        response
            .json::<Vec<Game>>()
            .await
            .unwrap_or_default()
            .into_iter()
            .next()
            .unwrap_or_default()
    } else {
        Default::default()
    }
}

#[derive(Deserialize, Default)]
pub struct Game {
    pub date: i64,
    pub first_half_start: i64,
    pub second_half_end: i64,
    pub is_final: bool,
    pub postponed: bool,
    pub abandoned: bool,
}
