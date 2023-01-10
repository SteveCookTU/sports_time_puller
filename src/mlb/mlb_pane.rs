use crate::mlb::game::Game;
use crate::mlb::schedule::{Schedule, ScheduleGame};
use crate::mlb::GameResult;
use crate::time_zone::TimeZone;
use crate::{mlb, Pane};
use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, Timelike};
use eframe::egui;
use eframe::egui::{Context, Direction, Layout, Ui};
use egui_extras::Column;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

static HEADERS: [&str; 10] = [
    "Game",
    "Date",
    "Venue Start Time",
    "Venue End Time",
    "Game Duration",
    "Pre-Game Delay",
    "During Game Delay",
    "Converted Start Time",
    "Converted End Time",
    "Broadcasts",
];

pub struct MlbPane {
    pub teams: Arc<RwLock<BTreeMap<u16, String>>>,
    pub selected_team: Option<(u16, String)>,
    pub date: NaiveDate,
    pub time_zone: TimeZone,
    results: Arc<RwLock<Vec<GameResult>>>,
}

impl MlbPane {
    pub fn new() -> Self {
        let teams = Arc::new(RwLock::new(BTreeMap::new()));

        mlb::teams::get_teams(teams.clone());

        Self {
            teams,
            selected_team: None,
            date: Local::now().date_naive(),
            time_zone: TimeZone::Est,
            results: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn load_results(&self, ctx: Context) {
        let request = ehttp::Request::get(format!("https://statsapi.mlb.com/api/v1/schedule?language=en&sportId=1&date={}&hydrate=game,broadcasts", self.date));
        let results = self.results.clone();
        let selected_team = self.selected_team.clone();
        let time_zone = self.time_zone;
        let selected_date = self.date;
        ehttp::fetch(request, move |response| {
            if let Ok(response) = response {
                if let Some(json) = response.text() {
                    if let Ok(schedule) = serde_json::from_str::<Schedule>(json) {
                        for date in schedule.dates {
                            for schedule_game in date.games {
                                if let Some(&(id, _)) = selected_team.as_ref() {
                                    if schedule_game.teams.away.team.id != id
                                        && schedule_game.teams.home.team.id != id
                                    {
                                        continue;
                                    }
                                }
                                if schedule_game.status.detailed_state.as_str() != "Final"
                                    && !schedule_game
                                        .status
                                        .detailed_state
                                        .contains("Completed Early")
                                {
                                    results.write().unwrap().push(GameResult {
                                        title: format!(
                                            "{} at {}",
                                            schedule_game.teams.away.team.name,
                                            schedule_game.teams.home.team.name
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
                                    ctx.request_repaint();
                                    continue;
                                }
                                let results = results.clone();
                                get_live_game_data(
                                    results,
                                    schedule_game,
                                    time_zone,
                                    selected_date,
                                    ctx.clone(),
                                );
                            }
                        }
                    }
                }
            }
        });
    }
}

fn get_live_game_data(
    results: Arc<RwLock<Vec<GameResult>>>,
    schedule_game: ScheduleGame,
    time_zone: TimeZone,
    selected_date: NaiveDate,
    ctx: Context,
) {
    let request = ehttp::Request::get(format!(
        "https://statsapi.mlb.com/api/v1.1/game/{}/feed/live",
        schedule_game.game_pk
    ));
    ehttp::fetch(request, move |response| {
        if let Ok(response) = response {
            if let Some(json) = response.text() {
                if let Ok(game) = serde_json::from_str::<Game>(json) {
                    let start_time =
                        DateTime::parse_from_rfc3339(&game.game_data.game_info.first_pitch)
                            .unwrap_or_default()
                            .with_timezone(
                                &FixedOffset::east_opt(time_zone as i32 * 3600).unwrap(),
                            );
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
                                    if let Some(description) =
                                        play_event.details.description.as_ref()
                                    {
                                        if description.to_lowercase().contains("delayed") {
                                            if let Some(end_time) = play_event.end_time.as_ref() {
                                                let start_time = DateTime::parse_from_rfc3339(
                                                    &play_event.start_time,
                                                )
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
                    let game_duration =
                        Duration::minutes(game.game_data.game_info.game_duration_minutes);
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
                                    bi.home_away
                                ))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    results.write().unwrap().push(GameResult {
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
                    ctx.request_repaint();
                }
            }
        }
    });
}

impl Pane for MlbPane {
    fn side_panel(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add(egui_extras::DatePickerButton::new(&mut self.date));
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("Team:");
                egui::ComboBox::from_id_source("team")
                    .wrap(true)
                    .selected_text(
                        self.selected_team
                            .as_ref()
                            .map(|t| t.1.as_str())
                            .unwrap_or("All"),
                    )
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "All").clicked() {
                            self.selected_team = None;
                        }
                        let teams = self.teams.read().unwrap();
                        for (&id, team) in teams.iter() {
                            if ui.selectable_label(false, team).clicked() {
                                self.selected_team = Some((id, team.to_string()));
                            }
                        }
                    });
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("Time Zone:");
                egui::ComboBox::from_id_source("time_zone")
                    .wrap(true)
                    .selected_text(self.time_zone.region())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.time_zone,
                            TimeZone::Est,
                            TimeZone::Est.region(),
                        );
                        ui.selectable_value(
                            &mut self.time_zone,
                            TimeZone::Cst,
                            TimeZone::Cst.region(),
                        );
                        ui.selectable_value(
                            &mut self.time_zone,
                            TimeZone::Mst,
                            TimeZone::Mst.region(),
                        );
                        ui.selectable_value(
                            &mut self.time_zone,
                            TimeZone::Pst,
                            TimeZone::Pst.region(),
                        );
                    });
            });
            ui.add_space(5.0);
            if ui.button("Load Games").clicked() {
                self.load_results(ui.ctx().clone());
            }
        });
    }

    fn central_panel(&mut self, ui: &mut Ui) {
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .columns(Column::auto().resizable(true), 10)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .header(15.0, |mut header| {
                for &header_str in HEADERS.iter() {
                    header.col(|ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.heading(header_str);
                        });
                    });
                }
            })
            .body(|body| {
                let results = self.results.read().unwrap();
                body.rows(25.0, results.len(), |index, mut row| {
                    let result = &results[index];
                    row.col(|ui| {
                        ui.label(&result.title);
                    });
                    row.col(|ui| {
                        ui.label(&result.date);
                    });
                    row.col(|ui| {
                        ui.label(&result.venue_start);
                    });
                    row.col(|ui| {
                        ui.label(&result.venue_end);
                    });
                    row.col(|ui| {
                        ui.label(&result.duration);
                    });
                    row.col(|ui| {
                        ui.label(&result.pre_game_delay);
                    });
                    row.col(|ui| {
                        ui.label(&result.delay_time);
                    });
                    row.col(|ui| {
                        ui.label(&result.start_time);
                    });
                    row.col(|ui| {
                        ui.label(&result.end_time);
                    });
                    row.col(|ui| {
                        ui.label(&result.broadcasts);
                    });
                });
            });
    }
}
