use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseBody {
    pub build_time: String,
}