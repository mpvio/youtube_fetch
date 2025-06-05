use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::youtube_video_improved::StringDiff;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtPlaylist {
    pub id: String,
    pub snippet: FullPlSnippet,
    pub statistics: Vec<PlStats>,
    pub changes: Option<Vec<PlChanges>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullPlSnippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub channel_title: String,
    pub privacy_status: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlStats {
    pub time: String,
    pub item_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlChanges {
    pub time: String,
    pub title: Option<StringDiff>,
    pub description: Option<StringDiff>,
    pub channel_title: Option<StringDiff>,
    pub privacy_status: Option<StringDiff>
}