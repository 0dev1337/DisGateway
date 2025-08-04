use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GuildIdOnly {
    pub id: String,
}
