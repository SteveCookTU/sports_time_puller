pub mod cfb;
pub mod common;
pub mod mlb;
pub mod nhl;
pub mod soccer;
pub mod teams;
pub mod time_zone;

#[derive(Clone, Default)]
struct RequestParams<T: Clone + Copy + Default> {
    team: T,
    date: String,
    time_zone: i8,
}
