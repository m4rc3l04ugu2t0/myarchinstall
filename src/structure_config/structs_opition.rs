use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Timezone {
    pub region: String,
    pub city: String,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub language: Vec<String>,
    pub keymap: String,
}

#[derive(Deserialize, Debug)]
pub struct System {
    pub hostname: String,
    pub root_password: String,
    pub username: String,
    pub username_password: String,
}

#[derive(Deserialize, Debug)]
pub struct Packages {
    pub essentials: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Zran {
    pub enabled: bool,
    pub size: String,
}

#[derive(Deserialize, Debug)]
pub struct Drives {
    pub graphic: Vec<String>,
}
