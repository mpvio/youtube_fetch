use std::{fs::File, io::{BufReader, Seek, SeekFrom}};

use chrono::Local;

use crate::{
    youtube_channel::{Snippet as ChnSnippet}, 
    youtube_video::Snippet, 
    youtube_video_improved::{Changes, StringDiff, TagsDiff, YtVideo},
    yt_channel_improved::{Changes as ChnChanges, LocalizedDiff, YtChannel}
};

pub async fn write_better_channel(channel: YtChannel) -> Result<&'static str, std::io::Error> {
    let id: &String = &channel.id;
    let title: &String = &channel.snippet.title;
    let title_abridged: &String = &title.chars().take(10).collect();
    let mut file = File::options().read(true).write(true).create(true).open(format!("{title_abridged} {id}.json"))?;

    let mut old: YtChannel = if file.metadata()?.len() == 0 {
        YtChannel {
            id: id.clone(),
            snippet: channel.snippet.clone(),
            statistics: Vec::new(),
            changes: None
        }
    } else {
        serde_json::from_reader(BufReader::new(&file))?
    };

    let time = if let Some(current_stats) = channel.statistics.first() {
        // output result
        println!("{id} {title}:\n{:#?}", &current_stats);
        // push current stats to existing vec
        old.statistics.push(current_stats.clone());
        // return time data was taken, else assign current time
        &current_stats.time
    } else {
        &Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    };

    let current_snippet = &channel.snippet;
    let old_snippet = &old.snippet;
    // check if title/ tags/ desc/ etc. changed
    let no_changes = current_snippet == old_snippet;

    if !no_changes {
        let changes = compare_snippets_channel(current_snippet, old_snippet, time);
        old.snippet = channel.snippet;
        match old.changes {
            Some(ref mut vec_changes) => {
                vec_changes.push(changes)
            },
            None => {
                let mut vec_changes = Vec::<ChnChanges>::new();
                vec_changes.push(changes);
                old.changes = Some(vec_changes);
            },
        }
    }

    //overwrite file with updated json/ struct
    let _ = file.seek(SeekFrom::Start(0));
    let _ = file.set_len(0);
    let _ = serde_json::to_writer_pretty(file, &old)?;

    Ok("Channel Successful.")
}

pub async fn write_better_video(video: YtVideo) -> Result<&'static str, std::io::Error> {
    let id: &String = &video.id;
    let title: &String = &video.snippet.title;
    let title_abridged: String = title.chars().take(10).collect();
    let mut file = File::options().read(true).write(true).create(true).open(format!("{title_abridged} {id}.json"))?;
    
    let mut old_data: YtVideo = if file.metadata()?.len() == 0 {
        YtVideo {
            id: id.clone(),
            snippet: video.snippet.clone(),
            statistics: Vec::new(),
            changes: None
        }
    } else {
        serde_json::from_reader(BufReader::new(&file))?
    };
    
    let time = if let Some(current_stats) = video.statistics.first() {
        // output result
        println!("{id} {title}:\n{:#?}", &current_stats);
        // push current stats to existing vec
        old_data.statistics.push(current_stats.clone());
        // return time data was taken, else assign current time
        &current_stats.time
    } else {
        &Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    };

    let current_snippet = &video.snippet;
    let old_snippet = &old_data.snippet;
    // check if title/ tags/ desc/ etc. changed
    let no_changes = current_snippet == old_snippet;
    if !no_changes {
        let changes = compare_snippets(current_snippet, old_snippet, time);
        old_data.snippet = video.snippet;
        match old_data.changes {
            Some(ref mut vec_changes) => {
                vec_changes.push(changes)
            },
            None => {
                let mut vec_changes = Vec::<Changes>::new();
                vec_changes.push(changes);
                old_data.changes = Some(vec_changes);
            },
        }
    }

    //overwrite file with updated json/ struct
    let _ = file.seek(SeekFrom::Start(0));
    let _ = file.set_len(0);
    let _ = serde_json::to_writer_pretty(file, &old_data)?;

    Ok("Video Successful.")
}

fn compare_snippets_channel(new_snippet: &ChnSnippet, old_snippet: &ChnSnippet, time: &String) -> ChnChanges {
    let mut changes: ChnChanges = ChnChanges 
    { 
        time: time.to_string(), 
        published_at: None, 
        title: None, 
        description: None,
        localized: None
    };

    if new_snippet.published_at != old_snippet.published_at {
        changes.published_at = Some(
            StringDiff{
                old: old_snippet.published_at.clone(),
                new: new_snippet.published_at.clone()
            })
    }
    
    if new_snippet.title != old_snippet.title {
        changes.title = Some(
            StringDiff{
                old: old_snippet.title.clone(),
                new: new_snippet.title.clone()
            })
    }

    if new_snippet.description != old_snippet.description {
        changes.description = Some(
            StringDiff{
                old: old_snippet.description.clone(),
                new: new_snippet.description.clone()
            })
    }
    
    if new_snippet.localized != old_snippet.localized {
        let nsl = &new_snippet.localized;
        let osl = &old_snippet.localized;
        let title_change = if nsl.title != osl.title {
            Some(
                StringDiff{
                    old: osl.title.clone(),
                    new: nsl.title.clone()
                }
            )
        } else {
            None
        };
        let desc_change = if nsl.description != osl.description {
            Some(
                StringDiff{
                    old: osl.description.clone(),
                    new: nsl.description.clone()
                }
            )
        } else {
            None
        };
        changes.localized = Some(
            LocalizedDiff {
            title: title_change,
            description: desc_change
        });
    }

    changes
}

fn compare_snippets(new_snippet: &Snippet, old_snippet: &Snippet, time: &String) -> Changes{
    let mut changes: Changes = Changes 
    { 
        time: time.to_string(), 
        published_at: None, 
        title: None, 
        description: None, 
        channel_title: None, 
        tags: None 
    };

    if new_snippet.published_at != old_snippet.published_at {
        changes.published_at = Some(
            StringDiff{
                old: old_snippet.published_at.clone(),
                new: new_snippet.published_at.clone()
            })
    }
    
    if new_snippet.title != old_snippet.title {
        changes.title = Some(
            StringDiff{
                old: old_snippet.title.clone(),
                new: new_snippet.title.clone()
            })
    }

    if new_snippet.description != old_snippet.description {
        changes.description = Some(
            StringDiff{
                old: old_snippet.description.clone(),
                new: new_snippet.description.clone()
            })
    }
    
    if new_snippet.tags != old_snippet.tags {
        changes.tags = Some(
            TagsDiff{
                old: old_snippet.tags.clone(),
                new: new_snippet.tags.clone()
            })
    }

    changes
}