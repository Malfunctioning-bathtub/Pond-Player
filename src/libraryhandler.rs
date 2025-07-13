use std::{collections::VecDeque};
use log::{self, debug, error, info, trace};

#[derive(Debug, Clone)]
pub enum EitherTagOrSongList {
    Tag(Tag),
    SongList(VecDeque<u32>)
}

impl EitherTagOrSongList {
    pub fn vecdeque_from_songlist(song_list: EitherTagOrSongList ) -> VecDeque<u32> {
        trace!("Fetching song IDs from song list: {:?}", song_list);
        match song_list {
            Self::SongList(song_ids) => song_ids,
            Self::Tag(tag) => {
                error!("Could not fetch song IDs from Tag: {:#?}", tag);
                panic!("")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    ReleaseArtist(String),
    ReleaseTitle(String),
    TrackTitle(String),
    FilePath(String)
}

impl Tag {
    pub fn to_string(self) -> String {
        match self {
            Self::ReleaseArtist(string) => string,
            Self::ReleaseTitle(string) => string,
            Self::TrackTitle(string) => string,
            Self::FilePath(string) => string
        }
    }
}

pub struct LibraryHandler {
    metadata_collection: MetadataCollection,
    song_table: SongTable
}

pub struct SongTable {
    song_table: VecDeque<VecDeque<VecDeque<u32>>>
}

impl SongTable {
    pub fn new(table_path: String) -> Self {
        info!("Constructing new SongTable from {}", table_path);
        let init_table = serde_json::from_str(&std::fs::read_to_string(table_path).unwrap()).unwrap();
        
        Self {
            song_table:init_table
        }
    }

    pub fn songs_from_song_ids(&self, ids: VecDeque<u32>) -> VecDeque<VecDeque<VecDeque<u32>>> {
        trace!("Fetching songs with ids: {:#?}", ids);
        let mut return_vecdeque: VecDeque<VecDeque<VecDeque<u32>>> = VecDeque::new();

        for id in ids {
            return_vecdeque.push_back(self.song_table[id.try_into().unwrap()].clone());
        }
        return return_vecdeque
    }

    pub fn song_from_song_id(&self, id: u32) -> VecDeque<VecDeque<u32>> {
        trace!("Fetching song with ID: {}", id); 
        return self.song_table[id.try_into().unwrap()].clone();
    }
}

pub struct MetadataCollection {
    metadata_collection: VecDeque<VecDeque<VecDeque<EitherTagOrSongList>>>
}

impl MetadataCollection {
    pub fn new(metadata_path: String) -> Self {
        trace!("Constructing new MetadataCollection from {}", metadata_path);
        let json_collection: VecDeque<VecDeque<VecDeque<serde_json::Value>>> = serde_json::from_str(&std::fs::read_to_string(metadata_path).unwrap()).unwrap();

        let mut init_collection: VecDeque<VecDeque<VecDeque<EitherTagOrSongList>>> = VecDeque::new();

        let mut property_index = 0;
        for property in json_collection {
            let mut temp_property: VecDeque<VecDeque<EitherTagOrSongList>> = VecDeque::new();
            for tag in property {
                let mut property_song_list_pair_vecdeque: VecDeque<EitherTagOrSongList> = VecDeque::new();
                let tag_key = serde_json::from_value(tag[0].clone()).unwrap();
                property_song_list_pair_vecdeque.push_back(
                    EitherTagOrSongList::Tag(
                        match property_index {
                            0 => Tag::ReleaseArtist(tag_key),
                            1 => Tag::ReleaseTitle(tag_key),
                            2 => Tag::TrackTitle(tag_key),
                            3 => Tag::FilePath(tag_key),
                        
                            _ => {
                                error!("Property index: {} does not refer to any existing property", property_index);
                                panic!();
                            }
                        }
                    )
                );
                property_song_list_pair_vecdeque.push_back(EitherTagOrSongList::SongList(serde_json::from_value(tag[1].clone()).unwrap()));
                temp_property.push_back(property_song_list_pair_vecdeque);
            }
            init_collection.push_back(temp_property);
            property_index += 1;
        }

        Self { 
            metadata_collection: init_collection
        }
    }

    pub fn tag_from_property_and_tag_id(&self, property_id: u32, tag_id: u32) -> Tag {
        let enum_tag: EitherTagOrSongList = self.metadata_collection[property_id.try_into().unwrap()][tag_id.try_into().unwrap()][0].clone();
        match enum_tag {
            EitherTagOrSongList::Tag(tag) => {
                return tag;
            }
            EitherTagOrSongList::SongList(_songlist) => {
                error!("Cannot fetch Tag from SongList (metadata collection likely improperly formatted)");
                panic!()
            }
        }
    }

    pub fn song_ids_from_tag(&self, tag: Tag) -> VecDeque<u32> {
        let i: usize;
        match tag {
            Tag::ReleaseArtist(_) => i = 0,
            Tag::ReleaseTitle(_) => i = 1,
            Tag::TrackTitle(_) => i = 2,
            Tag::FilePath(_) => i = 3
        } 
        return MetadataCollection::song_ids_from_tag_and_property_vecdeque(tag, &self.metadata_collection[i]);
    }

    pub fn song_ids_from_tag_and_property_vecdeque(req_tag: Tag, property_vecdeque: &VecDeque<VecDeque<EitherTagOrSongList>>) -> VecDeque<u32> {
        trace!("Fetching Song IDs with Tag: {:?} in property: {:?}", req_tag, property_vecdeque);
        for tag in property_vecdeque {
            match &tag[0] {
                EitherTagOrSongList::Tag(generic_tag) => {
                    if generic_tag == &req_tag {
                        return EitherTagOrSongList::vecdeque_from_songlist(tag[1].clone());
                    }
                }
                EitherTagOrSongList::SongList(song_list) => {
                    error!("Cannot not fetch song IDs from a taglist: {:#?}", song_list);
                    panic!();
                }
            }
        }
        return VecDeque::new();
    }
}

impl LibraryHandler {
    pub fn new() -> Self {
        Self { 
            metadata_collection: MetadataCollection::new("./assets/metadatacollection.json".to_string()),
            song_table: SongTable::new("./assets/songtable.json".to_string())
        }
    }

    pub fn metadata_list_from_song_id(&self, song_id: u32) -> VecDeque<VecDeque<Tag>> {
        let property_list: VecDeque<VecDeque<Tag>> = VecDeque::new();
        let property_index: u32  = 0;
        for property in self.song_table.song_from_song_id(song_id) {
            let mut tag_list: VecDeque<Tag> = VecDeque::new();
            for tag in property {
                tag_list.push_back(self.metadata_collection.tag_from_property_and_tag_id(property_index, tag));
            }
        }
        return property_list;
    }

    pub fn metadata_tags_from_song_and_property_id(&self, song_id: u32, property_id: u32) -> Option<VecDeque<Tag>> {
        let mut tag_list: VecDeque<Tag> = VecDeque::new();
        for tag_id in self.song_table.song_from_song_id(song_id)[property_id.try_into().unwrap()].clone() {
            tag_list.push_back(self.metadata_collection.tag_from_property_and_tag_id(property_id, tag_id));
        }

        return Some(tag_list);
    }

    pub fn filepath_from_song_id(&self, song_id: u32) -> Option<String> {
        let raw_tag: Tag = self.metadata_tags_from_song_and_property_id(song_id, 3).unwrap()[0.try_into().unwrap()].clone();
        match raw_tag {
            Tag::FilePath(i) => {
                return Some(i);
            }
            _ => {
                return None;
            }
        }
    }

    pub fn song_ids_from_tag(&self, tag: Tag) -> VecDeque<u32> {
        info!("Fetching Song IDs with Tag: {:?}", tag);
        return self.metadata_collection.song_ids_from_tag(tag);
    }

    pub fn tag_from_property_and_tag_id(&self, property_id: u32, tag_id: u32) -> Tag {
        return self.metadata_collection.tag_from_property_and_tag_id(property_id, tag_id)
    }
}
