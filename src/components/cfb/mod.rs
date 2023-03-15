mod calendar;
mod outlets;

use crate::components::cfb::calendar::get_calendar;
use crate::components::cfb::outlets::{get_outlets, OUTLETS};
use crate::components::teams::*;
use crate::components::time_zone::*;
use crate::components::RequestParams;
use crate::time_zone::TimeZone as Tz;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use leptos::*;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Clone, Deserialize, Debug)]
pub struct GameResult {
    pub team: String,
    pub start: String,
    pub kickoff: String,
    pub end: String,
    pub start_trans: String,
    pub kickoff_trans: String,
    pub end_trans: String,
    pub date: String,
}

async fn load_results(params: RequestParams<u8>) -> Vec<GameResult> {
    let time_zone = params.time_zone;
    let selected_date =
        NaiveDate::from_str(&params.date).unwrap_or_else(|_| Local::now().date_naive());
    let id = params.team;
    let year = if selected_date < NaiveDate::from_ymd_opt(selected_date.year(), 7, 1).unwrap() {
        selected_date.year() - 1
    } else {
        selected_date.year()
    };

    let calendar = get_calendar(year).await;

    let week = calendar.iter().find(|week| {
        selected_date
            < DateTime::parse_from_rfc3339(&week.last_game_start)
                .unwrap_or_default()
                .date_naive()
    });
    let Some(week) = week else {
        return vec![];
    };

    if let Ok(response) = reqwest::get(format!("https://war-helper.com/time?year={year}&week={}&seasonType={}&offset={time_zone}&outlet={}", week.week, week.season_type, &OUTLETS[id as usize])).await {
        if let Ok(result) = response.json::<Vec<GameResult>>().await {
            return result.into_iter().filter_map(|mut r| {
                log!("{:?}", r);
                if r.date == params.date {
                    r.start = format!("{} UTC", r.start);
                    r.kickoff = format!("{} UTC", r.kickoff);
                    r.end = format!("{} UTC", r.end);

                    r.start_trans += format!(" {}", Tz::from(time_zone).region()).as_str();
                    r.kickoff_trans += format!(" {}", Tz::from(time_zone).region()).as_str();
                    r.end_trans += format!(" {}", Tz::from(time_zone).region()).as_str();
                    Some(r)
                } else {
                    None
                }
            }).collect()
        } else {
            log!("Failed to parse response")
        }
    } else {
        log!("failed to get response")
    }
    vec![]
}

#[component]
pub fn cfb(cx: Scope) -> impl IntoView {
    let (date, set_date) =
        create_signal(cx, Local::now().date_naive().format("%Y-%m-%d").to_string());
    let (time_zone, set_time_zone) = create_signal(cx, Tz::Edt as i8);
    let (outlet, set_outlet) = create_signal(cx, 0);
    let retrieve_results =
        create_action(cx, |value: &RequestParams<u8>| load_results(value.clone()));

    view! {
        cx,
        <div class="flex flex-col">
            <div class="flex h-12 justify-around items-center m-4 bg-gray-300 rounded-lg shadow-sm shadow-gray-400">
                <Teams value={outlet.get()} on_change=move |ev| {
                        set_outlet(event_target_value(&ev).parse::<u8>().unwrap_or_default());
                    }
                    get_teams=|| {get_outlets()}
                />
                <TimeZone value={time_zone.get()} set_time_zone={set_time_zone}/>
                <input class="bg-transparent border border-gray-600 rounded-md text-right" type="date" value={date} on:input=move |ev| {
                    set_date(event_target_value(&ev));
                }/>
                <button class="bg-transparent border border-gray-600 rounded-md transition-colors hover:bg-gray-200 px-2 py-1" on:click=move |_| retrieve_results.dispatch(RequestParams {
                        team: outlet.get(), date: date.get(), time_zone: time_zone.get()
                    })>"Submit"</button>
            </div>
            <table class="mx-4 border-spacing-0 border-separate rounded-t-lg shadow-sm shadow-gray-400">
                <thead>
                    <tr>
                        <th class="table-cell-tl bg-gray-400">"Game"</th>
                        <th class="table-cell bg-gray-400">"Date"</th>
                        <th class="table-cell bg-gray-400">"Start"</th>
                        <th class="table-cell bg-gray-400">"Kickoff"</th>
                        <th class="table-cell bg-gray-400">"End"</th>
                        <th class="table-cell bg-gray-400">"Conv. Start"</th>
                        <th class="table-cell bg-gray-400">"Conv. Kickoff"</th>
                        <th class="table-cell-tr bg-gray-400">"Conv. End"</th>
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
                                                <td class="table-cell bg-gray-300">{&r.team}</td>
                                                <td class="table-cell bg-gray-300">{&r.date}</td>
                                                <td class="table-cell bg-gray-300">{&r.start}</td>
                                                <td class="table-cell bg-gray-300">{&r.kickoff}</td>
                                                <td class="table-cell bg-gray-300">{&r.end}</td>
                                                <td class="table-cell bg-gray-300">{&r.start_trans}</td>
                                                <td class="table-cell bg-gray-300">{&r.kickoff_trans}</td>
                                                <td class="table-cell bg-gray-300">{&r.end_trans}</td>
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
        </div>
    }
}
