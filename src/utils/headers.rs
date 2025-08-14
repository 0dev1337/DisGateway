use reqwest::header::{HeaderMap, HeaderValue};

pub fn build_discord_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("*/*"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9"));

    headers.insert("authorization", HeaderValue::from_str(token).unwrap());

    // headers.insert("content-length", HeaderValue::from_static("109"));
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    // headers.insert("cookie", HeaderValue::from_static("__dcfduid=..."));
    headers.insert("origin", HeaderValue::from_static("https://discord.com"));
    headers.insert("priority", HeaderValue::from_static("u=1, i"));
    headers.insert("referer", HeaderValue::from_static("https://discord.com/channels/1118045983931125850/1118045983931125853"));

    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\", \"Brave\";v=\"138\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));

    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    headers.insert("sec-gpc", HeaderValue::from_static("1"));

    headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36"));

    headers.insert("x-debug-options", HeaderValue::from_static("bugReporterEnabled"));
    headers.insert("x-discord-locale", HeaderValue::from_static("en-US"));
    headers.insert("x-discord-timezone", HeaderValue::from_static("Asia/Calcutta"));

    headers.insert("x-super-properties", HeaderValue::from_static("eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiQ2hyb21lIiwiZGV2aWNlIjoiIiwic3lzdGVtX2xvY2FsZSI6ImVuLVVTIiwiaGFzX2NsaWVudF9tb2RzIjpmYWxzZSwiYnJvd3Nlcl91c2VyX2FnZW50IjoiTW96aWxsYS81LjAgKFdpbmRvd3MgTlQgMTAuMDsgV2luNjQ7IHg2NCkgQXBwbGVXZWJLaXQvNTM3LjM2IChLSFRNTCwgbGlrZSBHZWNrbykgQ2hyb21lLzEzOS4wLjAuMCBTYWZhcmkvNTM3LjM2IiwiYnJvd3Nlcl92ZXJzaW9uIjoiMTM5LjAuMC4wIiwib3NfdmVyc2lvbiI6IjEwIiwicmVmZXJyZXIiOiIiLCJyZWZlcnJpbmdfZG9tYWluIjoiIiwicmVmZXJyZXJfY3VycmVudCI6IiIsInJlZmVycmluZ19kb21haW5fY3VycmVudCI6IiIsInJlbGVhc2VfY2hhbm5lbCI6InN0YWJsZSIsImNsaWVudF9idWlsZF9udW1iZXIiOjQzMTM4MiwiY2xpZW50X2V2ZW50X3NvdXJjZSI6bnVsbCwiY2xpZW50X2xhdW5jaF9pZCI6ImQ1YjlmMmIxLTRiYjMtNGFhYi1hMGQ2LWVjZDQxOTI3YjI4OCIsImxhdW5jaF9zaWduYXR1cmUiOiI4MTY2Y2E4OC05MThiLTQ1MGUtODQ3Ni0yZGFlMDBmZDczMDUiLCJjbGllbnRfaGVhcnRiZWF0X3Nlc3Npb25faWQiOiJhNWUwM2Y1My0wM2UzLTQ5YmEtYjkwZC04ODA2Y2M4OTUxZjMiLCJjbGllbnRfYXBwX3N0YXRlIjoidW5mb2N1c2VkIn0="));

    headers
}
