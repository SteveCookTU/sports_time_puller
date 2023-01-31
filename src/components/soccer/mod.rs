mod game;
mod schedule;

use crate::components::soccer::game::get_game;
use crate::components::soccer::schedule::get_matches;
use crate::components::time_zone::*;
use crate::components::RequestParams;
use crate::time_zone::TimeZone as Tz;
use chrono::{DateTime, Datelike, Days, FixedOffset, Local, NaiveDate, NaiveDateTime, Timelike};
use leptos::*;
use std::str::FromStr;

#[derive(Default)]
struct GameResult {
    competition: String,
    title: String,
    date: String,
    start: String,
    end: String,
    start_trans: String,
    end_trans: String,
    broadcasts: String,
}

async fn load_results(params: RequestParams<()>) -> Vec<GameResult> {
    let time_zone = params.time_zone;
    let selected_date =
        NaiveDate::from_str(&params.date).unwrap_or_else(|_| Local::now().date_naive());

    let mut results = vec![];
    let matches = get_matches(
        selected_date.checked_sub_days(Days::new(1)).unwrap(),
        selected_date.checked_add_days(Days::new(1)).unwrap(),
    )
    .await;
    for mat in matches {
        let game = get_game(mat.opta_id).await;
        if game.postponed {
            results.push(GameResult {
                competition: mat.competition.name,
                title: format!("{} at {}", mat.away.full_name, mat.home.full_name),
                date: "Postponed".to_string(),
                ..Default::default()
            });
            continue;
        } else if game.abandoned {
            results.push(GameResult {
                competition: mat.competition.name,
                title: format!("{} at {}", mat.away.full_name, mat.home.full_name),
                date: "Abandoned".to_string(),
                ..Default::default()
            });
            continue;
        } else if !game.is_final {
            results.push(GameResult {
                competition: mat.competition.name,
                title: format!("{} at {}", mat.away.full_name, mat.home.full_name),
                date: "Scheduled".to_string(),
                ..Default::default()
            });
            continue;
        }

        let start_time =
            NaiveDateTime::from_timestamp_millis(game.first_half_start).unwrap_or_default();
        let end_time =
            NaiveDateTime::from_timestamp_millis(game.second_half_end).unwrap_or_default();
        let start_trans = DateTime::<FixedOffset>::from_utc(
            start_time,
            FixedOffset::east_opt(time_zone as i32 * 3600).unwrap(),
        );
        let end_trans = DateTime::<FixedOffset>::from_utc(
            end_time,
            FixedOffset::east_opt(time_zone as i32 * 3600).unwrap(),
        );

        if start_trans.date_naive() != selected_date && end_trans.date_naive() != selected_date {
            continue;
        }

        let broadcasts = mat
            .broadcasters
            .into_iter()
            .chain(mat.home_club_broadcasters)
            .chain(mat.away_club_broadcasters)
            .filter(|b| b.broadcaster_type.contains("TV"))
            .map(|b| b.broadcaster_name)
            .collect::<Vec<_>>()
            .join(", ");
        results.push(GameResult {
            competition: mat.competition.name,
            title: format!("{} at {}", mat.away.full_name, mat.home.full_name),
            date: start_trans.date_naive().to_string(),
            start: if start_time.month() != selected_date.month() {
                "Invalid Start".to_string()
            } else {
                format!("{:0>2}:{:0>2} UTC", start_time.hour(), start_time.minute())
            },
            end: format!("{:0>2}:{:0>2} UTC", end_time.hour(), end_time.minute(),),
            start_trans: if start_time.month() != selected_date.month() {
                "Invalid Start".to_string()
            } else {
                format!(
                    "{:0>2}:{:0>2} {}",
                    start_trans.hour(),
                    start_trans.minute(),
                    Tz::from(time_zone).region()
                )
            },
            end_trans: format!(
                "{:0>2}:{:0>2} {}",
                end_trans.hour(),
                end_trans.minute(),
                Tz::from(time_zone).region()
            ),
            broadcasts,
        });
    }

    results
}

#[component]
pub fn soccer(cx: Scope) -> impl IntoView {
    let (date, set_date) =
        create_signal(cx, Local::now().date_naive().format("%Y-%m-%d").to_string());
    let (time_zone, set_time_zone) = create_signal(cx, Tz::Est as i8);
    let retrieve_results =
        create_action(cx, |value: &RequestParams<()>| load_results(value.clone()));

    view! {
        cx,
        <div class="flex flex-col">
            <div class="flex h-12 justify-around items-center m-4 bg-gray-300 rounded-lg shadow-sm shadow-gray-400">
                <TimeZone value={time_zone.get()} set_time_zone={set_time_zone}/>
                <input class="bg-transparent border border-gray-600 rounded-md text-right" type="date" value={date} on:input=move |ev| {
                    set_date(event_target_value(&ev));
                }/>
                <button class="bg-transparent border border-gray-600 rounded-md transition-colors hover:bg-gray-200 px-2 py-1" on:click=move |_| retrieve_results.dispatch(RequestParams {
                        team: (), date: date.get(), time_zone: time_zone.get()
                    })>"Submit"</button>
            </div>
            <table class="mx-4 border-spacing-0 border-separate rounded-t-lg shadow-sm shadow-gray-400">
                <thead>
                    <tr>
                        <th class="table-cell-tl bg-gray-400">"Competition"</th>
                        <th class="table-cell bg-gray-400">"Game"</th>
                        <th class="table-cell bg-gray-400">"Date"</th>
                        <th class="table-cell bg-gray-400">"Start"</th>
                        <th class="table-cell bg-gray-400">"End"</th>
                        <th class="table-cell bg-gray-400">"Conv. Start"</th>
                        <th class="table-cell bg-gray-400">"Conv. End"</th>
                        <th class="table-cell-tr bg-gray-400">"Broadcasts"</th>
                    </tr>
                </thead>
                <tbody>
                    { move || retrieve_results.value().with(|results: &Option<Vec<GameResult>>| {
                        if let Some(results) = results {
                            view ! {
                                cx,
                                {
                                    results.iter().map(|r| {
                                        view! {
                                            cx,
                                            <tr>
                                                <td class="table-cell bg-gray-300">{&r.competition}</td>
                                                <td class="table-cell bg-gray-300">{&r.title}</td>
                                                <td class="table-cell bg-gray-300">{&r.date}</td>
                                                <td class="table-cell bg-gray-300">{&r.start}</td>
                                                <td class="table-cell bg-gray-300">{&r.end}</td>
                                                <td class="table-cell bg-gray-300">{&r.start_trans}</td>
                                                <td class="table-cell bg-gray-300">{&r.end_trans}</td>
                                                <td class="table-cell bg-gray-300">{&r.broadcasts}</td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>().into_view(cx)
                                }
                            }
                        } else {
                            view! {
                                cx,
                                <></>
                            }.into_view(cx)
                        }
                    })
                    }
                </tbody>
            </table>
            <p class="text-right whitespace-pre mr-4 font-semibold">"N = National\nH = Home\nA = Away"</p>
        </div>
    }
}
