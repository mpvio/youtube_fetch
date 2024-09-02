use serde_derive::Deserialize;
use serde_derive::Serialize;

//version from YouTube API
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRoot {
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub snippet: Snippet,
    pub statistics: Statistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub published_at: String,
    pub title: String,
    pub description: String,
    pub channel_title: String,
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub view_count: String,
    pub like_count: String,
    pub favorite_count: String,
    pub comment_count: String,
}

//complete version
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRootComplete {
    pub items: Vec<ItemComplete>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemComplete {
    pub id: String,
    pub snippet: Snippet,
    pub statistics: StatisticsComplete,
    #[serde(rename = "TIME")]
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsComplete {
    pub view_count: String,
    pub favorite_count: String,
    pub comment_count: String,
    pub like_count: String,
    pub dislike_count: String,
}