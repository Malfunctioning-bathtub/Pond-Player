use std::{collections::HashMap, rc::Rc};
use log::{self, error, trace};

#[derive(Debug)]
pub struct LibraryHandler {
    pub release_artists: HashMap<Rc<str>, Vec<Rc<Song>>>,
    pub release_titles: HashMap<Rc<str>, Vec<Rc<Song>>>,
    pub track_titles: HashMap<Rc<str>, Vec<Rc<Song>>>,
}

#[derive(Debug)]
pub struct Song {
    pub release_artist: Option<Rc<str>>,
    pub release_title: Option<Rc<str>>,
    pub track_title: Option<Rc<str>>,
    pub file_path: Box<str>,
}

impl Song {
    fn new(file_path: Box<str>) -> Self {
        Song{
            release_artist: None,
            release_title: None,
            track_title: None,
            file_path
        }
    }
}

impl LibraryHandler {
    pub fn new(metadata_path: &str) -> Self {
        trace!("Constructing new LibraryHandler from {}", metadata_path);
        let json_collection: Vec<Vec<serde_json::Value>> = serde_json::from_str(&std::fs::read_to_string(metadata_path).unwrap()).unwrap();

        let mut result = Self{
            release_artists: HashMap::new(),
            release_titles: HashMap::new(),
            track_titles: HashMap::new(),
        };

        let mut songs = vec![];

        let mut iter = json_collection.iter();

        iter.next().unwrap().iter().for_each(|file_path|
            songs.push(Rc::new(Song::new({
                if let serde_json::Value::String(file_path) = file_path {
                    file_path.clone().into_boxed_str()
                } else {Self::parse_error("Index 0 of the array isnt a string.")}
            })))
        );

        iter.enumerate().for_each(|(index, outer)| outer.iter().for_each(|value| {
            let arr: &Vec<serde_json::Value>;
            if let serde_json::Value::Array(value) = value {
                arr = value;
            } else {
                Self::parse_error("Contents of indices higher than 0 within a particular field must be a list.")
            }

            let mut value = arr.iter();
            let key: Rc<str>;
            if let Some(serde_json::Value::String(name)) = value.next() {
                key = (*name.clone()).into();
            } else {
                Self::parse_error("Tags must be strings and the list of songs corresponding to each tag must contain song ids.");
            }

            if let Some(serde_json::Value::Array(arr)) = value.next() {
                value = arr.iter();
            } else {
                Self::parse_error("Tags must have a corresponding list of song ids, as opposed to a single song id or any other type.")
            }

            value.for_each(|song_index| {
                if let serde_json::Value::Number(song_index) = song_index {
                    let song_index = song_index.as_u64().unwrap_or_else(|| Self::parse_error("List of songids corresponding to tag strings must be usize, not any integer.")) as usize;
                    result.append_map(index, &key, &mut songs, song_index);
                } else {
                    Self::parse_error("List of songids corresponding to tag strings must be usize, not any value.")
                }
            });
        }));
        for song in songs {
            if let Some(release_artist) = song.release_artist.clone() {
                result.release_artists.get_mut(&release_artist).unwrap().push(Rc::clone(&song));
            }
            if let Some(release_title) = song.release_title.clone() {
                result.release_titles.get_mut(&release_title).unwrap().push(Rc::clone(&song));
            }
            if let Some(track_title) = song.track_title.clone() {
                result.track_titles.get_mut(&track_title).unwrap().push(Rc::clone(&song));
            }
        }
        // println!("{result:?}");
        result
    }

    fn append_map(&mut self, index: usize, key: &Rc<str>, songs: &mut [Rc<Song>], song_index: usize) {
        let hashmap = match index {
            0 => &mut self.release_artists,
            1 => &mut self.release_titles,
            2 => &mut self.track_titles,
            _ => {
                Self::parse_error(&format!("Property index: {} with value {} does not refer to any existing property",  index, key));
            }
        };
        if let Some((k, _)) = hashmap.get_key_value(key) {
            match index {
                0 => Rc::get_mut(&mut songs[song_index]).unwrap().release_artist = Some(Rc::clone(k)),
                1 => Rc::get_mut(&mut songs[song_index]).unwrap().release_title = Some(Rc::clone(k)),
                2 => Rc::get_mut(&mut songs[song_index]).unwrap().track_title = Some(Rc::clone(k)),
                _ => panic!("Mathematically impossible")
            }
        } else {
            let k = Rc::clone(key);
            hashmap.insert(Rc::clone(&k), vec![]);
            match index {
                0 => Rc::get_mut(&mut songs[song_index]).unwrap().release_artist = Some(k),
                1 => Rc::get_mut(&mut songs[song_index]).unwrap().release_title = Some(k),
                2 => Rc::get_mut(&mut songs[song_index]).unwrap().track_title = Some(k),
                _ => panic!("Mathematically impossible")
            }
        }
    }

    fn parse_error(message: &str) -> ! {
        error!("`metadatacollection.json` malformed:\n{message}");
        panic!()
    }
}