use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

use regex::Regex;
use reqwest;
use tokio;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "EndPoint")]
    pub end_point: String,
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub clients: i64,
    pub gametype: String,
    pub hostname: String,
    pub mapname: String,
    #[serde(rename = "sv_maxclients")]
    pub sv_maxclients: i64,
    pub enhanced_host_support: bool,
    pub request_steam_ticket: Option<String>,
    pub resources: Vec<String>,
    pub server: String,
    pub vars: Vars,
    pub self_reported_clients: Option<i64>,
    pub players: Vec<Player>,
    #[serde(rename = "ownerID")]
    pub owner_id: i64,
    pub private: bool,
    pub fallback: bool,
    pub connect_end_points: Vec<String>,
    pub upvote_power: i64,
    pub burst_power: i64,
    #[serde(rename = "support_status")]
    pub support_status: Option<String>,
    #[serde(rename = "svMaxclients")]
    pub sv_maxclients2: Option<i64>,
    pub owner_name: String,
    pub owner_profile: String,
    pub owner_avatar: String,
    pub last_seen: String,
    pub icon_version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vars {
    pub activitypub_feed: Option<String>,
    #[serde(rename = "banner_connecting")]
    pub banner_connecting: Option<String>,
    #[serde(rename = "banner_detail")]
    pub banner_detail: Option<String>,
    pub gamename: Option<String>,
    pub locale: Option<String>,
    #[serde(rename = "onesync_enabled")]
    pub onesync_enabled: Option<String>,
    #[serde(rename = "sv_enforceGameBuild")]
    pub sv_enforce_game_build: Option<String>,
    #[serde(rename = "sv_enhancedHostSupport")]
    pub sv_enhanced_host_support: Option<String>,
    #[serde(rename = "sv_lan")]
    pub sv_lan: Option<String>,
    #[serde(rename = "sv_licenseKeyToken")]
    pub sv_license_key_token: Option<String>,
    #[serde(rename = "sv_maxClients")]
    pub sv_max_clients: Option<String>,
    #[serde(rename = "sv_projectDesc")]
    pub sv_project_desc: Option<String>,
    #[serde(rename = "sv_projectName")]
    pub sv_project_name: Option<String>,
    #[serde(rename = "sv_pureLevel")]
    pub sv_pure_level: Option<String>,
    #[serde(rename = "sv_scriptHookAllowed")]
    pub sv_script_hook_allowed: Option<String>,
    pub tags: Option<String>,
    #[serde(rename = "txAdmin-version")]
    pub tx_admin_version: Option<String>,
    #[serde(rename = "🎧Teamspeak")]
    pub teamspeak: Option<String>,
    #[serde(rename = "📢Discord")]
    pub discord: Option<String>,
    pub premium: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub endpoint: String,
    pub id: i64,
    pub identifiers: Vec<String>,
    pub name: String,
    pub ping: i64,
}

#[tokio::main]
async fn main() {
    let api = "https://servers-frontend.fivem.net/api/servers/stream/";
    let api2 = "https://servers-frontend.fivem.net/api/servers/single/";
    let headers = [
        ("Host", "servers-frontend.fivem.net"),
        (
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
        ),
        ("Accept-Language", "en-US,en;q=0.5"),
        ("Connection", "keep-alive"),
        ("Upgrade-Insecure-Requests", "1"),
        ("Sec-Fetch-Dest", "document"),
        ("Sec-Fetch-Mode", "navigate"),
        ("Sec-Fetch-User", "?1"),
        (
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
        ("Sec-Fetch-Site", "cross-site"),
        ("Pragma", "no-cache"),
        ("Cache-Control", "no-cache"),
    ];

    loop {
        let mut data_to_wait = Duration::from_millis(1000);
        let actual_scraped_data = fs::read_to_string("data").unwrap();
        let client = reqwest::Client::new();
        let mut req = client.get(api);
        for header in headers {
            req = req.header(header.0, header.1);
        }
        let req = req.send().await.unwrap();
        let response = req.text().await.unwrap();
        let servers: Vec<String> = Vec::new();
        let re = Regex::new(r"\b[a-z]{2}\d[a-z]\d{2}\b").unwrap();

        // Recherche des correspondances dans le document
        let matches: Vec<_> = re.find_iter(&response).map(|mat| mat.as_str()).collect();
        println!("Scraping...");
        for server in matches {
            let mut should_continue: bool = true;
            let client = reqwest::Client::builder().build().unwrap();
            let mut req = client.get(format!("{api2}{server}"));
            for header in headers {
                req = req.header(header.0, header.1);
            }
            let req = req.send().await.unwrap();
            let response = req.text().await.unwrap();
            let server = serde_json::from_str(&response);
            if server.is_ok() {
                let server: Root = server.unwrap();
                for player in server.data.players {
                    let mut identifiers: String = String::new();
                    for id in player.identifiers {
                        identifiers.push_str(&id);
                        identifiers.push(',');
                    }
                    identifiers.pop();
                    if actual_scraped_data.contains(&player.name) {
                        for line in actual_scraped_data.lines() {
                            if line.contains(&player.name) {
                                if line.contains(&server.data.hostname) {
                                    continue;
                                }
                            }
                        }
                    }

                    let user: String = format!("{}#{},{}", player.name, server.data.hostname, identifiers);
                    //append to the file
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open("data")
                        .unwrap();

                    if let Err(e) = writeln!(file, "{user}") {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            } else {
                println!("Error: {}", server.unwrap_err());
                data_to_wait = Duration::from_millis(10000);
                should_continue = true;
            }
            sleep(data_to_wait);
                
        }
    }
}
