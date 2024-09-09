use lazy_static::lazy_static;

lazy_static! {
    pub static ref OUTLETS: Vec<String> = vec![
        "ABC".to_string(),
        "ACC Network".to_string(),
        "BIG10".to_string(),
        "CBS".to_string(),
        "CBSSN".to_string(),
        "ESPN".to_string(),
        "ESPN2".to_string(),
        "ESPNU".to_string(),
        "NBC".to_string(),
        "NFLN".to_string(),
        "PAC12".to_string(),
        "FOX".to_string(),
        "FS1".to_string(),
        "SEC Network".to_string()
    ];
}

pub async fn get_outlets() -> Vec<(u8, String)> {
    OUTLETS
        .clone()
        .into_iter()
        .enumerate()
        .map(|s| (s.0 as u8, s.1))
        .collect()
}
