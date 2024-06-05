use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StatusResponse {
    pub version: Version,
    pub players: Players,
    // description: Option<Description>,
    // favicon: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub name: Option<String>,
    pub protocol: i32,
}

#[derive(Deserialize, Debug)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<Player>,
}

#[derive(Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub id: String,
}

// #[derive(Deserialize, Debug)]
// pub struct Description {
//     text: String,
// }