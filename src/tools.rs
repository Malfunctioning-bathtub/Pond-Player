use std::{collections::{HashMap, VecDeque}, time::Duration, io::BufReader, fs::File};
use rodio::{Decoder, Source};

pub fn first_frame_setup(sink: &rodio::Sink, volume_slider_value: f32) {
    set_volume(sink, volume_slider_value);
}

pub fn song_details_to_file_path(album_artist: &String, album: &String, track: &String, library: &HashMap<String, HashMap<String, HashMap<String, String>>>) -> String{
    return library[&album_artist.to_owned()][&album.to_owned()][&track.to_owned()].clone();
}

pub fn skip(sink: &rodio::Sink, song_length: &mut Duration, queue: &mut VecDeque<String>) {
    let temp_song_file = BufReader::new(File::open(queue.front().unwrap()).unwrap());
    let temp_source = Decoder::new(temp_song_file).unwrap();
    *song_length = temp_source.total_duration().unwrap();
    sink.append(temp_source);
    sink.skip_one();
    queue.pop_front();
}

pub fn set_volume(sink: &rodio::Sink, volume_slider_value: f32) {
    if volume_slider_value == -1.25{
        sink.set_volume(0.0);
    }
    else {
        sink.set_volume(7.0_f32.powf(volume_slider_value - 1.0));
    }
}

pub fn append_to_queue(queue: &mut VecDeque<String>, album_artist: &String, album: &String, track: &String, library: &HashMap<String, HashMap<String, HashMap<String, String>>>) {
    let song_path:String = song_details_to_file_path(album_artist, album, track, library);
    queue.push_back(song_path);
}