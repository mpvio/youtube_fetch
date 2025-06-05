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
https://www.googleapis.com/youtube/v3/playlistItems?
part=contentDetails
&maxResults=50
&pageToken=EAAaHlBUOkNESWlFREV6T0RBek1FUkdORGcyTVRNMVFUaw
&playlistId=PL3N_0dV_yL3ZoA08G-CzbR-tWMV-I8xQn
&key=AIzaSyACKPcWPpqthbVfqc7UdoabN20hJRk6IHo
&pageToken=EAAaHlBUOkNCNGlFREU1TVRORE9FRkROVGN3TTBNMk56TQ
*/