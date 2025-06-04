use std::io::{self};
use std::env;

use youtube_get_funcs::{youtube_get_channels, youtube_get_videos};

use crate::write_to_json_funcs::write_better_generic;
use crate::yt_traits::{YtItem};

pub mod youtube_video;
pub mod ryd_struct;
pub mod youtube_channel;
pub mod youtube_get_funcs;
pub mod write_to_json_funcs;
pub mod youtube_video_improved;
pub mod yt_channel_improved;
pub mod yt_traits;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("args: {:#?}", &args);
    
    let ids: Vec<String> = if args.len() > 1 {
        args.into_iter().skip(1).collect()  // Skip the first arg (program name)
    } else {
        let inputs = get_ids_from_user();
        //println!("inputs: {:#?}", &inputs);
        inputs.split_ascii_whitespace().map(|s| s.to_owned()).collect()
    };
    
    //println!("ids: {:#?}", &ids);
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
            buffer.trim().to_string()
        },
        Err(_) => {
            String::new()
        },
    }
}

async fn youtube_api_access(input : Vec<String>){
    let str_input : Vec<&str> = input.iter().map(|s| &**s).collect();
    let (vids, chans) = parse_ids(str_input);

    let mut items: Vec<YtItem> = Vec::new();

    for video in youtube_get_videos(
        vids, 
        "items(id, snippet(title,publishedAt,description,tags,channelTitle),statistics)", 
        "https://www.googleapis.com/youtube/v3/videos").await {
            items.push(YtItem::Vi(video));
        }

    for channel in youtube_get_channels(
        chans,
        "items(id, snippet(title, description, publishedAt, localized), statistics)",
        "https://www.googleapis.com/youtube/v3/channels").await {
            items.push(YtItem::Ch(channel));
        }
    
    for item in items {
        let outcome = match item {
            YtItem::Vi(yt_video) => write_better_generic(yt_video).await,
            YtItem::Ch(yt_channel) => write_better_generic(yt_channel).await,
        };

        match outcome {
            Ok(_) => {}, //println!("{}", msg)
            Err(err) => println!("{:#?}", err),
        }
    }
}

fn parse_ids(ids: Vec<&str>) -> (Vec<&str>, Vec<&str>){
    let mut videos : Vec<&str> = vec![];
    let mut channels : Vec<&str> = vec![];
    let mut others : Vec<&str> = vec![];

    for id in ids {
        //println!("currently: {:#?}", &id);
        if id.len() == 11 {
            //println!("video: {:#?}", &id);
            videos.push(id);
        } else if id.len() == 24 {
            //println!("channel: {:#?}", &id);
            channels.push(id);
        } else {
            //println!("other: {:#?}", &id);
            others.push(id);
        }
    }

    (videos, channels)
}