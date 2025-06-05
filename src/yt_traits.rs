use std::fmt::Debug;

use crate::{
    youtube_channel::{self, Snippet as CSnippet},
    youtube_video::{self, Snippet as VSnippet}, 
    youtube_video_improved::{
        Changes as VChanges, 
        FullStatistics, 
        StringDiff, 
        TagsDiff, 
        YtVideo}, 
    yt_channel_improved::{
        Changes as ChnChanges, 
        FullStatistics as ChnStats, 
        LocalizedDiff, 
        YtChannel}, yt_playlist_improved::{FullPlSnippet, PlChanges, PlStats, YtPlaylist}
};

pub enum YtItem {
    Vi(YtVideo),
    Ch(YtChannel),
    Pl(YtPlaylist)
}

pub enum YtSnippet {
    Vi(youtube_video::Snippet),
    Ch(youtube_channel::Snippet)
}

pub trait YtItemWithSnippet {
    type SnippetType: PartialEq + SnippetComparable<ChangesType = Self::ChangesType>;
    type ChangesType;
    type StatsType: Clone + Debug;
    
    fn id(&self) -> &String;
    fn title(&self) -> &String;
    fn snippet(&self) -> &Self::SnippetType;
    fn take_snippet(self) -> Self::SnippetType;
    fn snippet_mut(&mut self) -> &mut Self::SnippetType;
    fn statistics(&self) -> &Vec<Self::StatsType>;
    fn statistics_mut(&mut self) -> &mut Vec<Self::StatsType>;
    fn changes_mut(&mut self) -> &mut Option<Vec<Self::ChangesType>>;
    fn new_empty(id: String, snippet: &Self::SnippetType) -> Self;
    fn get_time(stats: &Self::StatsType) -> &String;
}

impl YtItemWithSnippet for YtPlaylist {
    type SnippetType = FullPlSnippet;
    type ChangesType = PlChanges;
    type StatsType = PlStats;

    fn id(&self) -> &String { &self.id }
    fn title(&self) -> &String { &self.snippet.title }
    fn snippet(&self) -> &FullPlSnippet { &self.snippet }
    fn take_snippet(self) -> FullPlSnippet { self.snippet }
    fn snippet_mut(&mut self) -> &mut FullPlSnippet { &mut self.snippet }
    fn statistics(&self) -> &Vec<PlStats> { &self.statistics }
    fn statistics_mut(&mut self) -> &mut Vec<PlStats> { &mut self.statistics }
    fn changes_mut(&mut self) -> &mut Option<Vec<PlChanges>> { &mut self.changes }
    fn new_empty(id: String, snippet: &Self::SnippetType) -> Self {
        YtPlaylist { 
            id, 
            snippet: snippet.clone(), 
            statistics: Vec::new(),
            changes: None 
        }
    }
    fn get_time(stats: &Self::StatsType) -> &String { &stats.time }
}

impl YtItemWithSnippet for YtVideo {
    type SnippetType = VSnippet;
    type ChangesType = VChanges;
    type StatsType = FullStatistics;
    
    fn id(&self) -> &String { &self.id }
    fn title(&self) -> &String { &self.snippet.title }
    fn snippet(&self) -> &VSnippet { &self.snippet }
    fn take_snippet(self) -> VSnippet { self.snippet }
    fn snippet_mut(&mut self) -> &mut VSnippet { &mut self.snippet }
    fn statistics(&self) -> &Vec<FullStatistics> { &self.statistics }
    fn statistics_mut(&mut self) -> &mut Vec<FullStatistics> { &mut self.statistics }
    fn changes_mut(&mut self) -> &mut Option<Vec<VChanges>> { &mut self.changes }
    fn new_empty(id: String, snippet: &VSnippet) -> Self {
        YtVideo {
            id,
            snippet: snippet.clone(),
            statistics: Vec::new(),
            changes: None,
        }
    }
    fn get_time(stats: &FullStatistics) -> &String { &stats.time }
}

impl YtItemWithSnippet for YtChannel {
    type SnippetType = CSnippet;
    type ChangesType = ChnChanges;
    type StatsType = ChnStats;
    
    fn id(&self) -> &String { &self.id }
    fn title(&self) -> &String { &self.snippet.title }
    fn snippet(&self) -> &CSnippet { &self.snippet }
    fn take_snippet(self) -> CSnippet { self.snippet }
    fn snippet_mut(&mut self) -> &mut CSnippet { &mut self.snippet }
    fn statistics(&self) -> &Vec<ChnStats> { &self.statistics }
    fn statistics_mut(&mut self) -> &mut Vec<ChnStats> { &mut self.statistics }
    fn changes_mut(&mut self) -> &mut Option<Vec<ChnChanges>> { &mut self.changes }
    fn new_empty(id: String, snippet: &CSnippet) -> Self {
        YtChannel {
            id,
            snippet: snippet.clone(),
            statistics: Vec::new(),
            changes: None,
        }
    }
    fn get_time(stats: &ChnStats) -> &String { &stats.time }
}

pub trait SnippetComparable {
    type ChangesType;
    
    fn compare(&self, old: &Self, time: &String) -> Self::ChangesType;
}

impl SnippetComparable for FullPlSnippet {
    type ChangesType = PlChanges;
    
    fn compare(&self, old: &Self, time: &String) -> PlChanges {
        let mut changes: PlChanges = PlChanges { 
            time: time.to_string(), 
            title: None, 
            description: None, 
            channel_title: None, 
            privacy_status: None
        };

        if self.title != old.title {
            changes.title = Some(StringDiff { 
                old: old.title.clone(), 
                new: self.title.clone() 
            });
        }

        if self.description != old.description {
            changes.description = Some(StringDiff { 
                old: old.description.clone(), 
                new: self.description.clone() 
            });
        }

        if self.channel_title != old.channel_title {
            changes.channel_title = Some(StringDiff { 
                old: old.channel_title.clone(), 
                new: self.channel_title.clone() 
            });
        }

        if self.privacy_status != old.privacy_status {
            changes.privacy_status = Some(StringDiff { 
                old: old.privacy_status.clone(), 
                new: self.privacy_status.clone() 
            });
        }

        // if self.item_count != old.item_count {
        //     changes.item_count = Some(I64Diff { 
        //         old: old.item_count.clone(), 
        //         new: self.item_count.clone() 
        //     });
        // }

        changes
    }
}

impl SnippetComparable for VSnippet {
    type ChangesType = VChanges;
    
    fn compare(&self, old: &Self, time: &String) -> VChanges {
        let mut changes = VChanges {
            time: time.to_string(),
            published_at: None,
            title: None,
            description: None,
            channel_title: None,
            tags: None,
        };

        if self.published_at != old.published_at {
            changes.published_at = Some(StringDiff {
                old: old.published_at.clone(),
                new: self.published_at.clone(),
            });
        }
        
        if self.title != old.title {
            changes.title = Some(StringDiff {
                old: old.title.clone(),
                new: self.title.clone(),
            });
        }

        if self.description != old.description {
            changes.description = Some(StringDiff {
                old: old.description.clone(),
                new: self.description.clone(),
            });
        }

        if self.tags != old.tags {
            changes.tags = Some(
                TagsDiff {
                    old: old.tags.clone(),
                    new: self.tags.clone()
                }
            );
        }
        
        changes
    }
}


impl SnippetComparable for CSnippet {
    type ChangesType = ChnChanges;
    
    fn compare(&self, old: &Self, time: &String) -> ChnChanges {
        let mut changes = ChnChanges {
            time: time.to_string(),
            published_at: None,
            title: None,
            description: None,
            localized: None,
        };

        if self.published_at != old.published_at {
            changes.published_at = Some(StringDiff {
                old: old.published_at.clone(),
                new: self.published_at.clone(),
            });
        }
        
        if self.title != old.title {
            changes.title = Some(StringDiff {
                old: old.title.clone(),
                new: self.title.clone(),
            });
        }

        if self.description != old.description {
            changes.description = Some(StringDiff {
                old: old.description.clone(),
                new: self.description.clone(),
            });
        }

        if self.localized != old.localized {
            let nsl = &self.localized;
            let osl = &old.localized;
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
}
