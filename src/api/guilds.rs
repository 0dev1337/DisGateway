use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use crate::models::{GuildIdOnly, RateLimitResponse};
use crate::utils::headers::build_discord_headers;

pub fn create_server(count: i64, token : &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let template_code = "2TffvPucqHkN";
    let body = json!({
        "name": format!("GATEWAY #{}", count),
        "icon": null,
        "channels": [],
        "system_channel_id": null,
        "guild_template_code": template_code,
    });

    let response = client
        .post("https://discord.com/api/v9/guilds")
        .headers(build_discord_headers(token))
        .json(&body)
        .send()?;

    if response.status().as_u16() == 429 {
        let body = response.text()?;
        let parsed: RateLimitResponse = serde_json::from_str(&body)?;
        println!("Ratelimited for {:?} seconds.... Sleeping", parsed.retry_after);
        sleep(Duration::from_secs_f64(parsed.retry_after));
        create_server(count,token)?; // retry
    } else {
        let body = response.text()?;
        let parsed: GuildIdOnly = serde_json::from_str(&body)?;
        crate::utils::helpers::save_to_file(&parsed.id);
        println!("Successfully Created Server #{} | ID : {}", count, parsed.id);
    }
    Ok(())
}
