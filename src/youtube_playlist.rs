use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistRoot {
    pub next_page_token: Option<String>,
    //pub prev_page_token: Option<String>,
    pub items: Vec<Item>,
    pub page_info: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub content_details: ContentDetails
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentDetails {
    pub video_id: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total_results: i64,
    pub results_per_page: i64,
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

/*
https://www.googleapis.com/youtube/v3/playlistItems?
part=contentDetails
&maxResults=50
&pageToken=EAAaHlBUOkNESWlFREV6T0RBek1FUkdORGcyTVRNMVFUaw
&playlistId=PL3N_0dV_yL3ZoA08G-CzbR-tWMV-I8xQn
&key=[YOUR_API_KEY]
&pageToken=EAAaHlBUOkNCNGlFREU1TVRORE9FRkROVGN3TTBNMk56TQ
*/