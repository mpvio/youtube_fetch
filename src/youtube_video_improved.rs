use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::youtube_video::Snippet;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtVideo{
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
    pub favorite_count: String,
    pub comment_count: String,
    pub like_count: String,
    pub dislike_count: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Changes {
    pub time: String,
    pub published_at: Option<StringDiff>,
    pub title: Option<StringDiff>,
    pub description: Option<StringDiff>,
    pub channel_title: Option<StringDiff>,
    pub tags: Option<TagsDiff>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringDiff {
    pub old: String,
    pub new: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagsDiff {
    pub old: Option<Vec<String>>,
    pub new: Option<Vec<String>>
}