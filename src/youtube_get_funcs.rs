use std::fs;

use chrono::Local;

use crate::{
    ryd_struct, 
    youtube_channel::{self}, 
    youtube_playlist::PlaylistRoot, 
    youtube_video::{self}, 
    youtube_video_improved::{FullStatistics, YtVideo}, 
    yt_channel_improved::{FullStatistics as ChnStats, YtChannel}
};

pub async fn yt_get_playlist(id: &str) -> Result<(i64, Vec<String>), reqwest::Error> {
    let api_key_string = get_api_key();
    let client = reqwest::Client::new();
    let mut videos = Vec::new();
    let mut total_results = 0;
    let mut next_page_token: Option<String> = None;

    loop {
        let mut request = client.get(
            "https://www.googleapis.com/youtube/v3/playlistItems")
            .query(&[("key", &api_key_string)])
            .query(&[("part", "contentDetails")])
            .query(&[("playlistId", id)])
            .query(&[("maxResults", "50")]);

        if let Some(token) = &next_page_token {
            request = request.query(&[("pageToken", token)]);
        }

        let response = request.send().await?;
        let result: PlaylistRoot = response.json().await?;

        if total_results == 0 {
            total_results = result.page_info.total_results;
        }
        
        videos.extend(result.items.into_iter().map(|item| item.content_details.video_id));

        next_page_token = result.next_page_token;
        if next_page_token.is_none() {
            break;
        }
    }

    Ok((total_results, videos))
}

pub async fn youtube_get_channels(ids : Vec<&str>, fields : &str, url: &str) -> Vec<YtChannel> {
    let api_key_string = get_api_key();
    let api_key = api_key_string.as_str();
    let params = [
        ("key", api_key),
        ("part", "snippet, statistics"),
        ("id", &ids.join(",")),
        ("fields", fields)
    ];

    let mut yt_complete_obj : Vec<YtChannel> = vec![];

    if let Ok(get_url) = reqwest::Url::parse_with_params(url, params){
        if let Ok(response) = reqwest::get(get_url).await {
            if response.status() == reqwest::StatusCode::OK {
                if let Ok(result) = response.json::<youtube_channel::ChannelRoot>().await {
                    //start changes here
                    for item in result.items {
                        let yt_channel = convert_channel(item);
                        yt_complete_obj.push(yt_channel);
                    }
                }
            }
        }
    }
    yt_complete_obj
}

fn convert_channel(channel : youtube_channel::Item) -> YtChannel {
    let time: String = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let full_stats : ChnStats = ChnStats {
        time,
        video_count: channel.statistics.video_count,
        view_count: channel.statistics.view_count,
        subscriber_count: channel.statistics.subscriber_count,
        hidden_subscriber_count: channel.statistics.hidden_subscriber_count
    };

    let mut statistics : Vec<ChnStats> = vec![];
    statistics.push(full_stats);

    return YtChannel {
        id: channel.id,
        snippet: channel.snippet,
        statistics: statistics,
        changes: None
    };
}

pub async fn youtube_get_videos(ids : Vec<&str>, fields : &str, url: &str) -> Vec<YtVideo> {
    let api_key_string = get_api_key();
    let api_key = api_key_string.as_str();
    let params = [
        ("key", api_key),
        ("part", "snippet, statistics"),
        ("id", &ids.join(",")),
        ("fields", fields)
    ];

    let mut complete_items: Vec<YtVideo> = vec![];
    if let Ok(get_url) = reqwest::Url::parse_with_params(url, params) {
        //println!("get_url: {:#?}", &get_url);
        let response = reqwest::get(get_url).await;
        match response {
            Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                let yt = res.json::<youtube_video::VideoRoot>().await;
                match yt {
                    Ok(result) => {
                        //println!("result: {:#?}", &result);
                        for item in result.items {
                            let result = convert_video(item).await;
                            complete_items.push(result);
                        }
                    },
                    Err(err) => {
                        println!("{:#?}", err);
                    },
                }
            } else {
                println!("{:#?}", res.status());
            }
        },
            Err(e) => {
                println!("{:#?}", e);
            },
        }
    };
    complete_items
}

async fn convert_video(old_result : youtube_video::Item) -> YtVideo {
    let time: String = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let dislikes = single_ryd(&old_result.id).await;

    //println!("{old_result:#?}");
    let full_stats : FullStatistics = FullStatistics { 
        time, 
        view_count: old_result.statistics.view_count, 
        favorite_count: old_result.statistics.favorite_count, 
        comment_count: old_result.statistics.comment_count, 
        like_count: old_result.statistics.like_count, 
        dislike_count: dislikes 
    };

    let id: String = old_result.id;
    let snippet: youtube_video::Snippet = old_result.snippet;
    let mut statistics : Vec<FullStatistics> = Vec::new();
    statistics.push(full_stats);

    return YtVideo { 
        id, 
        snippet, 
        statistics, 
        changes: None 
    };

}

async fn single_ryd(video_id : &String) -> String {
    let url = format!("https://returnyoutubedislikeapi.com/votes?videoId={}", video_id);
    let response = reqwest::get(url).await;
    if let Ok(result) = response {
        let res = result.json::<ryd_struct::RydResult>().await;
        if let Ok(dislike) = res {
            return dislike.dislikes.to_string();
        }
    }
    "0".to_string()
}

fn get_api_key() -> String {
    let file_name = "target/api_key.txt";

    let contents = fs::read_to_string(file_name)
        .expect("No key found");

    return contents;
}