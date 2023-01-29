use serde::Deserialize;

pub async fn get_teams() -> Vec<(u8, String)> {
    if let Ok(response) = reqwest::get("https://statsapi.web.nhl.com/api/v1/teams").await {
        let mut teams = response.json::<Teams>().await.unwrap_or_default();
        teams.teams.sort_by(|t1, t2| t1.name.cmp(&t2.name));
        let mut teams = teams
            .teams
            .into_iter()
            .map(|t| (t.id, t.name))
            .collect::<Vec<_>>();
        teams.insert(0, (0, "All".to_string()));
        teams
    } else {
        vec![]
    }
}

#[derive(Deserialize, Default)]
pub struct Teams {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Default)]
pub struct Team {
    pub id: u8,
    pub name: String,
    pub active: Option<bool>,
}
