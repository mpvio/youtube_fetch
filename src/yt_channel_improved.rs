use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::youtube_channel::Snippet;
use crate::youtube_video_improved::StringDiff;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtChannel{
    pub id: String,
    pub snippet: Snippet,
    pub statistics: Vec<FullStatistics>,
    pub changes: Option<Vec<Changes>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullStatistics {
    pub time: String,
    pub view_count: String,
    pub subscriber_count: String,
    pub hidden_subscriber_count: bool,
    pub video_count: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Changes {
    pub time: String,
    pub title: Option<StringDiff>,
    pub description: Option<StringDiff>,
    pub published_at: Option<StringDiff>,
    pub localized: Option<LocalizedDiff>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedDiff {
    pub title: Option<StringDiff>,
    pub description: Option<StringDiff>,
}