use reqwest::header::{HeaderMap, HeaderValue};

pub fn build_discord_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("*/*"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9"));

    headers.insert("authorization", HeaderValue::from_str(token).unwrap());

    headers.insert("content-length", HeaderValue::from_static("109"));
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    headers.insert("cookie", HeaderValue::from_static("__dcfduid=..."));
    headers.insert("origin", HeaderValue::from_static("https://discord.com"));
    headers.insert("priority", HeaderValue::from_static("u=1, i"));
    headers.insert("referer", HeaderValue::from_static("https://discord.com/channels/@me/1401995652740681809"));

    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\", \"Brave\";v=\"138\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));

    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    headers.insert("sec-gpc", HeaderValue::from_static("1"));

    headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)..."));

    headers.insert("x-debug-options", HeaderValue::from_static("bugReporterEnabled"));
    headers.insert("x-discord-locale", HeaderValue::from_static("en-US"));
    headers.insert("x-discord-timezone", HeaderValue::from_static("Asia/Calcutta"));

    headers.insert("x-super-properties", HeaderValue::from_static("eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiQ2hyb21lIiwiZGV2aWNlIjoiIiwic3lzdGVtX2xvY2FsZSI6ImVuLVVTIiwiaGFzX2NsaWVudF9tb2RzIjpmYWxzZSwiYnJvd3Nlcl91c2VyX2FnZW50IjoiTW96aWxsYS81LjAgKFdpbmRvd3MgTlQgMTAuMDsgV2luNjQ7IHg2NCkgQXBwbGVXZWJLaXQvNTM3LjM2IChLSFRNTCwgbGlrZSBHZWNrbykgQ2hyb21lLzEzOC4wLjAuMCBTYWZhcmkvNTM3LjM2IiwiYnJvd3Nlcl92ZXJzaW9uIjoiMTM4LjAuMC4wIiwib3NfdmVyc2lvbiI6IjEwIiwicmVmZXJyZXIiOiIiLCJyZWZlcnJpbmdfZG9tYWluIjoiIiwicmVmZXJyX2N1cnJlbnQiOiIiLCJyZWZlcnJpbmdfZG9tYWluX2N1cnJlbnQiOiIiLCJyZWxlYXNlX2NoYW5uZWwiOiJzdGFibGUiLCJjbGllbnRfYnVpbGRfbnVtYmVyIjo0MjYwMzAsImNsaWVudF9ldmVudF9zb3VyY2UiOm51bGwsImNsaWVudF9sYXVuY2hfaWQiOiJlY2I4MDZlYy02YmMxLTQ2N2YtODlhYy0zNzY3Y2I0OGNiYzciLCJsYXVuY2hfc2lnbmF0dXJlIjoiMmQ0ZWMyOGQtNDY4YS00NzNiLTg3MDYtYzk5OWQyMWZhMjUxIiwiY2xpZW50X2FwcF9zdGF0ZSI6ImZvY3VzZWQiLCJjbGllbnRfaGVhcnRiZWF0X3Nlc3Npb25faWQiOiJmOWUwNzZkNy00NTM0LTRlMGMtOTBkZS04NTg5MTA2ZGM1M2MifQ=="));

    headers
}
