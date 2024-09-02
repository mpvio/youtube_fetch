use std::{fs::{self, File}, io::{self, BufReader, Seek, SeekFrom}};

use chrono::Local;
use youtube_channel::ChannelRootComplete;
use youtube_video::{StatisticsComplete, VideoRootComplete};

pub mod youtube_video;
pub mod ryd_struct;
pub mod youtube_channel;

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

    write_videos_to_file(videos).await;
    write_channels_to_file(channels).await;
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

async fn youtube_get_channels(ids : Vec<&str>, fields : &str, url: &str) -> ChannelRootComplete {
    let api_key_string = get_api_key();
    let api_key = api_key_string.as_str();
    //let api_key: &str = "AIzaSyCZXDPW8OzP00D_54g4tyvDeJ45RA2idYg";
    let params = [
        ("key", api_key),
        ("part", "snippet, statistics"),
        ("id", &ids.join(",")),
        ("fields", fields)
    ];

    let mut yt_complete_obj = ChannelRootComplete {
        items: vec![]
    };

    if let Ok(get_url) = reqwest::Url::parse_with_params(url, params){
        if let Ok(response) = reqwest::get(get_url).await {
            if response.status() == reqwest::StatusCode::OK {
                if let Ok(result) = response.json::<youtube_channel::ChannelRoot>().await {
                    //println!("{:#?}", result);

                    let mut complete_items: Vec<youtube_channel::ItemComplete> = vec![];
                    let time: String = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

                    for item in result.items {
                        complete_items.push(youtube_channel::ItemComplete {
                            id: item.id,
                            snippet: item.snippet,
                            statistics: item.statistics,
                            time: time.clone()
                        });
                    }

                    yt_complete_obj = ChannelRootComplete {
                        items: complete_items
                    };
                }
            }
        }
    }
    yt_complete_obj
}

async fn youtube_get_videos(ids : Vec<&str>, fields : &str, url: &str) -> VideoRootComplete {
    let api_key_string = get_api_key();
    let api_key = api_key_string.as_str();
    //let api_key: &str = "AIzaSyCZXDPW8OzP00D_54g4tyvDeJ45RA2idYg";
    let params = [
        ("key", api_key),
        ("part", "snippet, statistics"),
        ("id", &ids.join(",")),
        ("fields", fields)
    ];
    let mut yt_complete_obj = VideoRootComplete {
        items: vec![]
    };
    if let Ok(get_url) = reqwest::Url::parse_with_params(url, params) {
        let response = reqwest::get(get_url).await; //?.text().await
        if let Ok(res) = response { 
            if res.status() == reqwest::StatusCode::OK {
                let yt = res.json::<youtube_video::VideoRoot>().await;
                if let Ok(result) = yt {
                    //println!("{:#?}", result);
                    let mut complete_items: Vec<youtube_video::ItemComplete> = vec![];
                    for item in result.items {
                        complete_items.push(transform_result(item).await);
                    }
                    yt_complete_obj = VideoRootComplete {
                        items: complete_items
                    };
                }
            }
        }
    };
    yt_complete_obj
}

async fn transform_result(old_result : youtube_video::Item) -> youtube_video::ItemComplete {
    let time: String = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let dislikes = single_ryd(&old_result.id).await;

    let new_statistics : StatisticsComplete  = youtube_video::StatisticsComplete {
        view_count: old_result.statistics.view_count,
        favorite_count: old_result.statistics.favorite_count,
        comment_count: old_result.statistics.comment_count,
        like_count: old_result.statistics.like_count,
        dislike_count: dislikes,
    };
    
    let new_item : youtube_video::ItemComplete = youtube_video::ItemComplete {
        id: old_result.id,
        snippet: old_result.snippet,
        statistics: new_statistics,
        time,
    };

    new_item

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

async fn write_videos_to_file(video_obj : VideoRootComplete){
    for video in video_obj.items {
        let title = video.snippet.title.clone() + " " + &video.id;
        let file_name = format!("{}.json", title);

        if let Ok(mut file) = File::options()
        .read(true)
        .write(true)
        .open(&file_name){
            let reader = BufReader::new(&file);
            let x: Result<VideoRootComplete, serde_json::Error> = serde_json::from_reader(reader);
            match x {
                Ok(mut video_data) => {
                    video_data.items.push(video.clone());
                    let _ = file.seek(SeekFrom::Start(0));
                    let _ = serde_json::to_writer_pretty(file, &video_data);
                    println!("{title} updated.");
                },
                Err(_) => println!("not a video"),
            }
        } else {
            if let Ok(file) = std::fs::File::create(&file_name){
                let new_json = VideoRootComplete {
                    items: [video].to_vec()
                };
                let _ = serde_json::to_writer_pretty(file, &new_json);
                println!("{title} written.");
            }
        }
    };
}

async fn write_channels_to_file(channel_obj : ChannelRootComplete){
    for channel in channel_obj.items {
        let title = channel.snippet.title.clone() + " " + &channel.id;
        let file_name = format!("{}.json", title);

        if let Ok(mut file) = File::options()
        .read(true)
        .write(true)
        .open(&file_name){
            let reader = BufReader::new(&file);
            let x: Result<ChannelRootComplete, serde_json::Error> = serde_json::from_reader(reader);
            match x {
                Ok(mut video_data) => {
                    video_data.items.push(channel.clone());
                    let _ = file.seek(SeekFrom::Start(0));
                    let _ = serde_json::to_writer_pretty(file, &video_data);
                    println!("{title} updated.");
                },
                Err(_) => println!("not a video"),
            }
        } else {
            if let Ok(file) = std::fs::File::create(&file_name){
                let new_json = ChannelRootComplete {
                    items: [channel].to_vec()
                };
                let _ = serde_json::to_writer_pretty(file, &new_json);
                println!("{title} written.");
            }
        }
    };
}

fn get_api_key() -> String {
    let file_name = "api_key.txt";
    
    // if let Ok(mut file) = File::options()
    // .read(true)
    // .open(&file_name) {
    //     let reader = BufReader::new(&file);
    // }

    let contents = fs::read_to_string(file_name)
        .expect("No key found");

    return contents;
}