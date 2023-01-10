use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub fn get_teams(team_map: Arc<RwLock<BTreeMap<u16, String>>>) {
    let request = ehttp::Request::get("https://statsapi.mlb.com/api/v1/teams?sportId=1");
    ehttp::fetch(request, move |response| {
        if let Ok(response) = response {
            if let Some(text) = response.text() {
                if let Ok(teams) = serde_json::from_str::<Teams>(text) {
                    let mut team_map = team_map.write().unwrap();
                    for team in teams.teams {
                        if team.active.unwrap_or_default() {
                            team_map.insert(team.id, team.name);
                        }
                    }
                }
            }
        }
    });
}

#[derive(Deserialize, Clone)]
pub struct Teams {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Clone)]
pub struct Team {
    pub id: u16,
    pub active: Option<bool>,
    pub name: String,
}
