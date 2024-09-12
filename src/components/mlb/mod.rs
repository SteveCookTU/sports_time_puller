use crate::components::mlb::game::Game;
use crate::components::mlb::schedule::{Schedule, ScheduleGame};
use crate::components::mlb::teams::get_teams;
use crate::components::teams::*;
use crate::components::time_zone::*;
use crate::components::RequestParams;
use crate::time_zone::TimeZone as Tz;
use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, Timelike};
use leptos::*;
use std::str::FromStr;

mod game;
mod schedule;
mod teams;

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

async fn load_results(params: RequestParams<u16>) -> Vec<GameResult> {
    let time_zone = params.time_zone;
    let selected_date =
        NaiveDate::from_str(&params.date).unwrap_or_else(|_| Local::now().date_naive());
    let id = params.team;
    let mut results = vec![];
    let response = reqwest::get(format!("https://statsapi.mlb.com/api/v1/schedule?language=en&sportId=1&date={}&hydrate=game,broadcasts", params.date)).await.unwrap();
    let schedule = response.json::<Schedule>().await.unwrap_or_default();
    for date in schedule.dates {
        for schedule_game in date.games {
            if id != 0
                && schedule_game.teams.away.team.id != id
                && schedule_game.teams.home.team.id != id
            {
                continue;
            }
            if !["Final", "Game Over"].contains(&schedule_game.status.detailed_state.as_str())
                && !schedule_game
                    .status
                    .detailed_state
                    .contains("Completed Early")
            {
                results.push(GameResult {
                    title: format!(
                        "{} at {}",
                        schedule_game.teams.away.team.name, schedule_game.teams.home.team.name
                    ),
                    date: schedule_game.status.detailed_state.clone(),
                    venue_start: String::new(),
                    venue_end: String::new(),
                    duration: String::new(),
                    pre_game_delay: String::new(),
                    delay_time: String::new(),
                    start_time: String::new(),
                    end_time: String::new(),
                    broadcasts: String::new(),
                });
                continue;
            }
            get_live_game_data(&mut results, schedule_game, time_zone.into(), selected_date).await;
        }
    }
    results
}

async fn get_live_game_data(
    results: &mut Vec<GameResult>,
    schedule_game: ScheduleGame,
    time_zone: Tz,
    selected_date: NaiveDate,
) {
    if let Ok(response) = reqwest::get(format!(
        "https://statsapi.mlb.com/api/v1.1/game/{}/feed/live",
        schedule_game.game_pk
    ))
    .await
    {
        if let Ok(game) = response.json::<Game>().await {
            let start_time = DateTime::parse_from_rfc3339(&game.game_data.game_info.first_pitch)
                .unwrap_or_default()
                .with_timezone(&FixedOffset::east_opt(time_zone as i32 * 3600).unwrap());
            if start_time.date_naive().to_string() != selected_date.to_string() {
                return;
            }

            let delay_time: Duration = {
                let play = &game.live_data.play_info.all_plays[0];
                let mut duration = Duration::minutes(
                    game.game_data
                        .game_info
                        .delay_duration_minutes
                        .unwrap_or_default(),
                );
                for play_event in play.play_events.iter().skip(1) {
                    if let Some(description) = play_event.details.description.as_ref() {
                        if description.to_lowercase().contains("delayed start") {
                            duration = Duration::minutes(0);
                            break;
                        }
                    }
                }
                if duration.is_zero() {
                    for play in game.live_data.play_info.all_plays.iter() {
                        for play_event in play.play_events.iter() {
                            if let Some(description) = play_event.details.description.as_ref() {
                                if description.to_lowercase().contains("delayed")
                                    && !description.to_lowercase().contains("delayed start")
                                {
                                    if let Some(end_time) = play_event.end_time.as_ref() {
                                        let start_time =
                                            DateTime::parse_from_rfc3339(&play_event.start_time)
                                                .unwrap();
                                        let end_time =
                                            DateTime::parse_from_rfc3339(end_time).unwrap();

                                        let diff = end_time - start_time;
                                        duration = duration + diff;
                                    }
                                }
                            }
                        }
                    }
                }
                duration
            };
            let game_duration = Duration::minutes(game.game_data.game_info.game_duration_minutes);
            let end_time = start_time + game_duration + delay_time;
            let delay_duration = Duration::minutes(
                game.game_data
                    .game_info
                    .delay_duration_minutes
                    .unwrap_or_default(),
            );

            let pre_game_delay = delay_duration - delay_time;

            let venue_start_time = start_time.with_timezone(
                &FixedOffset::east_opt(3600 * game.game_data.venue.time_zone.offset as i32)
                    .unwrap(),
            );
            let venue_end_time = end_time.with_timezone(
                &FixedOffset::east_opt(3600 * game.game_data.venue.time_zone.offset as i32)
                    .unwrap(),
            );

            let broadcasts = schedule_game
                .broadcasts
                .into_iter()
                .filter_map(|bi| {
                    if bi.broadcast_type.as_str() == "TV" {
                        Some(format!(
                            "{} ({})",
                            bi.name.replace("(out-of-market only)", ""),
                            bi.home_away.chars().next().unwrap().to_uppercase()
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            results.push(GameResult {
                title: format!(
                    "{} at {}",
                    schedule_game.teams.away.team.name, schedule_game.teams.home.team.name
                ),
                date: start_time.date_naive().to_string(),
                venue_start: format!(
                    "{:0>2}:{:0>2} {}",
                    venue_start_time.hour(),
                    venue_start_time.minute(),
                    game.game_data.venue.time_zone.tz
                ),
                venue_end: format!(
                    "{:0>2}:{:0>2} {}",
                    venue_end_time.hour(),
                    venue_end_time.minute(),
                    game.game_data.venue.time_zone.tz
                ),
                duration: format!(
                    "{}:{:0>2}",
                    game_duration.num_hours(),
                    game_duration.num_minutes() % 60
                ),
                pre_game_delay: format!(
                    "{}:{:0>2}",
                    pre_game_delay.num_hours(),
                    pre_game_delay.num_minutes() % 60,
                ),
                delay_time: format!(
                    "{}:{:0>2}",
                    delay_time.num_hours(),
                    delay_time.num_minutes() % 60,
                ),
                start_time: format!(
                    "{:0>2}:{:0>2} {}",
                    start_time.hour(),
                    start_time.minute(),
                    time_zone.region()
                ),
                end_time: format!(
                    "{:0>2}:{:0>2} {}",
                    end_time.hour(),
                    end_time.minute(),
                    time_zone.region()
                ),
                broadcasts,
            });
        }
    }
}

#[component]
pub fn mlb() -> impl IntoView {
    let (date, set_date) = create_signal(Local::now().date_naive().format("%Y-%m-%d").to_string());
    let (time_zone, set_time_zone) = create_signal(Tz::Edt as i8);
    let (team, set_team) = create_signal(0);
    let retrieve_results = create_action(|value: &RequestParams<u16>| load_results(value.clone()));

    view! {
        <div class="flex flex-col">
            <div class="flex h-12 justify-around items-center m-4 bg-gray-300 rounded-lg shadow-sm shadow-gray-400">
                <Teams value={team.get()} on_change=move |ev| {
                        set_team.set(event_target_value(&ev).parse::<u16>().unwrap_or_default());
                    }
                    get_teams=|| {get_teams()}
                />
                <TimeZone value={time_zone.get()} set_time_zone={set_time_zone}/>
                <input class="bg-transparent border border-gray-600 rounded-md text-right" type="date" value={date} on:input=move |ev| {
                    set_date.set(event_target_value(&ev));
                }/>
                <button class="bg-transparent border border-gray-600 rounded-md transition-colors hover:bg-gray-200 px-2 py-1" on:click=move |_| retrieve_results.dispatch(RequestParams {
                        team: team.get(), date: date.get(), time_zone: time_zone.get()
                    })>"Submit"</button>
            </div>
            <table class="mx-4 border-spacing-0 border-separate rounded-t-lg shadow-sm shadow-gray-400">
                <thead>
                    <tr>
                        <th class="table-cell-tl bg-gray-400">"Game"</th>
                        <th class="table-cell bg-gray-400">"Date"</th>
                        <th class="table-cell bg-gray-400">"Venue Start"</th>
                        <th class="table-cell bg-gray-400">"Venue End"</th>
                        <th class="table-cell bg-gray-400">"Duration"</th>
                        <th class="table-cell bg-gray-400">"Pre-Delay"</th>
                        <th class="table-cell bg-gray-400">"Delay"</th>
                        <th class="table-cell bg-gray-400">"Start"</th>
                        <th class="table-cell bg-gray-400">"End"</th>
                        <th class="table-cell-tr bg-gray-400">"Broadcasts"</th>
                    </tr>
                </thead>
                <tbody>
                    { move || retrieve_results.value().with(|results: &Option<Vec<GameResult>>| {
                        if let Some(results) = results {
                            view ! {
                                {
                                    results.iter().map(|r| {
                                        view! {
                                            <tr>
                                                <td class="table-cell bg-gray-300">{&r.title}</td>
                                                <td class="table-cell bg-gray-300">{&r.date}</td>
                                                <td class="table-cell bg-gray-300">{&r.venue_start}</td>
                                                <td class="table-cell bg-gray-300">{&r.venue_end}</td>
                                                <td class="table-cell bg-gray-300">{&r.duration}</td>
                                                <td class="table-cell bg-gray-300">{&r.pre_game_delay}</td>
                                                <td class="table-cell bg-gray-300">{&r.delay_time}</td>
                                                <td class="table-cell bg-gray-300">{&r.start_time}</td>
                                                <td class="table-cell bg-gray-300">{&r.end_time}</td>
                                                <td class="table-cell bg-gray-300">{&r.broadcasts}</td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>().into_view()
                                }
                            }
                        } else {
                            view! {
                                <></>
                            }.into_view()
                        }
                    })
                    }
                </tbody>
            </table>
            <p class="text-right whitespace-pre mr-4 font-semibold">"H = Home\nA = Away"</p>
        </div>
    }
}
