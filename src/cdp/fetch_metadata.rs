use chrono::{DateTime, Utc, Local};

pub fn fetch_metadata(name: &String, version: &String) -> super::models::Metadata {
    let response_metadata = super::models::Metadata {
        name: name.clone(),
        version: version.clone(),
        authors: "".to_string(),
        publish_date: fetch_build_date(name, version),
        description: "".to_string(),
    };

    return response_metadata;
}

fn fetch_build_date(name: &String, version: &String) -> String {
    let url = format!("https://docs.rs/crate/{name}/{version}/builds.json");
    let resp = reqwest::blocking::get(url).expect("Failed to send request");
    if resp.status().is_success() {
        let body = resp.text().expect("Failed to retrieve response body");
        let parsed: Vec<super::models::ResponseBody> = serde_json::from_str(&body).expect("Failed to deserialize response body");
        let build_time = parsed.get(0).map(|response| response.build_time.clone()).unwrap_or_default();
        let formatted_date = format_date(build_time);
        return formatted_date.to_string();
    }
    return "".to_string();
}

fn format_date(timestamp: String) -> String {
    let datetime: DateTime<Utc> = DateTime::parse_from_rfc3339(timestamp.as_str())
        .expect("Failed to parse date string")
        .with_timezone(&Utc);
    let local_datetime: DateTime<Local> = datetime.into();
    let formatted_date = local_datetime.format("%m-%d-%Y %H:%M:%S").to_string();
    return formatted_date;
}
