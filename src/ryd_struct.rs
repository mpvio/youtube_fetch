use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RydResult {
    pub id: String,
    pub date_created: String,
    pub likes: i64,
    pub dislikes: i64,
    pub rating: f64,
    pub view_count: i64,
    pub deleted: bool,
}
