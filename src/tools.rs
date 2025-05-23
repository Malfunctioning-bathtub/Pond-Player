use serde_json;
use std::{collections::HashMap, fs::File};

pub fn song_details_to_file_path(album_artist:String, album:String, track:String, library:HashMap<String, HashMap<String, HashMap<String, String>>>) -> String {
    // return library[&album_artist.to_owned()][&album.to_owned()][&track.to_owned()];
    return "bob".to_string();
}