use std::io::{self};
use std::env;

use write_to_json_funcs::{write_better_channel, write_better_video};
use youtube_get_funcs::{youtube_get_channels, youtube_get_videos};
use youtube_video_improved::YtVideo;
use yt_channel_improved::YtChannel;

pub mod youtube_video;
pub mod ryd_struct;
pub mod youtube_channel;
pub mod youtube_get_funcs;
pub mod write_to_json_funcs;
pub mod youtube_video_improved;
pub mod yt_channel_improved;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    let ids: Vec<String> = if args.len() > 1 {
        args.into_iter().skip(1).collect()  // Skip the first arg (program name)
    } else {
        let inputs = get_ids_from_user();
        inputs.split_ascii_whitespace().map(|s| s.to_owned()).collect()
    };
    
    if !ids.is_empty() {
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

async fn youtube_api_access(input : Vec<String>){
    let str_input : Vec<&str> = input.iter().map(|s| &**s).collect();
    let (vids, chans) = parse_ids(str_input);

    let videos: Vec<YtVideo> = youtube_get_videos(
        vids, 
        "items(id, snippet(title,publishedAt,description,tags,channelTitle),statistics)", 
        "https://www.googleapis.com/youtube/v3/videos").await;

    let channels: Vec<YtChannel> = youtube_get_channels(
        chans,
        "items(id, snippet(title, description, publishedAt, localized), statistics)",
        "https://www.googleapis.com/youtube/v3/channels").await;

    for video in videos {
        let outcome = write_better_video(video).await;
        match outcome {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("{:#?}", err),
        }
    }

    for channel in channels {
        let outcome = write_better_channel(channel).await;
        match outcome {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("{:#?}", err),
        }
    }
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