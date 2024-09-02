use std::io::{self};

use write_to_json_funcs::{write_channels_to_file, write_videos_to_file};
use youtube_channel::ChannelRootComplete;
use youtube_get_funcs::{youtube_get_channels, youtube_get_videos};
use youtube_video::VideoRootComplete;

pub mod youtube_video;
pub mod ryd_struct;
pub mod youtube_channel;
pub mod youtube_get_funcs;
pub mod write_to_json_funcs;

#[tokio::main]
async fn main() {
    let inputs: String = get_ids_from_user();
    let ids : Vec<&str> = inputs.split_ascii_whitespace().collect();
    if ids.len() != 0 {
        youtube_api_access(ids).await;
    }
}

fn get_ids_from_user() -> String {
    let mut buffer: String = String::new();
    println!("Enter IDs: ");
    let stdin: io::Stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            buffer
        },
        Err(_) => {
            String::new()
        },
    }
}

async fn youtube_api_access(input : Vec<&str>){
    let (vids, chans) = parse_ids(input);

    let videos: VideoRootComplete = youtube_get_videos(
        vids, 
        "items(id, snippet(title,publishedAt,description,tags,channelTitle),statistics)", 
        "https://www.googleapis.com/youtube/v3/videos").await;

    let channels: ChannelRootComplete = youtube_get_channels(
        chans,
        "items(id, snippet(title, description, publishedAt, localized), statistics)",
        "https://www.googleapis.com/youtube/v3/channels").await;

    write_videos_to_file(videos);
    write_channels_to_file(channels);
}

fn parse_ids(ids: Vec<&str>) -> (Vec<&str>, Vec<&str>){
    let mut videos : Vec<&str> = vec![];
    let mut channels : Vec<&str> = vec![];
    let mut others : Vec<&str> = vec![];

    for id in ids {
        if id.len() == 11 {
            videos.push(id);
        } else if id.len() == 24 {
            channels.push(id);
        } else {
            others.push(id);
        }
    }

    (videos, channels)
}