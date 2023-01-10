pub mod game;
pub mod mlb_pane;
pub mod schedule;
pub mod teams;

struct GameResult {
    pub title: String,
    pub date: String,
    pub venue_start: String,
    pub venue_end: String,
    pub duration: String,
    pub pre_game_delay: String,
    pub delay_time: String,
    pub start_time: String,
    pub end_time: String,
    pub broadcasts: String,
}
