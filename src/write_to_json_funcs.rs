use std::{fs::File, io::{BufReader, Seek, SeekFrom}};

use chrono::Local;

use crate::yt_traits::{SnippetComparable, YtItemWithSnippet};

pub async fn write_better_generic<T>(item: T) -> Result<String, std::io::Error>
where T: YtItemWithSnippet + serde::Serialize + for<'de> serde::Deserialize<'de> {
    let id = item.id();
    let title = item.title();
    let title_abridged = &convert_forbidden_ascii(title.clone());
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{title_abridged} {id}.json"))?;


    let mut old_data: T = if file.metadata()?.len() == 0 {
        T::new_empty(id.clone(), item.snippet())
    } else {
        serde_json::from_reader(BufReader::new(&file))?
    };

    let time = if let Some(current_stats) = item.statistics().first() {
        println!("{title}:\n{:#?}", &current_stats);
        old_data.statistics_mut().push(current_stats.clone());
        T::get_time(current_stats)
    } else {
        &Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    };

    let current_snippet = item.snippet();
    let old_snippet = old_data.snippet();
    let no_changes = current_snippet == old_snippet;

    if !no_changes {
        let changes = compare_snippets_generic(current_snippet, old_snippet, time);
        *old_data.snippet_mut() = item.take_snippet();
        match old_data.changes_mut() {
            Some(vec_changes) => vec_changes.push(changes),
            None => {
                let mut vec_changes = Vec::new();
                vec_changes.push(changes);
                *old_data.changes_mut() = Some(vec_changes);
            }
        }
    }   

    let _ = file.seek(SeekFrom::Start(0));
    let _ = file.set_len(0);
    let _ = serde_json::to_writer_pretty(file, &old_data)?;

    Ok(format!("{} Successful.", title_abridged))
}

fn compare_snippets_generic<S: SnippetComparable>(new: &S, old: &S, time: &String) -> S::ChangesType {
    new.compare(old, time)
}

fn convert_forbidden_ascii(title: String) -> String {
    title.chars().map(|c| match c {
        '<' => '(',
        '>' => ')',
        ':' => ';',
        '"' => '\'',
        '\\' => '[',
        '/' => ']',
        '|' => '_',
        '?' => '!',
        '*' => '^',
        _ => c
    }).collect()
}