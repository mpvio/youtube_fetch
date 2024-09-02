use std::{fs::File, io::{BufReader, Seek, SeekFrom}};

use crate::{youtube_channel::ChannelRootComplete, youtube_video::VideoRootComplete};

pub fn write_videos_to_file(video_obj : VideoRootComplete){
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

pub fn write_channels_to_file(channel_obj : ChannelRootComplete){
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