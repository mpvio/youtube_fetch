use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    pub items: Vec<PlItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlItem {
    pub id: String,
    pub snippet: PlSnippet,
    pub status: PlStatus,
    pub content_details: PlaylistSize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlSnippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub channel_title: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlStatus {
    pub privacy_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSize {
    pub item_count: i64,
}
/*

playlist's own info:
GET 
https://youtube.googleapis.com/youtube/v3/playlists?
part=status&part=contentDetails&part=snippet
&id=PLtJifsYDxDHEwRfRS_WoaAKueZkWMsr4G
&id=PLtJifsYDxDHHpVFsh8jxBz8ISZBsw37Ra
&id=PL3N_0dV_yL3ZoA08G-CzbR-tWMV-I8xQn
&maxResults=50
&key=[YOUR_API_KEY] 
HTTP/1.1

*/