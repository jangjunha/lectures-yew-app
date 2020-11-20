use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lecture {
    pub identifier: String,
    pub title: String,
    pub professor: String,
    pub credit: f64,
    pub times: Vec<LectureTime>,
    pub category: Vec<String>,
    pub metas: Vec<Meta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LectureTime {
    pub weekday: i64,
    pub period_begin: Option<i64>,
    pub period_end: Option<i64>,
    pub room: String,
    pub time_begin: Option<i64>,
    pub time_end: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    pub r#type: String,
}
