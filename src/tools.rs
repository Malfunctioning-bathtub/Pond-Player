use std::{collections::HashMap, time::Duration};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub fn first_frame_setup(sink: &rodio::Sink, volume_slider_value: f32) {
    set_volume(sink, volume_slider_value);
}

pub fn song_details_to_file_path(album_artist:&String, album:&String, track:&String, library:&HashMap<String, HashMap<String, HashMap<String, String>>>) -> String{
    return library[&album_artist.to_owned()][&album.to_owned()][&track.to_owned()].clone();
}

pub fn skip(sink: &rodio::Sink, song_length:&Duration) {
    sink.skip_one();
}

pub fn set_volume(sink: &rodio::Sink, volume_slider_value: f32) {
    if volume_slider_value == -1.25{
        sink.set_volume(0.0);
    }
    else {
        sink.set_volume(7.0_f32.powf(volume_slider_value - 1.0));
    }
}