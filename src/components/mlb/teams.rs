use serde::Deserialize;

pub async fn get_teams() -> Vec<(u16, String)> {
    if let Ok(response) = reqwest::get("https://statsapi.mlb.com/api/v1/teams?sportId=1").await {
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

#[derive(Deserialize, Clone, Default)]
pub struct Teams {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Clone, Default)]
pub struct Team {
    pub id: u16,
    pub active: Option<bool>,
    pub name: String,
}
