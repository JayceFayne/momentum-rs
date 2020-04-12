use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Background {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "forDate")]
    pub for_date: String,
    pub title: String,
    pub filename: String,
    pub attribution: String,
    pub source: String,
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
    pub is_favorite: bool,
    #[serde(rename = "templateId")]
    pub template_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Feed {
    pub backgrounds: Vec<Background>,
    pub ts_backgrounds: u64,
}
